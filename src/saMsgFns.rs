/*
 *  This is based on csa104Comp/msgFns.c
 */

use crate::clBindings::saMsg::*;
use crate::clBindings::saAis::*;
use crate::clLogUtils::*;

pub const ACTIVE_COMP_QUEUE: &str = "csa104msgqueue";
pub const QUEUE_LENGTH: i32 = 2048;

pub static mut msgLibraryHandle: SaMsgHandleT = 0;
pub static mut msgQueueHandle: SaMsgQueueHandleT = 0;

pub fn msgInitialize() {
    let mut rc: SaAisErrorT = SaAisErrorT_SA_AIS_OK;
    let MsgCallbacks: SaMsgCallbacksT = SaMsgCallbacksT {
        saMsgQueueOpenCallback: None,
        saMsgQueueGroupTrackCallback: None,
        saMsgMessageDeliveredCallback: None,
        saMsgMessageReceivedCallback: None
    };
    let mut Version: SaVersionT = SaVersionT {
        releaseCode: 'B' as u8,
        majorVersion: 1,
        minorVersion: 1
    };

    rc = unsafe {
        saMsgInitialize(&mut msgLibraryHandle, &MsgCallbacks, &mut Version)
    };

    if rc != SaAisErrorT_SA_AIS_OK {
        clprintf(format!("Init failed [0x{:X}]", rc));
        panic!();
    }
}

pub fn msgOpen(queuename: &str, bytesPerPriority: i32) -> SaAisErrorT {
    let mut rc: SaAisErrorT = SaAisErrorT_SA_AIS_OK;
    let saQueueName: SaNameT = strToSaName(queuename);

    let mut CreationAttributes: SaMsgQueueCreationAttributesT = SaMsgQueueCreationAttributesT {
        creationFlags: 0,
        size: [0; 4],
        retentionTime: 0
    };
    for i in 0..SA_MSG_MESSAGE_LOWEST_PRIORITY {
        CreationAttributes.size[i as usize] = bytesPerPriority as SaSizeT;
    }

    let OpenFlags: SaMsgQueueOpenFlagsT = SA_MSG_QUEUE_CREATE;

    rc = unsafe {
        saMsgQueueOpen(msgLibraryHandle, &saQueueName, &CreationAttributes, OpenFlags, SA_TIME_MAX as SaTimeT, &mut msgQueueHandle)
    };

    if rc != SaAisErrorT_SA_AIS_OK {
        clprintf(format!("Msg QueueOpen failed [0x{:X}]\n\r", rc));
    }

    return rc;
}

pub fn msgSend<T: ?Sized>(queuename: &str, buffer: Box<T>, length: i32) -> SaAisErrorT {
    let mut rc: SaAisErrorT = SaAisErrorT_SA_AIS_OK;
    let saQueueName: SaNameT = strToSaName(queuename);
    let message: SaMsgMessageT = SaMsgMessageT {
        type_: 0,
        version: SaVersionT {
            releaseCode: 0,
            majorVersion: 0,
            minorVersion: 0
        },
        senderName: 0x0 as *mut SaNameT, // NULL
        size: length as SaSizeT,
        data: Box::into_raw(buffer) as *mut std::os::raw::c_void,
        priority: SA_MSG_MESSAGE_HIGHEST_PRIORITY as SaUint8T
    };

    rc = unsafe {
        saMsgMessageSend(msgLibraryHandle, &saQueueName, &message, SA_TIME_MAX as SaTimeT)
    };

    if rc != SaAisErrorT_SA_AIS_OK {
        clprintf(format!("Msg saMsgMessageSend to queue [{}] failed [0x{:X}]", String::from_utf8_lossy(&saQueueName.value), rc));
    }

    return rc;
}

pub fn msgReceiverLoop() {
    let mut rc: SaAisErrorT = SaAisErrorT_SA_AIS_OK;
    let mut senderName: SaNameT = SaNameT {
        length: 0,
        value: [0; 256]
    };

    const DAT_SIZE: usize = 1024;
    let mut data: [u8; DAT_SIZE] = [0; DAT_SIZE];
    let mut senderId: SaMsgSenderIdT = 0;
    let mut sendTime: SaTimeT = 0;

    loop {
        let mut message: SaMsgMessageT = SaMsgMessageT {
            type_: 0,
            version: SaVersionT {
                releaseCode: 0,
                majorVersion: 0,
                minorVersion: 0
            },
            senderName: &mut senderName,
            size: DAT_SIZE as SaSizeT,
            data: data.as_mut_ptr() as *mut std::os::raw::c_void,
            priority: SA_MSG_MESSAGE_HIGHEST_PRIORITY as SaUint8T
        };

        unsafe {
            rc = saMsgMessageGet(msgQueueHandle, &mut message, &mut sendTime, &mut senderId, SA_TIME_MAX as SaTimeT);

            if rc != SaAisErrorT_SA_AIS_OK {
                clprintf(format!("Msg saMsgMessageGet failed [0x{:X}]\n\r", rc));
                break;
            }

            if (*message.senderName).length > 0 {
                clprintf(format!("Sender Name   : {}\n", String::from_utf8_lossy(&(*message.senderName).value)));
            }
            let len = libc::strlen(message.data as *const libc::c_char);
            let slice = std::slice::from_raw_parts(message.data as *const u8, len);
            let string_data = String::from_utf8_lossy(slice);
            clprintf(format!("Received Message  : {}\n", string_data));
        }
    }

    rc = unsafe {saMsgQueueClose(msgQueueHandle) };
    if rc != SaAisErrorT_SA_AIS_OK {
        clprintf(format!("Msg Queue Close failed [0x{:X}]\n\r", rc));
    }
}

fn strToSaName(name: &str) -> SaNameT {
    let nameSlice = name.as_bytes();
    let mut buff: [u8; 256] = [0; 256];
    buff[..nameSlice.len()].copy_from_slice(nameSlice);
    let rs: SaNameT = SaNameT {
        length: name.len() as SaUint16T,
        value: buff
    };

    return rs;
}
