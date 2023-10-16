pub use crate::clBindings::clCommon::*;
pub use crate::clBindings::clOsalApi::*;
pub use crate::clBindings::clIocApi::*;
/* automatically generated by rust-bindgen 0.66.1 */

pub const CL_EO_MAX_NAME_LEN: u32 = 32;
pub const CL_EO_DEFAULT_THREADS: u32 = 1;
pub const CL_EO_DEFAULT_NAME: &[u8; 15] = b"3RD_PARTY_COMP\0";
#[doc = " The type of the EOId, assigned to an EO as part of registration\n to the Component Manager."]
pub type ClEoIdT = ClUint64T;
#[doc = " This indicates the initial state of the EO."]
pub const ClEoStateT_CL_EO_STATE_INIT: ClEoStateT = 1;
#[doc = " This indicates that the EO is in active state."]
pub const ClEoStateT_CL_EO_STATE_ACTIVE: ClEoStateT = 2;
#[doc = " This indicates that the EO is in standby state."]
pub const ClEoStateT_CL_EO_STATE_STDBY: ClEoStateT = 4;
#[doc = " This indicates that the EO is in suspended state."]
pub const ClEoStateT_CL_EO_STATE_SUSPEND: ClEoStateT = 8;
#[doc = " This indicates that the EO is in stopped state."]
pub const ClEoStateT_CL_EO_STATE_STOP: ClEoStateT = 16;
#[doc = " This indicates that the EO is in killed state."]
pub const ClEoStateT_CL_EO_STATE_KILL: ClEoStateT = 32;
#[doc = " This indicates that the state of the EO is resumed from the standby state."]
pub const ClEoStateT_CL_EO_STATE_RESUME: ClEoStateT = 64;
#[doc = " This indicates that the EO is in failed state."]
pub const ClEoStateT_CL_EO_STATE_FAILED: ClEoStateT = 128;
#[doc = " This indicates that the EO is thread safe."]
pub const ClEoStateT_CL_EO_STATE_THREAD_SAFE: ClEoStateT = 256;
#[doc = " All states"]
pub const ClEoStateT_CL_EO_STATE_BITS: ClEoStateT = 9;
#[doc = " This is the EO state enumeration."]
pub type ClEoStateT = ::std::os::raw::c_uint;
#[doc = " Use main thread for RMD message receive. If you select this, the main\n thread must not be blocked in the \\e ClEoAppCreateCallbackT and must return\n immediately. Later, the main thread will be used for RMD message receive."]
pub const ClEoApplicationTypeT_CL_EO_USE_THREAD_FOR_RECV: ClEoApplicationTypeT = 1;
#[doc = " Give main thread to user application. If you select this, the main\n thread must be blocked in the \\e ClEoAppCreateCallbackT or used by the\n application and must return only when the finalize \\e ClEoAppDeleteCallbackT\n is called."]
pub const ClEoApplicationTypeT_CL_EO_USE_THREAD_FOR_APP: ClEoApplicationTypeT = 0;
pub type ClEoApplicationTypeT = ::std::os::raw::c_uint;
#[doc = " This indicated that the Component Manager will stop heartbeat of an EO\n if \\c CL_CPM_DONT_POLL is received in the heartbeat response."]
pub const ClEoPollingTypeT_CL_EO_DONT_POLL: ClEoPollingTypeT = 0;
#[doc = " This indicated that the Component Manager will increase heartbeat\n timeout to maximum polling timeout."]
pub const ClEoPollingTypeT_CL_EO_BUSY_POLL: ClEoPollingTypeT = 1;
#[doc = " This indicated that the Component Manager will continue with\n default heartbeat timeout."]
pub const ClEoPollingTypeT_CL_EO_DEFAULT_POLL: ClEoPollingTypeT = 2;
pub type ClEoPollingTypeT = ::std::os::raw::c_uint;
#[doc = " Feedback sent by the software component being polled in response of\n heartbeat [is-Alive]."]
#[repr(C)]
pub struct ClEoSchedFeedBackT {
    #[doc = " This indicates the polling Type \\e ClEoPollingTypeT."]
    pub freq: ClEoPollingTypeT,
    #[doc = " This indicates the health of the EO."]
    pub status: ClRcT,
}
#[test]
fn bindgen_test_layout_ClEoSchedFeedBackT() {
    const UNINIT: ::std::mem::MaybeUninit<ClEoSchedFeedBackT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClEoSchedFeedBackT>(),
        8usize,
        concat!("Size of: ", stringify!(ClEoSchedFeedBackT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClEoSchedFeedBackT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClEoSchedFeedBackT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).freq) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoSchedFeedBackT),
            "::",
            stringify!(freq)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoSchedFeedBackT),
            "::",
            stringify!(status)
        )
    );
}
#[doc = " The application should initialize itself in this function callback.\n Implementation of this is mainly dependent on the\n \\e ClEoApplicationTypeT selected by you."]
pub type ClEoAppCreateCallbackT = ::std::option::Option<
    unsafe extern "C" fn(argc: ClUint32T, argv: *mut *mut ::std::os::raw::c_char) -> ClRcT,
>;
#[doc = " The application should change the state the service it is providing.\n This service can be provided by the following mechanisms: \\n\n<BR>\n \\par 1. Using EO interface:\n      It is achieved in EO library [by masking the RMD received calls].\n      You need not specify anything in this callback. \\n\n<BR>\n \\par 2. By some other interface:\n      You must fill in this callout so that it can be called whenever\n      state change is required.\n"]
pub type ClEoAppStateChgCallbackT =
    ::std::option::Option<unsafe extern "C" fn(state: ClEoStateT) -> ClRcT>;
#[doc = " The application performs cleanup in this function callback."]
pub type ClEoAppDeleteCallbackT = ::std::option::Option<unsafe extern "C" fn() -> ClRcT>;
#[doc = " The application checks the health status in this function callback."]
pub type ClEoAppHealthCheckCallbackT =
    ::std::option::Option<unsafe extern "C" fn(schFeedback: *mut ClEoSchedFeedBackT) -> ClRcT>;
#[doc = " The application performs custom action in this callback"]
pub type ClEoCustomActionT = ::std::option::Option<
    unsafe extern "C" fn(
        compId: ClCompIdT,
        wmId: ClWaterMarkIdT,
        pWaterMark: *mut ClWaterMarkT,
        wmType: ClEoWaterMarkFlagT,
        argList: ClEoActionArgListT,
    ) -> ClRcT,
>;
#[doc = " This structure is passed during the clEoCreate API and contains the EO related\n configuration parameters."]
#[repr(C)]
pub struct ClEoConfigT {
    #[doc = " The EO name."]
    pub EOname: [ClCharT; 32usize],
    #[doc = " Indicates the EO thread priority."]
    pub pri: ClOsalThreadPriorityT,
    #[doc = " Indicates the number of RMD threads."]
    pub noOfThreads: ClUint32T,
    #[doc = " The requested IOC communication port."]
    pub reqIocPort: ClIocPortT,
    #[doc = " Indicates the maximum number of EO client."]
    pub maxNoClients: ClUint32T,
    #[doc = " Indicates whether the application needs main thread or not."]
    pub appType: ClEoApplicationTypeT,
    #[doc = " Application function that is called from main during the\n initialization process."]
    pub clEoCreateCallout: ClEoAppCreateCallbackT,
    #[doc = " Application function that is called when EO needs to be\n terminated."]
    pub clEoDeleteCallout: ClEoAppDeleteCallbackT,
    #[doc = " Application function that is called when EO is moved into\n the suspended state."]
    pub clEoStateChgCallout: ClEoAppStateChgCallbackT,
    #[doc = " Application function that is called when EO healthcheck is\n performed by Component Manager."]
    pub clEoHealthCheckCallout: ClEoAppHealthCheckCallbackT,
    #[doc = " Application function that has to be called when a Water Mark\n has been hit."]
    pub clEoCustomAction: ClEoCustomActionT,
    pub needSerialization: ClBoolT,
}
#[test]
fn bindgen_test_layout_ClEoConfigT() {
    const UNINIT: ::std::mem::MaybeUninit<ClEoConfigT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClEoConfigT>(),
        104usize,
        concat!("Size of: ", stringify!(ClEoConfigT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClEoConfigT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClEoConfigT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).EOname) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(EOname)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pri) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(pri)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).noOfThreads) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(noOfThreads)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reqIocPort) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(reqIocPort)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxNoClients) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(maxNoClients)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).appType) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(appType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clEoCreateCallout) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(clEoCreateCallout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clEoDeleteCallout) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(clEoDeleteCallout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clEoStateChgCallout) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(clEoStateChgCallout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clEoHealthCheckCallout) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(clEoHealthCheckCallout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).clEoCustomAction) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(clEoCustomAction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).needSerialization) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoConfigT),
            "::",
            stringify!(needSerialization)
        )
    );
}
extern "C" {
    #[doc = " Gives the name of the executable for the EO."]
    pub fn clEoProgNameGet(pName: *mut ClCharT, maxSize: ClUint32T) -> ClRcT;
}
extern "C" {
    #[doc = " Gives the name of the EO."]
    pub fn clEoNameGet() -> *mut ClCharT;
}
extern "C" {
    pub static mut clEoProgName: *mut ClCharT;
}
