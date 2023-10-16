pub use crate::clBindings::clCommon::*;
/* automatically generated by rust-bindgen 0.66.1 */

#[doc = " The type of the handle for the queue."]
pub type ClQueueT = ClPtrT;
#[doc = " The type of the handle for the queue node."]
pub type ClQueueNodeT = ClHandleT;
#[doc = " The type of the handle for the user-data."]
pub type ClQueueDataT = ClPtrT;
#[doc = "  \\brief Walk Callback gets called, whenever traverse happens on the Queue.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param userData (in) Data of the node is being accessed.\n  \\param userArg (in) User arg of the callback function\n\n  \\retval none\n\n  \\par Description:\n   This Walk callback function gets called, whenever user performs traverse\n   on the Queue.\n\n  \\par Library File:\n   ClUtil\n\n  \\sa clQueueCreate(), clQueueDelete()\n"]
pub type ClQueueWalkCallbackT = ::std::option::Option<
    unsafe extern "C" fn(userData: ClQueueDataT, userArg: *mut ::std::os::raw::c_void) -> ClRcT,
>;
#[doc = "  \\brief Dequeue callback gets called, whenever a Node is getting deleted.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param userData (in) Data of the node is being deleted.\n\n  \\retval none\n\n  \\par Description:\n   This dequeue callback function gets called, whenever user performs node\n   deletion on the Queue. The Data of the node will be exposed to the user\n   on the callback. This is the place where user can cleanup their data.\n\n  \\par Library File:\n   ClUtil\n\n  \\sa clQueueCreate(), clQueueDelete()\n"]
pub type ClQueueDequeueCallbackT =
    ::std::option::Option<unsafe extern "C" fn(userData: ClQueueDataT)>;
extern "C" {
    #[doc = "  \\brief Creates a queue.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param maxSize (in) Maximum size of the Queue. It\n  specifies the maximum number of elements that can exist at\n  any point of time in the Queue. This must be an unsigned integer.\n  You can enqueue elements into the queue until this maximum limit is reached.\n  If you specify this parameter as 0, then there is no limit\n  on the size of the Queue.\n\n  \\param fpUserDequeueCallBack (in) Pointer to the user's dequeue callback function.\n  This function accepts a parameter of type \\e ClQueueDataT.\n  After dequeueing, the dequeued user-data is passed as an\n  argument to the callback function.\n\n  \\param fpUserDestroyCallBack (in) Pointer to the user's destroy callback function.\n  This function accepts a parameter of type \\e ClQueueDataT.\n  While destroying the queue by clQueueDelete(), for each element in the queue this callback\n  function is called.\n\n  \\param pQueueHandle (out) Pointer to the variable of type\n  \\e ClQueueT in which the function returns a valid Queue handle.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_NO_MEMORY On memory allocation failure.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL pointer.\n\n  \\note\n  Returned error is a combination of the component Id and error code.\n  Use \\c CL_GET_ERROR_CODE(RC) defined in clCommonErrors.h to get the error code.\n\n  \\par Description:\n  This API is used to create and initialize a queue.\n\n  \\par Library File:\n   ClUtil\n\n  \\sa clQueueCreate()\n"]
    pub fn clQueueCreate(
        maxSize: ClUint32T,
        fpUserDequeueCallBack: ClQueueDequeueCallbackT,
        fpUserDestroyCallBack: ClQueueDequeueCallbackT,
        pQueueHandle: *mut ClQueueT,
    ) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Enqueues an element (user-data) into the Queue.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param queueHandle (in) Handle of queue returned by \\e clQueueCreate API.\n  \\param userData (in) User-data. Memory allocation and deallocation\n  for user-data must be done by you. The data of the node to be stored in\n  the queue.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_NO_MEMORY On memory allocation failure.\n  \\retval CL_ERR_MAXSIZE_REACHED If the maximum size is reached.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n\n  \\note\n  Returned error is a combination of the component Id and error code.\n  Use \\c CL_GET_ERROR_CODE(RET_CODE) defined in clCommonErrors.h to get the error code.\n\n  \\par Description:\n  This API is used to enqueue an element (user-data) into the queue.\n\n  \\par Library File:\n   ClUtil\n\n  \\sa clQueueCreate()\n"]
    pub fn clQueueNodeInsert(queueHandle: ClQueueT, userData: ClQueueDataT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Dequeues an element from the queue.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param queueHandle (in) Handle of queue returned by \\e clQueueCreate API.\n  \\param userData (out) Handle of userData, userData of the dequeued node will be returned.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle.\n  \\retval CL_ERR_NULL_POINTER On passing null pointer of userData.\n  \\retval CL_ERR_NOT_EXIST If the queue is empty.\n\n  \\note\n   Returned error is a combination of the component Id and error code.\n   Use \\c CL_GET_ERROR_CODE(RET_CODE) defined in clCommonErrors.h to get the error code.\n\n  \\par Description:\n  This API is used to dequeue the element from the front of the queue. The\n  user dequeue callback, registered during the creation time, is called with the element (user-data)\n  that is dequeued.\n\n  \\par Library File:\n   ClUtil\n\n  \\sa clQueueCreate()\n"]
    pub fn clQueueNodeDelete(queueHandle: ClQueueT, userData: *mut ClQueueDataT) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Walks through the queue.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param queueHandle (in) handle of queue returned by \\e clQueueCreate Api.\n\n  \\param fpUserWalkFunction (in) pointer to the callback function.\n  it accepts the following two parameters:\n  \\arg ClQueueDataT\n  \\arg void *\n  each of elements in the queue is passed one by one as the\n  first argument to the callback function.\n\n  \\param userArg (in) user-specified argument. this variable is passed as the second\n  argument to the user's call back function.\n\n  \\retval CL_OK the api executed successfully.\n  \\retval CL_ERR_NULL_POINTER on passing a null pointer.\n  \\retval CL_ERR_INVALID_HANDLE on passing an invalid handle\n\n  \\note\n  returned error is a combination of the component id and error code.\n  use \\c CL_GET_ERROR_CODE(RET_CODE) defined in clCommonErrors.h to get the error code.\n\n  \\par Description\n  This Api is used to perform a walk on the queue. The user-specified\n  callback function is called with every element (user data) in the queue.\n\n  \\par Library File\n   ClUtil\n\n  \\sa clQueueCreate(), clQueueDelete()\n"]
    pub fn clQueueWalk(
        queueHandle: ClQueueT,
        fpUserWalkFunction: ClQueueWalkCallbackT,
        userArg: *mut ::std::os::raw::c_void,
    ) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Retrieves the number of data elements in the queue.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param queueHandle (in) Handle of queue returned by the \\e clQueueCreate API.\n  \\param pSize (out) Pointer to variable of type \\e ClUint32T, in which the\n   size of the queue is returned.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL value for callback function.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle\n\n  \\note\n  Returned error is a combination of the component Id and error code.\n  Use \\c CL_GET_ERROR_CODE(RET_CODE) defined in clCommonErrors.h to get the error code.\n\n  \\par description:\n  This API is used to retrieve the number of data elements in the queue.\n\n  \\par Library File:\n   ClUtil\n\n  \\sa clQueueCreate()\n"]
    pub fn clQueueSizeGet(queueHandle: ClQueueT, pSize: *mut ClUint32T) -> ClRcT;
}
extern "C" {
    #[doc = "  \\brief Destroys the queue.\n\n  \\par Header File:\n   clQueueApi.h\n\n  \\param pQueueHandle(in) Pointer to the queue handle returned by \\e clQueueCreate API.\n\n  \\retval CL_OK The API executed successfully.\n  \\retval CL_ERR_NULL_POINTER On passing a NULL pointer.\n  \\retval CL_ERR_INVALID_HANDLE On passing an invalid handle\n\n  \\note\n  Returned error is a combination of the component Id and error code.\n  Use \\c CL_GET_ERROR_CODE(RET_CODE) defined in clCommonErrors.h to get the error code.\n\n  \\par description:\n  This API is used to delete all the elements in the queue.\n  The destroy callback function, registered during creation is called for\n  every element in the queue.\n\n  \\par Library File:\n   ClUtil\n\n  \\sa clQueueCreate()\n"]
    pub fn clQueueDelete(pQueueHandle: *mut ClQueueT) -> ClRcT;
}