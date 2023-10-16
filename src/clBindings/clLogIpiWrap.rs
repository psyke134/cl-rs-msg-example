pub use crate::clBindings::clIocApi::*;
pub use crate::clBindings::clLogApi::*;
pub use crate::clBindings::clTimerApi::*;
/* automatically generated by rust-bindgen 0.66.1 */

pub const CL_LOG_EMERGENCY: u32 = 1;
pub const CL_LOG_ALERT: u32 = 2;
pub const CL_LOG_CRITICAL: u32 = 3;
pub const CL_LOG_ERROR: u32 = 4;
pub const CL_LOG_WARNING: u32 = 5;
pub const CL_LOG_NOTICE: u32 = 6;
pub const CL_LOG_INFO: u32 = 7;
pub const CL_LOG_INFORMATIONAL: u32 = 7;
pub const CL_LOG_DEBUG: u32 = 8;
pub const CL_LOG_DEBUG1: u32 = 8;
pub const CL_LOG_DEBUG2: u32 = 9;
pub const CL_LOG_DEBUG3: u32 = 10;
pub const CL_LOG_DEBUG4: u32 = 11;
pub const CL_LOG_DEBUG5: u32 = 12;
pub const CL_LOG_TRACE: u32 = 12;
pub const CL_LOG_DEBUG6: u32 = 13;
pub const CL_LOG_DEBUG7: u32 = 14;
pub const CL_LOG_DEBUG8: u32 = 15;
pub const CL_LOG_DEBUG9: u32 = 16;
pub const CL_LOG_MAX: u32 = 21;
pub const CL_LOG_END: u32 = 21;
pub const CL_LOG_FILENAME_LENGTH: u32 = 80;
pub const CL_LOG_FILEPATH_LENGTH: u32 = 200;
pub const CL_LOG_FORMAT_LENGTH: u32 = 50;
pub const CL_LOG_SG: u32 = 1;
pub const CL_LOG_SU: u32 = 2;
pub const CL_LOG_COMP: u32 = 4;
pub const CL_LOG_COMPNAME_LENGTH: u32 = 50;
pub const CL_LOG_MSG_LENGTH: u32 = 178;
extern "C" {
    #[doc = " This contains the list of all common log messages used by ASP\n components."]
    pub static mut clLogCommonMsg: [*mut ClCharT; 0usize];
}
extern "C" {
    #[doc = " The default system stream handle."]
    pub static mut CL_LOG_HANDLE_SYS: ClHandleT;
}
extern "C" {
    #[doc = " The default application stream handle."]
    pub static mut CL_LOG_HANDLE_APP: ClHandleT;
}
#[doc = " The type of the handle for the stream."]
pub type ClLogStreamHdlT = ClHandleT;
#[repr(C)]
pub struct ClLogHeaderT {
    #[doc = " Severity of the message."]
    pub logSeverity: ClLogSeverityT,
    #[doc = " Handle to the stream."]
    pub streamHdl: ClLogStreamHdlT,
    #[doc = " Id of the process."]
    pub pid: ClUint32T,
    #[doc = " Physical address of the component."]
    pub iocAddress: ClIocPhysicalAddressT,
    #[doc = " Time when the message was generated."]
    pub time: ClTimerTimeOutT,
    #[doc = " Component name, this also includes the library name (if any)."]
    pub compName: [ClCharT; 50usize],
    #[doc = " The message."]
    pub msg: [ClCharT; 178usize],
}
#[test]
fn bindgen_test_layout_ClLogHeaderT() {
    const UNINIT: ::std::mem::MaybeUninit<ClLogHeaderT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClLogHeaderT>(),
        264usize,
        concat!("Size of: ", stringify!(ClLogHeaderT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClLogHeaderT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClLogHeaderT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).logSeverity) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogHeaderT),
            "::",
            stringify!(logSeverity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).streamHdl) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogHeaderT),
            "::",
            stringify!(streamHdl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pid) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogHeaderT),
            "::",
            stringify!(pid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iocAddress) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogHeaderT),
            "::",
            stringify!(iocAddress)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).time) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogHeaderT),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).compName) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogHeaderT),
            "::",
            stringify!(compName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).msg) as usize - ptr as usize },
        86usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogHeaderT),
            "::",
            stringify!(msg)
        )
    );
}
#[repr(C)]
pub struct ClLogFileConfigT {
    #[doc = "  Maximum log size."]
    pub maxRecordSize: ClUint32T,
    #[doc = "  The maximum size of file."]
    pub maxFileSize: ClUint64T,
    #[doc = " This stores the original file name."]
    pub fileName: [ClCharT; 80usize],
    #[doc = "  The location of file."]
    pub fileLoc: [ClCharT; 200usize],
}
#[test]
fn bindgen_test_layout_ClLogFileConfigT() {
    const UNINIT: ::std::mem::MaybeUninit<ClLogFileConfigT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClLogFileConfigT>(),
        296usize,
        concat!("Size of: ", stringify!(ClLogFileConfigT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClLogFileConfigT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClLogFileConfigT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxRecordSize) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFileConfigT),
            "::",
            stringify!(maxRecordSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxFileSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFileConfigT),
            "::",
            stringify!(maxFileSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fileName) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFileConfigT),
            "::",
            stringify!(fileName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fileLoc) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFileConfigT),
            "::",
            stringify!(fileLoc)
        )
    );
}
#[repr(C)]
pub struct ClLogFormatFileConfigT {
    #[doc = "  The format expression."]
    pub format: [ClCharT; 50usize],
    #[doc = " The file configuration."]
    pub fileConfig: ClLogFileConfigT,
}
#[test]
fn bindgen_test_layout_ClLogFormatFileConfigT() {
    const UNINIT: ::std::mem::MaybeUninit<ClLogFormatFileConfigT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClLogFormatFileConfigT>(),
        352usize,
        concat!("Size of: ", stringify!(ClLogFormatFileConfigT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClLogFormatFileConfigT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClLogFormatFileConfigT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).format) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFormatFileConfigT),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fileConfig) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFormatFileConfigT),
            "::",
            stringify!(fileConfig)
        )
    );
}
#[repr(C)]
pub struct ClLogFilterPatternT {
    #[doc = "  This bit pattern specifies which fields are applicable."]
    pub bitMap: ClUint32T,
    #[doc = "  The Service Group name."]
    pub sgName: ClNameT,
    #[doc = "  The Service Unit name."]
    pub suName: ClNameT,
    #[doc = "  The Component name."]
    pub compName: ClNameT,
}
#[test]
fn bindgen_test_layout_ClLogFilterPatternT() {
    const UNINIT: ::std::mem::MaybeUninit<ClLogFilterPatternT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClLogFilterPatternT>(),
        780usize,
        concat!("Size of: ", stringify!(ClLogFilterPatternT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClLogFilterPatternT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClLogFilterPatternT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bitMap) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFilterPatternT),
            "::",
            stringify!(bitMap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sgName) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFilterPatternT),
            "::",
            stringify!(sgName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).suName) as usize - ptr as usize },
        262usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFilterPatternT),
            "::",
            stringify!(suName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).compName) as usize - ptr as usize },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(ClLogFilterPatternT),
            "::",
            stringify!(compName)
        )
    );
}
pub type ClLogSeverityFlagsT = ClUint16T;
extern "C" {
    pub fn clLogLibInitialize() -> ClRcT;
}
extern "C" {
    pub fn clLogLibFinalize() -> ClRcT;
}
extern "C" {
    pub fn clLogOpen(handle: *mut ClLogStreamHdlT, fileConfig: ClLogFormatFileConfigT) -> ClRcT;
}
extern "C" {
    pub fn clLogClose(handle: ClLogStreamHdlT) -> ClRcT;
}
extern "C" {
    pub fn clLogLevelSet(severity: ClLogSeverityT) -> ClRcT;
}
extern "C" {
    pub fn clLogLevelGet(severity: *mut ClLogSeverityT) -> ClRcT;
}