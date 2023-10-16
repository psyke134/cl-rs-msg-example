#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(improper_ctypes)]
#[allow(unused_variables)]

// use std::mem;
use libc;
use std::io;

use std::thread;

mod compCfg;
pub use compCfg::*;

mod saMsgFns;
use saMsgFns::*;

mod clLogUtils;
use clLogUtils::*;

mod clBindings;
use clBindings::clCommon::*;
use clBindings::clLogApi::*;
use clBindings::clCpmApi::*;
use clBindings::saAmf::*;

static mut mypid: pid_t = 0;
static mut amfHandle: SaAmfHandleT = 0;
static mut unblockNow: ClBoolT = 0;

fn errorExit()
{
    // clprintf (CL_LOG_SEV_ERROR, "Component [%.*s] : PID [%d]. Initialization error [0x%x]\n",
    //           appName.length, appName.value, mypid, rc);
    std::process::exit(-1);
}

fn string_csi_flags(s: u32) -> &'static str {
    match s {
        S if S & SA_AMF_CSI_ADD_ONE != 0 => "Add One",
        S if S & SA_AMF_CSI_TARGET_ONE != 0 => "Target One",
        S if S & SA_AMF_CSI_TARGET_ALL != 0 => "Target All",
        _ => "Unknown",
    }
}

fn string_ha_state(s: u32) -> &'static str {
    match s {
        SaAmfHAStateT_SA_AMF_HA_ACTIVE => "Active",
        SaAmfHAStateT_SA_AMF_HA_STANDBY => "Standby",
        SaAmfHAStateT_SA_AMF_HA_QUIESCED => "Quiesced",
        SaAmfHAStateT_SA_AMF_HA_QUIESCING => "Quiescing",
        _ => "Unknown",
    }
}

static mut standby: bool = false;
static mut myName: String = String::new();

fn senderLoop() {
    let mut count: i32 = 0;

    unsafe {
        while standby {
            count += 1;
            let message = format!("Msg {} from {}", count, myName);
            clprintf(format!("csa104: Sending Message: {}", &message));
            msgSend::<str>(ACTIVE_COMP_QUEUE, message.clone().into_boxed_str(), (message.len() + 1) as i32);
            thread::sleep(std::time::Duration::from_millis(2000));
        }
    }
}

fn main() {
    unsafe {
        // Update EO name since we couldn't initialize it directly
        clEoConfig.EOname = to32ByteArray(COMP_EO_NAME);
    }

    let mut appname: SaNameT = SaNameT {
        length: 0,
        value: [0; 256]
    };
    let mut callbacks: SaAmfCallbacksT = SaAmfCallbacksT {
        saAmfHealthcheckCallback: None,
        saAmfComponentTerminateCallback: Some(clCompAppTerminate),
        saAmfCSISetCallback: Some(clCompAppAMFCSISet),
        saAmfCSIRemoveCallback: Some(clCompAppAMFCSIRemove),
        saAmfProtectionGroupTrackCallback: None,
        saAmfProxiedComponentInstantiateCallback: None,
        saAmfProxiedComponentCleanupCallback: None
    };
    let mut version: SaVersionT = SaVersionT {
        releaseCode: 'B' as u8,
        majorVersion: 1,
        minorVersion: 1
    };
    let mut iocPort: ClIocPortT = 0;
    let mut rc: SaAisErrorT = SaAisErrorT_SA_AIS_OK;

    let mut dispatch_fd: SaSelectionObjectT = 0;
    let mut read_fds: libc::fd_set = unsafe { std::mem::zeroed() };

    unsafe {
        mypid = getpid();

        rc = saAmfInitialize(&mut amfHandle, &mut callbacks, &mut version);
        
        if rc != SaAisErrorT_SA_AIS_OK {
            errorExit();
        }

        //  FD_ZERO(&read_fds);
        libc::FD_ZERO(&mut read_fds);

        rc = saAmfSelectionObjectGet(amfHandle, &mut dispatch_fd);
        if rc != SaAisErrorT_SA_AIS_OK {
            errorExit();
        }

        //  FD_SET(dispatch_fd, &read_fds);
        libc::FD_SET(dispatch_fd as i32, &mut read_fds);

        rc = saAmfComponentNameGet(amfHandle, &mut appname);
        if rc != SaAisErrorT_SA_AIS_OK {
            errorExit();
        }

        rc = saAmfComponentRegister(amfHandle, &mut appname, std::ptr::null_mut());
        if rc != SaAisErrorT_SA_AIS_OK {
            errorExit();
        }

        msgInitialize();

        clEoMyEoIocPortGet(&mut iocPort);
        
        let appname_slice = std::slice::from_raw_parts(appname.value.as_ptr(), appname.length as usize);
        let appname_str = String::from_utf8_lossy(appname_slice);
        myName = appname_str.to_string();
        clprintf (format!("Component [{}] : PID [{}]. Initializing\0", appname_str, mypid));
        clprintf (format!("IOC Address             : 0x{:X}\0", clIocLocalAddressGet()));
        clprintf (format!("IOC Port                : 0x{:X}\0", iocPort));

        while unblockNow == 0 {
            let rs: i32 = libc::select((dispatch_fd + 1) as i32, &mut read_fds, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
            if rs < 0 {
                let errno = io::Error::last_os_error().raw_os_error().unwrap();
                if errno == libc::EINTR {
                    continue;
                }
                // clprintf (CL_LOG_SEV_ERROR, "Error in select()");
                clprintf (format!("Error in select\0"));
                break;
            }
            saAmfDispatch(amfHandle, SaDispatchFlagsT_SA_DISPATCH_ALL);
        }

        rc = saAmfFinalize(amfHandle);
        if rc != SaAisErrorT_SA_AIS_OK {
            // clprintf (CL_LOG_SEV_ERROR, "AMF finalization error[0x%X]", rc);
            clprintf (format!("AMF finalization error[0x{:X}]\0", rc));
        }

        // clprintf (CL_LOG_SEV_INFO, "AMF Finalized");
        clprintf (format!("AMF Finalized"));
    }
}

unsafe extern "C" fn clCompAppTerminate(invocation: SaInvocationT, compName: *const SaNameT)
{
    let mut rc: SaAisErrorT = SaAisErrorT_SA_AIS_OK;

    // clprintf (CL_LOG_SEV_INFO, "Component [%.*s] : PID [%d]. Terminating\n",
    //           compName->length, compName->value, mypid);
    let compName_slice = unsafe { std::slice::from_raw_parts((*compName).value.as_ptr(), (*compName).length as usize) };
    let compName_str = String::from_utf8_lossy(compName_slice);
    clprintf (format!("Component [{}] : PID [{}]. Terminating\0", compName_str, mypid));

    rc = saAmfComponentUnregister(amfHandle, compName, std::ptr::null_mut());
    if rc != SaAisErrorT_SA_AIS_OK {
        // clprintf (CL_LOG_SEV_ERROR, "Component [%.*s] : PID [%d]. Termination error [0x%x]\n",
        //       compName->length, compName->value, mypid, rc);
        clprintf (format!("Component [{}] : PID [{}]. Termination error[0x{:X}]\0", compName_str, mypid, rc));
        return;
    }

    saAmfResponse(amfHandle, invocation, SaAisErrorT_SA_AIS_OK);

    // clprintf (CL_LOG_SEV_INFO, "Component [%.*s] : PID [%d]. Terminated\n",
    //           compName->length, compName->value, mypid);
    clprintf (format!("Component [{}] : PID [{}]. Terminated\0", compName_str, mypid));

    unblockNow = 1;
}

unsafe extern "C" fn clCompAppAMFCSISet(invocation: SaInvocationT, compName: *const SaNameT, haState: SaAmfHAStateT, csiDescriptor: SaAmfCSIDescriptorT)
{
    // clprintf (CL_LOG_SEV_INFO, "Component [%.*s] : PID [%d]. CSI Set Received\n", 
    //           compName->length, compName->value, mypid);
    let compName_slice = unsafe { std::slice::from_raw_parts((*compName).value.as_ptr(), (*compName).length as usize) };
    let compName_str = String::from_utf8_lossy(compName_slice);
    clprintf (format!("Component [{}] : PID [{}]. CSI Set Received\0", compName_str, mypid));

    clCompAppAMFPrintCSI(csiDescriptor, haState);

    match haState {
        SaAmfHAStateT_SA_AMF_HA_ACTIVE => {
            msgOpen(ACTIVE_COMP_QUEUE, QUEUE_LENGTH);
            thread::spawn(
                || {
                    msgReceiverLoop();
                }
            );
            saAmfResponse(amfHandle, invocation, SaAisErrorT_SA_AIS_OK);
        }
        SaAmfHAStateT_SA_AMF_HA_STANDBY => {
            standby = true;
            thread::spawn(
                || {
                    senderLoop();
                }
            );
            saAmfResponse(amfHandle, invocation, SaAisErrorT_SA_AIS_OK);
        }
        SaAmfHAStateT_SA_AMF_HA_QUIESCED => {
            saAmfResponse(amfHandle, invocation, SaAisErrorT_SA_AIS_OK);
        }
        SaAmfHAStateT_SA_AMF_HA_QUIESCING => {
            saAmfCSIQuiescingComplete(amfHandle, invocation, SaAisErrorT_SA_AIS_OK);
        }
        _ => {
            assert!(false, "");
        }
    }
}

unsafe extern "C" fn clCompAppAMFCSIRemove(invocation: SaInvocationT, compName: *const SaNameT, csiName: *const SaNameT, csiFlags: SaAmfCSIFlagsT)
{
    // clprintf (CL_LOG_SEV_INFO, "Component [%.*s] : PID [%d]. CSI Remove Received\n", 
    //           compName->length, compName->value, mypid);
    let compName_slice = unsafe { std::slice::from_raw_parts((*compName).value.as_ptr(), (*compName).length as usize) };
    let compName_str = String::from_utf8_lossy(compName_slice);
    clprintf (format!("Component [{}] : PID [{}]. CSI Remove Received\0", compName_str, mypid));

    // clprintf (CL_LOG_SEV_INFO, "CSI                     : %.*s\n", csiName->length, csiName->value);
    let csiName_slice = unsafe { std::slice::from_raw_parts((*csiName).value.as_ptr(), (*csiName).length as usize) };
    let csiName_str = String::from_utf8_lossy(csiName_slice);
    clprintf (format!("CSI Name : {}\0", csiName_str));
    // clprintf (CL_LOG_SEV_INFO, "CSI Flags               : 0x%d\n", csiFlags);
    clprintf (format!("CSI Flags : {}\0", string_csi_flags(csiFlags)));

    saAmfResponse(amfHandle, invocation, SaAisErrorT_SA_AIS_OK);
}

unsafe extern "C" fn clCompAppAMFPrintCSI (csiDescriptor: SaAmfCSIDescriptorT , haState: SaAmfHAStateT)
{
    //clprintf (CL_LOG_SEV_INFO, "CSI Flags : [%s]", STRING_CSI_FLAGS(csiDescriptor.csiFlags));
    clprintf (format!("CSI Flags : {}\0", string_csi_flags(csiDescriptor.csiFlags)));

    if (SA_AMF_CSI_TARGET_ALL != csiDescriptor.csiFlags)
    {
        let csiName_slice = unsafe { std::slice::from_raw_parts((csiDescriptor.csiName).value.as_ptr(), (csiDescriptor.csiName).length as usize) };
        let csiName_str = String::from_utf8_lossy(csiName_slice);
        clprintf (format!("CSI Name : {}\0", csiName_str));
    }

    if (SA_AMF_CSI_ADD_ONE == csiDescriptor.csiFlags)
    {
        //clprintf (CL_LOG_SEV_INFO, "Name value pairs :");
        if(csiDescriptor.csiAttr.number != 0)
        {
           clprintf (format!("Name value pairs:\0"));
           let csi_attributes = std::slice::from_raw_parts_mut(csiDescriptor.csiAttr.attr, csiDescriptor.csiAttr.number as usize);
           for i in 0..csiDescriptor.csiAttr.number as usize {
               let attr_name = csi_attributes[i].attrName;
               clprintf(format!("Name: [{}]\0", *attr_name));
               let attr_value = csi_attributes[i].attrValue;
               clprintf(format!("Value : [{}]\0", *attr_value));
           }
        }
    }
    
    //clprintf (CL_LOG_SEV_INFO, "HA state : [%s]", STRING_HA_STATE(haState));
    clprintf (format!("HA state number : [{}]\0", haState as usize));
    clprintf (format!("HA state : [{}]\0", string_ha_state(haState)));

    if (SaAmfHAStateT_SA_AMF_HA_ACTIVE == haState)
    {
        //clprintf (CL_LOG_SEV_INFO, "Active Descriptor :");
        clprintf (format!("Active Descriptor :\0"));
        //clprintf (CL_LOG_SEV_INFO, "Transition Descriptor : [%d]", csiDescriptor.csiStateDescriptor.activeDescriptor.transitionDescriptor);
        //clprintf (format!("Transition Descriptor : [{}]\0", csiDescriptor.csiStateDescriptor.activeDescriptor.transitionDescriptor));
        clprintf (format!("Transition Descriptor : [{}]\0", csiDescriptor.csiStateDescriptor.activeDescriptor.as_ref().transitionDescriptor));
        //clprintf (CL_LOG_SEV_INFO, "Active Component : [%s]", csiDescriptor.csiStateDescriptor.activeDescriptor.activeCompName.value);
        //let activeCompName_slice = unsafe { std::slice::from_raw_parts((csiDescriptor.csiStateDescriptor.activeDescriptor.activeCompName).value.as_ptr(), (csiDescriptor.csiStateDescriptor.activeDescriptor.activeCompName).length as usize) };
        let activeCompName_slice = unsafe { std::slice::from_raw_parts((csiDescriptor.csiStateDescriptor.activeDescriptor.as_ref().activeCompName).value.as_ptr(), (csiDescriptor.csiStateDescriptor.activeDescriptor.as_ref().activeCompName).length as usize) };
        let activeCompName_str = String::from_utf8_lossy(activeCompName_slice);
        clprintf (format!("Active Component : [{}]\0", activeCompName_str));
        //clprintf (format!("Active Component : [{}]\0", csiDescriptor.csiStateDescriptor.activeDescriptor.activeCompName.value));
    }
    else if (SaAmfHAStateT_SA_AMF_HA_STANDBY == haState)
    {
        //clprintf (CL_LOG_SEV_INFO, "Standby Descriptor :");
        clprintf (format!("Standby Descriptor :\0"));
        //clprintf (CL_LOG_SEV_INFO, "Standby Rank : [%d]", csiDescriptor.csiStateDescriptor.standbyDescriptor.standbyRank);
        //clprintf (format!("Standby Rank : [{}]\0", csiDescriptor.csiStateDescriptor.standbyDescriptor.standbyRank));
        clprintf (format!("Standby Rank : [{}]\0", csiDescriptor.csiStateDescriptor.standbyDescriptor.as_ref().standbyRank));
        //clprintf (CL_LOG_SEV_INFO, "Standby Component : [%s]", csiDescriptor.csiStateDescriptor.standbyDescriptor.activeCompName.value);
        //let activeCompName_slice = unsafe { std::slice::from_raw_parts((csiDescriptor.csiStateDescriptor.standbyDescriptor.activeCompName).value.as_ptr(), (csiDescriptor.csiStateDescriptor.standbyDescriptor.activeCompName).length as usize) };
        let activeCompName_slice = unsafe { std::slice::from_raw_parts((csiDescriptor.csiStateDescriptor.standbyDescriptor.as_ref().activeCompName).value.as_ptr(), (csiDescriptor.csiStateDescriptor.standbyDescriptor.as_ref().activeCompName).length as usize) };
        let activeCompName_str = String::from_utf8_lossy(activeCompName_slice);
        clprintf (format!("Active Component : [{}]\0", activeCompName_str));
    }
}
