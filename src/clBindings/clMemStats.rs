pub use crate::clBindings::clPoolIpi::*;
/* automatically generated by rust-bindgen 0.66.1 */

pub const CL_MEM_STATS_MAX_LIMIT: u32 = 0;
pub const ClMemDirection_CL_MEM_ALLOC: ClMemDirection = 0;
pub const ClMemDirection_CL_MEM_FREE: ClMemDirection = 1;
pub type ClMemDirection = ::std::os::raw::c_uint;
pub use self::ClMemDirection as ClMemDirectionT;
#[repr(C)]
pub struct ClMemStats {
    pub numAllocs: ClUint32T,
    pub numFrees: ClUint32T,
    pub currentAllocSize: ClUint32T,
    pub maxAllocSize: ClUint32T,
    pub numPools: ClUint32T,
}
#[test]
fn bindgen_test_layout_ClMemStats() {
    const UNINIT: ::std::mem::MaybeUninit<ClMemStats> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClMemStats>(),
        20usize,
        concat!("Size of: ", stringify!(ClMemStats))
    );
    assert_eq!(
        ::std::mem::align_of::<ClMemStats>(),
        4usize,
        concat!("Alignment of ", stringify!(ClMemStats))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numAllocs) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClMemStats),
            "::",
            stringify!(numAllocs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numFrees) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClMemStats),
            "::",
            stringify!(numFrees)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).currentAllocSize) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClMemStats),
            "::",
            stringify!(currentAllocSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxAllocSize) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ClMemStats),
            "::",
            stringify!(maxAllocSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numPools) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClMemStats),
            "::",
            stringify!(numPools)
        )
    );
}
pub type ClMemStatsT = ClMemStats;
#[repr(C)]
pub struct ClEoMemConfig {
    pub memLimit: ClUint32T,
    pub memLowWaterMark: ClWaterMarkT,
    pub memHighWaterMark: ClWaterMarkT,
    pub memMediumWaterMark: ClWaterMarkT,
}
#[test]
fn bindgen_test_layout_ClEoMemConfig() {
    const UNINIT: ::std::mem::MaybeUninit<ClEoMemConfig> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClEoMemConfig>(),
        56usize,
        concat!("Size of: ", stringify!(ClEoMemConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<ClEoMemConfig>(),
        8usize,
        concat!("Alignment of ", stringify!(ClEoMemConfig))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memLimit) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoMemConfig),
            "::",
            stringify!(memLimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memLowWaterMark) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoMemConfig),
            "::",
            stringify!(memLowWaterMark)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memHighWaterMark) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoMemConfig),
            "::",
            stringify!(memHighWaterMark)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memMediumWaterMark) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ClEoMemConfig),
            "::",
            stringify!(memMediumWaterMark)
        )
    );
}
pub type ClEoMemConfigT = ClEoMemConfig;
extern "C" {
    pub fn clMemStatsInitialize(pConfig: *const ClEoMemConfigT) -> ClRcT;
}
extern "C" {
    pub fn clMemStatsFinalize() -> ClRcT;
}
extern "C" {
    pub fn clMemStatsWaterMarksSet(pMemConfig: *const ClEoMemConfigT) -> ClRcT;
}
extern "C" {
    pub fn clEoMemWaterMarksUpdate(memDir: ClMemDirectionT);
}
extern "C" {
    pub fn clEoMemAdmitAllocate(size: ClUint32T) -> ClBoolT;
}
extern "C" {
    pub fn clEoMemNotifyFree(size: ClUint32T);
}