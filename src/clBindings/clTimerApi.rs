pub use crate::clBindings::clCommon::*;
pub use crate::clBindings::clBufferApi::*;
/* automatically generated by rust-bindgen 0.66.1 */

#[doc = " The type fo the callback fucntion that will be called on timer expiry.\n Cast your function to 'ClRcT (*) (void *)'\n"]
pub type ClTimerCallBackT =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ClRcT>;
pub type ClTimerReplicationCallbackT =
    ::std::option::Option<unsafe extern "C" fn(arg1: ClBufferHandleT) -> ClRcT>;
#[doc = " The type of the handle identifying the timer.\n\n"]
pub type ClTimerHandleT = ClPtrT;
#[doc = " The timeout value in seconds and milliseconds."]
#[repr(C)]
pub struct ClTimerTimeOutT {
    #[doc = " Number of seconds of the timeout"]
    pub tsSec: ClUint32T,
    #[doc = " Number of Milliseconds.  Its ok for this to be > 1000 (i.e. more than 1 second)"]
    pub tsMilliSec: ClUint32T,
}
#[test]
fn bindgen_test_layout_ClTimerTimeOutT() {
    const UNINIT: ::std::mem::MaybeUninit<ClTimerTimeOutT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClTimerTimeOutT>(),
        8usize,
        concat!("Size of: ", stringify!(ClTimerTimeOutT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClTimerTimeOutT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClTimerTimeOutT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tsSec) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerTimeOutT),
            "::",
            stringify!(tsSec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tsMilliSec) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerTimeOutT),
            "::",
            stringify!(tsMilliSec)
        )
    );
}
#[doc = " It contains the timer library configuration information.  It specifies\n the resolution of the timer and the priority of the timer library's task."]
#[repr(C)]
pub struct ClTimerConfigT {
    #[doc = " Timer resolution in milliseconds. It cannot be less than 10ms.\n The default is 10 milliseconds."]
    pub timerResolution: ClUint32T,
    #[doc = " Timer task priority. This value can vary between 1 and 160.\n The default value is 150."]
    pub timerTaskPriority: ClUint32T,
}
#[test]
fn bindgen_test_layout_ClTimerConfigT() {
    const UNINIT: ::std::mem::MaybeUninit<ClTimerConfigT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClTimerConfigT>(),
        8usize,
        concat!("Size of: ", stringify!(ClTimerConfigT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClTimerConfigT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClTimerConfigT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).timerResolution) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerConfigT),
            "::",
            stringify!(timerResolution)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).timerTaskPriority) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerConfigT),
            "::",
            stringify!(timerTaskPriority)
        )
    );
}
#[doc = " Fire just once"]
pub const ClTimerTypeT_CL_TIMER_ONE_SHOT: ClTimerTypeT = 0;
#[doc = " Fire periodically"]
pub const ClTimerTypeT_CL_TIMER_REPETITIVE: ClTimerTypeT = 1;
#[doc = " Fire periodically"]
pub const ClTimerTypeT_CL_TIMER_VOLATILE: ClTimerTypeT = 2;
#[doc = " Fire periodically"]
pub const ClTimerTypeT_CL_TIMER_MAX_TYPE: ClTimerTypeT = 3;
#[doc = "type of action on timer expiry. repitive start automaticaly starts after timeouts. CL_TIMER_ONE_SHOT:Timer has not started after timeout."]
pub type ClTimerTypeT = ::std::os::raw::c_uint;
#[doc = " Use the timer thread"]
pub const ClTimerContextT_CL_TIMER_TASK_CONTEXT: ClTimerContextT = 0;
#[doc = " A new thread will be created to invoke the callback."]
pub const ClTimerContextT_CL_TIMER_SEPARATE_CONTEXT: ClTimerContextT = 1;
#[doc = " A new thread will be created to invoke the callback."]
pub const ClTimerContextT_CL_TIMER_MAX_CONTEXT: ClTimerContextT = 2;
#[doc = " When the timer expires, decides the method of invocation of the timer\n callback function. Either timer callback function will be called from same\n thread context as the timer itself or new separate thread context.\n Timer task context is slightly more efficient and accurate.  However, if you\n chose timer task context, your callback function must complete rapidly and\n not block -- or you will be denying other timer expirations from being\n handled.\n"]
pub type ClTimerContextT = ::std::os::raw::c_uint;
#[repr(C)]
pub struct ClTimerStats {
    pub type_: ClTimerTypeT,
    pub context: ClTimerContextT,
    pub timeOut: ClTimeT,
    pub expiry: ClTimeT,
}
#[test]
fn bindgen_test_layout_ClTimerStats() {
    const UNINIT: ::std::mem::MaybeUninit<ClTimerStats> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClTimerStats>(),
        24usize,
        concat!("Size of: ", stringify!(ClTimerStats))
    );
    assert_eq!(
        ::std::mem::align_of::<ClTimerStats>(),
        8usize,
        concat!("Alignment of ", stringify!(ClTimerStats))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerStats),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).context) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerStats),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).timeOut) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerStats),
            "::",
            stringify!(timeOut)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).expiry) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClTimerStats),
            "::",
            stringify!(expiry)
        )
    );
}
pub type ClTimerStatsT = ClTimerStats;
extern "C" {
    #[doc = "  \\brief Configures the Timer library.\n\n  \\param pConfigData Pointer to instance of configuration structure.\n  You must pass a pointer to a \\e ClTimerConfigT as the input.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_TIMER_ERR_INVLD_PARAM On passing an invalid parameter.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL pointer.\n\n  \\par Description:\n  This API is used to configure the timer service library. The configurable\n  parameters in \\e ClTimerConfigT are:\n   -# <B> timerResolution </B>:\n  This value is in miliseconds and cannot be less than 10ms. Default value is 10 milliseconds. \\n\n   -# <B> timerTaskPriority </B>:\n  This value can vary between 1 and 160. Default value is 150.\n"]
    pub fn clTimerConfigInitialize(pConfigData: *mut ::std::os::raw::c_void) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Initializes the Timer library.\n\n  \\param pConfig\n\n  \\retval CL_OK The API executed successfully.\n  \\retval ERROR On failure.\n\n  \\par Description:\n  This API is used to initialize the timer service library. After invoking this API\n  you can create, start, stop and destroy timers.\n\n  \\sa clTimerFinalize()"]
    pub fn clTimerInitialize(pConfig: ClPtrT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Cleans up the Timer library.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval ERROR On failure.\n\n  \\par Description:\n  This API is used to clean up the timer service library. This API is invoked\n  during the system shutdown process or when timers are no longer needed.\n"]
    pub fn clTimerFinalize() -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Creates a timer.\n\n  \\param timeOut Timeout value of the timer.\n\n  \\param type Type of the timer to be created. It can be either One-shot or repetitive.\n\n  \\param timerTaskSpawn Determines whether the user-function invoked is\n  in a separate task or in the same context as the timer-task.\n\n  \\param fpAction Function to be called after timer expiry.\n\n  \\param pActionArgument Argument to be passed to the callback\n  function (fpAction in this case).\n\n  \\param pTimerHandle (out) Pointer to the memory location where the timer handle\n  created is being copied.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_TIMER_ERR_INVLD_PARAM On passing invalid parameters.\n  \\retval CL_ERR_NO_MEMORY On memory allocation failure.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL pointer.\n  \\retval CL_TIMER_ERR_NULL_CALLBACK_FUNCTION On passing a NULL callback function.\n  \\retval CL_TIMER_ERR_INVALID_TYPE On passing an invalid type.\n  \\retval CL_TIMER_ERR_INVALID_CONTEXT_TYPE On passing an invalid context.\n\n  \\par Description:\n  This API is used to create a new timer. This timer would remain inactive\n  until the timer is started. The callback function would be executed in the\n  context of the timer task when the timer expires. This API returns a\n  handle which needs to be specified whenever you want to start, stop, restart or destroy\n  the timer.\n"]
    pub fn clTimerCreate(
        timeOut: ClTimerTimeOutT,
        type_: ClTimerTypeT,
        timerTaskSpawn: ClTimerContextT,
        fpAction: ClTimerCallBackT,
        pActionArgument: *mut ::std::os::raw::c_void,
        pTimerHandle: *mut ClTimerHandleT,
    ) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Deletes a timer.\n\n  \\note\n  If the timer being deleted is active, then it is made inactive and deleted.\n\n  \\param pTimerHandle (out) Pointer to timer handle being deleted. The contents are set to 0.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL pointer.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n  \\retval CL_TIMER_ERR_INVALID If the internal timer representation is invalid.\n\n  \\par Description:\n  This API is used to delete an existing timer. It is invoked by the\n  application after the timer has expired.\n  Typically, this API is invoked during the time of application exit, but\n  it can also be called at any other time.\n"]
    pub fn clTimerDelete(pTimerHandle: *mut ClTimerHandleT) -> ClRcT;
}
extern "C" {
    pub fn clTimerDeleteAsync(pTimerHandle: *mut ClTimerHandleT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Starts a timer.\n\n  \\param timerHandle Handle of the timer being started.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n  \\retval CL_TIMER_ERR_INVALID If the internal timer representation is invalid.\n\n  \\par Description:\n  This API is used to start a timer. Before the timer can be started,\n  the timer must be created. The callback API\n  is executed when the timeout occurs. The callback\n  API would be executed in the context of the timer task.\n"]
    pub fn clTimerStart(timerHandle: ClTimerHandleT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\breif Stops a timer.\n\n  \\note\n  This API only stops the timer and does not destroy it.\n\n  \\param timerHandle Handle of the timer being stopped.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n  \\retval CL_TIMER_ERR_INVALID If the internal timer representation is invalid.\n\n  \\par Description:\n  This API is used to stop a timer. After invoking this API, the timer becomes inactive.\n"]
    pub fn clTimerStop(timerHandle: ClTimerHandleT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Creates a new timer and activates it.\n\n  \\param timeOut Timeout value of the timer.\n  \\param type Type of the timer to be created. It can be either One-shot or repetitive.\n  \\param timerTaskSpawn Determines whether the user-function invoked is\n  in a separate task or in the same context as the timer-task.\n  \\param fpAction Function to be called after timer expiry.\n  \\param actionArgument Argument to be passed to the callback function (fpAction in this case).\n  \\param  pTimerHandle (out) Pointer to the memory location where the timer handle\n  created is being copied.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_INVALID_PARAMETER  On passing an invalid parameter.\n  \\retval CL_ERR_NO_MEMORY On memory allocation failure.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL pointer.\n  \\retval CL_TIMER_ERR_NULL_CALLBACK_FUNCTION On passing an invalid callback function.\n  \\retval CL_TIMER_ERR_INVALID_TYPE If type is invalid.\n\n  \\par Description:\n  This API is used to create a new timer and activate it. It is essentially a\n  combination of clTimerCreate() and clTimerStart().\n  This API is useful when you want to create a new timer and\n  activate it at the time of its creation."]
    pub fn clTimerCreateAndStart(
        timeOut: ClTimerTimeOutT,
        type_: ClTimerTypeT,
        timerTaskSpawn: ClTimerContextT,
        fpAction: ClTimerCallBackT,
        pActionArgument: *mut ::std::os::raw::c_void,
        pTimerHandle: *mut ClTimerHandleT,
    ) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Restarts a timer.\n\n  \\param timerHandle Handle of the timer being restarted.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n  \\retval CL_TIMER_ERR_INVLD_STATE If timer is in invalid state.\n  \\retval CL_TIMER_ERR_INVALID If the internal timer representation is invalid.\n\n  \\par Description:\n  This API is used to restart a timer.\n"]
    pub fn clTimerRestart(timerHandle: ClTimerHandleT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Updates a timer.\n\n  \\param timerHandle Handle of the timer being updated.\n  \\param newTimeout New timeout value for the timer.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n  \\retval CL_TIMER_ERR_INVALID If the internal timer representation is invalid.\n\n  \\par Description:\n  This API is used to update the timeout value of a timer.\n"]
    pub fn clTimerUpdate(timerHandle: ClTimerHandleT, newTimeout: ClTimerTimeOutT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Returns the timer type.\n\n  \\param timerHandle Handle of the timer.\n\n  \\param  pTimerType  (out) The pointer to the location to which the type of the\n  timer is being copied. It can have two possible values:\n  If the value is -\n  \\arg  0: The timer type is One-shot.\n  \\arg  1: The timer type is repetitive.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL pointer.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n  \\retval CL_TIMER_ERR_INVALID If the internal timer representation is invalid.\n\n  \\par Description:\n  This API is used to query and return for a specific type of timer, whether it is one-shot or repetitive.\n"]
    pub fn clTimerTypeGet(timerHandle: ClTimerHandleT, pTimerType: *mut ClUint32T) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterRegister(
        clusterCallback: ClTimerCallBackT,
        replicationCallback: ClTimerReplicationCallbackT,
    ) -> ClRcT;
}
extern "C" {
    pub fn clTimerCreateCluster(
        timeOut: ClTimerTimeOutT,
        timerType: ClTimerTypeT,
        timerContext: ClTimerContextT,
        timerData: *mut ::std::os::raw::c_void,
        timerDataSize: ClUint32T,
        pTimerHandle: *mut ClTimerHandleT,
    ) -> ClRcT;
}
extern "C" {
    pub fn clTimerCreateAndStartCluster(
        timeOut: ClTimerTimeOutT,
        timerType: ClTimerTypeT,
        timerContext: ClTimerContextT,
        timerData: *mut ::std::os::raw::c_void,
        timerDataSize: ClUint32T,
        pTimerHandle: *mut ClTimerHandleT,
    ) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterPack(timer: ClTimerHandleT, msg: ClBufferHandleT) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterPackAll(msg: ClBufferHandleT) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterUnpack(msg: ClBufferHandleT, pTimerHandle: *mut ClTimerHandleT) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterUnpackAll(msg: ClBufferHandleT) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterFree(pTimerHandle: *mut ClTimerHandleT) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterConfigureAll() -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterConfigure(pTimerHandle: *mut ClTimerHandleT) -> ClRcT;
}
extern "C" {
    pub fn clTimerClusterSync() -> ClRcT;
}
extern "C" {
    pub fn clTimerIsRunning(timerHandle: ClTimerHandleT, pState: *mut ClBoolT) -> ClRcT;
}
extern "C" {
    pub fn clTimerIsStopped(timerHandle: ClTimerHandleT, pState: *mut ClBoolT) -> ClRcT;
}
extern "C" {
    pub fn clTimerStatsGet(ppStats: *mut *mut ClTimerStatsT, pNumTimers: *mut ClUint32T) -> ClRcT;
}
extern "C" {
    pub fn clTimerCheckAndDelete(pTimerHandle: *mut ClTimerHandleT) -> ClRcT;
}
