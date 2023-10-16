pub use crate::clBindings::clCommon::*;
/* automatically generated by rust-bindgen 0.66.1 */

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const CL_AMS_HEALTHCHECK_KEY_MAX: u32 = 32;
pub const CL_AMS_CSI_FLAG_ADD_ONE: u32 = 1;
pub const CL_AMS_CSI_FLAG_TARGET_ONE: u32 = 2;
pub const CL_AMS_CSI_FLAG_TARGET_ALL: u32 = 4;
pub const ClAmsMgmtStateT_CL_AMS_MGMT_STATE_NONE: ClAmsMgmtStateT = 0;
pub const ClAmsMgmtStateT_CL_AMS_MGMT_STATE_DISABLED: ClAmsMgmtStateT = 1;
pub const ClAmsMgmtStateT_CL_AMS_MGMT_STATE_ENABLED: ClAmsMgmtStateT = 2;
#[doc = " Different types of states associated with AMS entities. Note, each AMS\n entity need not have all of the following states."]
pub type ClAmsMgmtStateT = ::std::os::raw::c_uint;
pub const ClAmsAdminStateT_CL_AMS_ADMIN_STATE_NONE: ClAmsAdminStateT = 0;
pub const ClAmsAdminStateT_CL_AMS_ADMIN_STATE_UNLOCKED: ClAmsAdminStateT = 1;
pub const ClAmsAdminStateT_CL_AMS_ADMIN_STATE_LOCKED_A: ClAmsAdminStateT = 2;
pub const ClAmsAdminStateT_CL_AMS_ADMIN_STATE_LOCKED_I: ClAmsAdminStateT = 3;
pub const ClAmsAdminStateT_CL_AMS_ADMIN_STATE_SHUTTINGDOWN: ClAmsAdminStateT = 4;
pub const ClAmsAdminStateT_CL_AMS_ADMIN_STATE_MAX: ClAmsAdminStateT = 5;
pub type ClAmsAdminStateT = ::std::os::raw::c_uint;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_NONE: ClAmsPresenceStateT = 0;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_UNINSTANTIATED: ClAmsPresenceStateT = 1;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_INSTANTIATING: ClAmsPresenceStateT = 2;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_INSTANTIATED: ClAmsPresenceStateT = 3;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_TERMINATING: ClAmsPresenceStateT = 4;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_RESTARTING: ClAmsPresenceStateT = 5;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_INSTANTIATION_FAILED: ClAmsPresenceStateT = 6;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_TERMINATION_FAILED: ClAmsPresenceStateT = 7;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_ABSENT: ClAmsPresenceStateT = 0;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_FAULT: ClAmsPresenceStateT = 8;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_FAULT_WTC: ClAmsPresenceStateT = 9;
pub const ClAmsPresenceStateT_CL_AMS_PRESENCE_STATE_FAULT_WTR: ClAmsPresenceStateT = 10;
pub type ClAmsPresenceStateT = ::std::os::raw::c_uint;
pub const ClAmsOperStateT_CL_AMS_OPER_STATE_NONE: ClAmsOperStateT = 0;
pub const ClAmsOperStateT_CL_AMS_OPER_STATE_ENABLED: ClAmsOperStateT = 1;
pub const ClAmsOperStateT_CL_AMS_OPER_STATE_DISABLED: ClAmsOperStateT = 2;
pub type ClAmsOperStateT = ::std::os::raw::c_uint;
pub const ClAmsHAStateT_CL_AMS_HA_STATE_NONE: ClAmsHAStateT = 0;
pub const ClAmsHAStateT_CL_AMS_HA_STATE_ACTIVE: ClAmsHAStateT = 1;
pub const ClAmsHAStateT_CL_AMS_HA_STATE_STANDBY: ClAmsHAStateT = 2;
pub const ClAmsHAStateT_CL_AMS_HA_STATE_QUIESCED: ClAmsHAStateT = 3;
pub const ClAmsHAStateT_CL_AMS_HA_STATE_QUIESCING: ClAmsHAStateT = 4;
pub type ClAmsHAStateT = ::std::os::raw::c_uint;
pub const ClAmsReadinessStateT_CL_AMS_READINESS_STATE_NONE: ClAmsReadinessStateT = 0;
pub const ClAmsReadinessStateT_CL_AMS_READINESS_STATE_OUTOFSERVICE: ClAmsReadinessStateT = 1;
pub const ClAmsReadinessStateT_CL_AMS_READINESS_STATE_INSERVICE: ClAmsReadinessStateT = 2;
pub const ClAmsReadinessStateT_CL_AMS_READINESS_STATE_STOPPING: ClAmsReadinessStateT = 3;
pub type ClAmsReadinessStateT = ::std::os::raw::c_uint;
pub const ClAmsServiceStateT_CL_AMS_SERVICE_STATE_NONE: ClAmsServiceStateT = 0;
pub const ClAmsServiceStateT_CL_AMS_SERVICE_STATE_RUNNING: ClAmsServiceStateT = 1;
pub const ClAmsServiceStateT_CL_AMS_SERVICE_STATE_STOPPED: ClAmsServiceStateT = 2;
pub const ClAmsServiceStateT_CL_AMS_SERVICE_STATE_STARTINGUP: ClAmsServiceStateT = 3;
pub const ClAmsServiceStateT_CL_AMS_SERVICE_STATE_SHUTTINGDOWN: ClAmsServiceStateT = 4;
pub const ClAmsServiceStateT_CL_AMS_SERVICE_STATE_UNAVAILABLE: ClAmsServiceStateT = 5;
pub const ClAmsServiceStateT_CL_AMS_SERVICE_STATE_HOT_STANDBY: ClAmsServiceStateT = 6;
pub type ClAmsServiceStateT = ::std::os::raw::c_uint;
pub const ClAmsNodeClassT_CL_AMS_NODE_CLASS_NONE: ClAmsNodeClassT = 0;
pub const ClAmsNodeClassT_CL_AMS_NODE_CLASS_A: ClAmsNodeClassT = 1;
pub const ClAmsNodeClassT_CL_AMS_NODE_CLASS_B: ClAmsNodeClassT = 2;
pub const ClAmsNodeClassT_CL_AMS_NODE_CLASS_C: ClAmsNodeClassT = 3;
pub const ClAmsNodeClassT_CL_AMS_NODE_CLASS_D: ClAmsNodeClassT = 4;
pub const ClAmsNodeClassT_CL_AMS_NODE_CLASS_MAX: ClAmsNodeClassT = 5;
#[doc = " Node related types"]
pub type ClAmsNodeClassT = ::std::os::raw::c_uint;
pub const ClAmsNodeClusterMemberT_CL_AMS_NODE_IS_NOT_CLUSTER_MEMBER: ClAmsNodeClusterMemberT = 0;
pub const ClAmsNodeClusterMemberT_CL_AMS_NODE_IS_CLUSTER_MEMBER: ClAmsNodeClusterMemberT = 1;
pub const ClAmsNodeClusterMemberT_CL_AMS_NODE_IS_LEAVING_CLUSTER: ClAmsNodeClusterMemberT = 2;
pub type ClAmsNodeClusterMemberT = ::std::os::raw::c_uint;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_NONE: ClAmsSGRedundancyModelT = 0;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_NO_REDUNDANCY:
    ClAmsSGRedundancyModelT = 1;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_TWO_N: ClAmsSGRedundancyModelT = 2;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_M_PLUS_N: ClAmsSGRedundancyModelT = 3;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_N_WAY: ClAmsSGRedundancyModelT = 4;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_N_WAY_ACTIVE: ClAmsSGRedundancyModelT =
    5;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_CUSTOM: ClAmsSGRedundancyModelT = 6;
pub const ClAmsSGRedundancyModelT_CL_AMS_SG_REDUNDANCY_MODEL_MAX: ClAmsSGRedundancyModelT = 7;
#[doc = " Service Group related types"]
pub type ClAmsSGRedundancyModelT = ::std::os::raw::c_uint;
pub const ClAmsSGLoadingStrategyT_CL_AMS_SG_LOADING_STRATEGY_NONE: ClAmsSGLoadingStrategyT = 0;
pub const ClAmsSGLoadingStrategyT_CL_AMS_SG_LOADING_STRATEGY_LEAST_SI_PER_SU:
    ClAmsSGLoadingStrategyT = 1;
pub const ClAmsSGLoadingStrategyT_CL_AMS_SG_LOADING_STRATEGY_LEAST_SU_ASSIGNED:
    ClAmsSGLoadingStrategyT = 2;
pub const ClAmsSGLoadingStrategyT_CL_AMS_SG_LOADING_STRATEGY_LEAST_LOAD_PER_SU:
    ClAmsSGLoadingStrategyT = 3;
pub const ClAmsSGLoadingStrategyT_CL_AMS_SG_LOADING_STRATEGY_BY_SI_PREFERENCE:
    ClAmsSGLoadingStrategyT = 4;
pub const ClAmsSGLoadingStrategyT_CL_AMS_SG_LOADING_STRATEGY_USER_DEFINED: ClAmsSGLoadingStrategyT =
    5;
pub const ClAmsSGLoadingStrategyT_CL_AMS_SG_LOADING_STRATEGY_MAX: ClAmsSGLoadingStrategyT = 6;
pub type ClAmsSGLoadingStrategyT = ::std::os::raw::c_uint;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_X_ACTIVE_AND_Y_STANDBY: ClAmsCompCapModelT = 1;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_X_ACTIVE_OR_Y_STANDBY: ClAmsCompCapModelT = 2;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_ONE_ACTIVE_OR_X_STANDBY: ClAmsCompCapModelT = 3;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_ONE_ACTIVE_OR_ONE_STANDBY: ClAmsCompCapModelT = 4;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_X_ACTIVE: ClAmsCompCapModelT = 5;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_ONE_ACTIVE: ClAmsCompCapModelT = 6;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_NON_PREINSTANTIABLE: ClAmsCompCapModelT = 7;
pub const ClAmsCompCapModelT_CL_AMS_COMP_CAP_MAX: ClAmsCompCapModelT = 8;
#[doc = " Component related types"]
pub type ClAmsCompCapModelT = ::std::os::raw::c_uint;
pub const ClAmsCompPropertyT_CL_AMS_COMP_PROPERTY_SA_AWARE: ClAmsCompPropertyT = 1;
pub const ClAmsCompPropertyT_CL_AMS_COMP_PROPERTY_PROXIED_PREINSTANTIABLE: ClAmsCompPropertyT = 2;
pub const ClAmsCompPropertyT_CL_AMS_COMP_PROPERTY_PROXIED_NON_PREINSTANTIABLE: ClAmsCompPropertyT =
    3;
pub const ClAmsCompPropertyT_CL_AMS_COMP_PROPERTY_NON_PROXIED_NON_PREINSTANTIABLE:
    ClAmsCompPropertyT = 4;
pub const ClAmsCompPropertyT_CL_AMS_COMP_PROPERTY_MAX: ClAmsCompPropertyT = 5;
pub type ClAmsCompPropertyT = ::std::os::raw::c_uint;
pub const ClAmsCompHealthcheckInvocationT_CL_AMS_COMP_HEALTHCHECK_AMF_INVOKED:
    ClAmsCompHealthcheckInvocationT = 1;
pub const ClAmsCompHealthcheckInvocationT_CL_AMS_COMP_HEALTHCHECK_CLIENT_INVOKED:
    ClAmsCompHealthcheckInvocationT = 2;
pub type ClAmsCompHealthcheckInvocationT = ::std::os::raw::c_uint;
#[repr(C)]
pub struct ClAmsCompHealthcheckKeyT {
    pub key: [ClUint8T; 32usize],
    pub keyLen: ClUint16T,
}
#[test]
fn bindgen_test_layout_ClAmsCompHealthcheckKeyT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCompHealthcheckKeyT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCompHealthcheckKeyT>(),
        34usize,
        concat!("Size of: ", stringify!(ClAmsCompHealthcheckKeyT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCompHealthcheckKeyT>(),
        2usize,
        concat!("Alignment of ", stringify!(ClAmsCompHealthcheckKeyT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCompHealthcheckKeyT),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keyLen) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCompHealthcheckKeyT),
            "::",
            stringify!(keyLen)
        )
    );
}
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_NONE: ClAmsRecoveryT = 0;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_NO_RECOMMENDATION: ClAmsRecoveryT = 1;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_COMP_RESTART: ClAmsRecoveryT = 2;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_COMP_FAILOVER: ClAmsRecoveryT = 3;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_NODE_SWITCHOVER: ClAmsRecoveryT = 4;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_NODE_FAILOVER: ClAmsRecoveryT = 5;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_NODE_FAILFAST: ClAmsRecoveryT = 6;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_CLUSTER_RESET: ClAmsRecoveryT = 7;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_APP_RESTART: ClAmsRecoveryT = 8;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_INTERNALLY_RECOVERED: ClAmsRecoveryT = 20;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_SU_RESTART: ClAmsRecoveryT = 21;
pub const ClAmsRecoveryT_CL_AMS_RECOVERY_NODE_HALT: ClAmsRecoveryT = 22;
pub const ClAmsRecoveryT_CL_AMS_EXTERNAL_RECOVERY_RESET: ClAmsRecoveryT = 30;
pub const ClAmsRecoveryT_CL_AMS_EXTERNAL_RECOVERY_REBOOT: ClAmsRecoveryT = 31;
pub const ClAmsRecoveryT_CL_AMS_EXTERNAL_RECOVERY_POWER_ON: ClAmsRecoveryT = 32;
pub const ClAmsRecoveryT_CL_AMS_EXTERNAL_RECOVERY_POWER_OFF: ClAmsRecoveryT = 33;
pub type ClAmsRecoveryT = ::std::os::raw::c_uint;
pub use self::ClAmsRecoveryT as ClAmsLocalRecoveryT;
pub type ClAmsCSIFlagsT = ClUint32T;
#[doc = " A name"]
pub type ClAmsCSITypeT = ClNameT;
pub const ClAmsCSITransitionDescriptorT_CL_AMS_CSI_NEW_ASSIGN: ClAmsCSITransitionDescriptorT = 1;
pub const ClAmsCSITransitionDescriptorT_CL_AMS_CSI_QUIESCED: ClAmsCSITransitionDescriptorT = 2;
pub const ClAmsCSITransitionDescriptorT_CL_AMS_CSI_NOT_QUIESCED: ClAmsCSITransitionDescriptorT = 3;
pub const ClAmsCSITransitionDescriptorT_CL_AMS_CSI_STILL_ACTIVE: ClAmsCSITransitionDescriptorT = 4;
pub type ClAmsCSITransitionDescriptorT = ::std::os::raw::c_uint;
#[repr(C)]
pub struct ClAmsCSIActiveDescriptorT {
    pub transitionDescriptor: ClAmsCSITransitionDescriptorT,
    pub activeCompName: ClNameT,
}
#[test]
fn bindgen_test_layout_ClAmsCSIActiveDescriptorT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCSIActiveDescriptorT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCSIActiveDescriptorT>(),
        264usize,
        concat!("Size of: ", stringify!(ClAmsCSIActiveDescriptorT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCSIActiveDescriptorT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClAmsCSIActiveDescriptorT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).transitionDescriptor) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIActiveDescriptorT),
            "::",
            stringify!(transitionDescriptor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).activeCompName) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIActiveDescriptorT),
            "::",
            stringify!(activeCompName)
        )
    );
}
#[repr(C)]
pub struct ClAmsCSIStandbyDescriptorT {
    pub activeCompName: ClNameT,
    pub standbyRank: ClUint32T,
}
#[test]
fn bindgen_test_layout_ClAmsCSIStandbyDescriptorT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCSIStandbyDescriptorT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCSIStandbyDescriptorT>(),
        264usize,
        concat!("Size of: ", stringify!(ClAmsCSIStandbyDescriptorT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCSIStandbyDescriptorT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClAmsCSIStandbyDescriptorT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).activeCompName) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIStandbyDescriptorT),
            "::",
            stringify!(activeCompName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).standbyRank) as usize - ptr as usize },
        260usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIStandbyDescriptorT),
            "::",
            stringify!(standbyRank)
        )
    );
}
#[repr(C)]
pub struct ClAmsCSIStateDescriptorT {
    pub activeDescriptor: __BindgenUnionField<ClAmsCSIActiveDescriptorT>,
    pub standbyDescriptor: __BindgenUnionField<ClAmsCSIStandbyDescriptorT>,
    pub bindgen_union_field: [u32; 66usize],
}
#[test]
fn bindgen_test_layout_ClAmsCSIStateDescriptorT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCSIStateDescriptorT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCSIStateDescriptorT>(),
        264usize,
        concat!("Size of: ", stringify!(ClAmsCSIStateDescriptorT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCSIStateDescriptorT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClAmsCSIStateDescriptorT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).activeDescriptor) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIStateDescriptorT),
            "::",
            stringify!(activeDescriptor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).standbyDescriptor) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIStateDescriptorT),
            "::",
            stringify!(standbyDescriptor)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClAmsCSIAttributeT {
    pub attributeName: *mut ClUint8T,
    pub attributeValue: *mut ClUint8T,
}
#[test]
fn bindgen_test_layout_ClAmsCSIAttributeT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCSIAttributeT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCSIAttributeT>(),
        16usize,
        concat!("Size of: ", stringify!(ClAmsCSIAttributeT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCSIAttributeT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClAmsCSIAttributeT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributeName) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIAttributeT),
            "::",
            stringify!(attributeName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributeValue) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIAttributeT),
            "::",
            stringify!(attributeValue)
        )
    );
}
#[repr(C)]
pub struct ClAmsCSIAttributeListT {
    pub attribute: *mut ClAmsCSIAttributeT,
    pub numAttributes: ClUint32T,
}
#[test]
fn bindgen_test_layout_ClAmsCSIAttributeListT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCSIAttributeListT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCSIAttributeListT>(),
        16usize,
        concat!("Size of: ", stringify!(ClAmsCSIAttributeListT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCSIAttributeListT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClAmsCSIAttributeListT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attribute) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIAttributeListT),
            "::",
            stringify!(attribute)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numAttributes) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIAttributeListT),
            "::",
            stringify!(numAttributes)
        )
    );
}
#[repr(C)]
pub struct ClAmsCSIDescriptorT {
    pub csiFlags: ClAmsCSIFlagsT,
    pub csiName: ClNameT,
    pub csiStateDescriptor: ClAmsCSIStateDescriptorT,
    pub csiAttributeList: ClAmsCSIAttributeListT,
}
#[test]
fn bindgen_test_layout_ClAmsCSIDescriptorT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCSIDescriptorT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCSIDescriptorT>(),
        544usize,
        concat!("Size of: ", stringify!(ClAmsCSIDescriptorT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCSIDescriptorT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClAmsCSIDescriptorT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).csiFlags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIDescriptorT),
            "::",
            stringify!(csiFlags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).csiName) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIDescriptorT),
            "::",
            stringify!(csiName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).csiStateDescriptor) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIDescriptorT),
            "::",
            stringify!(csiStateDescriptor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).csiAttributeList) as usize - ptr as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSIDescriptorT),
            "::",
            stringify!(csiAttributeList)
        )
    );
}
#[repr(C)]
pub struct ClAmsCSITypeDescriptorT {
    pub csiDescriptor: ClAmsCSIDescriptorT,
    pub csiType: ClAmsCSITypeT,
    pub compName: ClNameT,
}
#[test]
fn bindgen_test_layout_ClAmsCSITypeDescriptorT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsCSITypeDescriptorT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsCSITypeDescriptorT>(),
        1064usize,
        concat!("Size of: ", stringify!(ClAmsCSITypeDescriptorT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsCSITypeDescriptorT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClAmsCSITypeDescriptorT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).csiDescriptor) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSITypeDescriptorT),
            "::",
            stringify!(csiDescriptor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).csiType) as usize - ptr as usize },
        544usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSITypeDescriptorT),
            "::",
            stringify!(csiType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).compName) as usize - ptr as usize },
        802usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsCSITypeDescriptorT),
            "::",
            stringify!(compName)
        )
    );
}
pub const ClAmsPGTrackFlagT_CL_AMS_PG_TRACK_CURRENT: ClAmsPGTrackFlagT = 1;
pub const ClAmsPGTrackFlagT_CL_AMS_PG_TRACK_CHANGES: ClAmsPGTrackFlagT = 2;
pub const ClAmsPGTrackFlagT_CL_AMS_PG_TRACK_CHANGES_ONLY: ClAmsPGTrackFlagT = 4;
#[doc = " Protection group related types"]
pub type ClAmsPGTrackFlagT = ::std::os::raw::c_uint;
pub const ClAmsPGChangeT_CL_AMS_PG_NO_CHANGE: ClAmsPGChangeT = 1;
pub const ClAmsPGChangeT_CL_AMS_PG_ADDED: ClAmsPGChangeT = 2;
pub const ClAmsPGChangeT_CL_AMS_PG_REMOVED: ClAmsPGChangeT = 3;
pub const ClAmsPGChangeT_CL_AMS_PG_STATE_CHANGE: ClAmsPGChangeT = 4;
pub type ClAmsPGChangeT = ::std::os::raw::c_uint;
#[repr(C)]
pub struct ClAmsPGMemberT {
    pub compName: ClNameT,
    pub haState: ClAmsHAStateT,
    pub rank: ClUint32T,
}
#[test]
fn bindgen_test_layout_ClAmsPGMemberT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsPGMemberT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsPGMemberT>(),
        268usize,
        concat!("Size of: ", stringify!(ClAmsPGMemberT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsPGMemberT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClAmsPGMemberT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).compName) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsPGMemberT),
            "::",
            stringify!(compName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).haState) as usize - ptr as usize },
        260usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsPGMemberT),
            "::",
            stringify!(haState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rank) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsPGMemberT),
            "::",
            stringify!(rank)
        )
    );
}
#[repr(C)]
pub struct ClAmsPGNotificationT {
    pub member: ClAmsPGMemberT,
    pub change: ClAmsPGChangeT,
}
#[test]
fn bindgen_test_layout_ClAmsPGNotificationT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsPGNotificationT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsPGNotificationT>(),
        272usize,
        concat!("Size of: ", stringify!(ClAmsPGNotificationT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsPGNotificationT>(),
        4usize,
        concat!("Alignment of ", stringify!(ClAmsPGNotificationT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).member) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsPGNotificationT),
            "::",
            stringify!(member)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).change) as usize - ptr as usize },
        268usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsPGNotificationT),
            "::",
            stringify!(change)
        )
    );
}
#[repr(C)]
pub struct ClAmsPGNotificationBufferT {
    pub numItems: ClUint32T,
    pub notification: *mut ClAmsPGNotificationT,
}
#[test]
fn bindgen_test_layout_ClAmsPGNotificationBufferT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsPGNotificationBufferT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsPGNotificationBufferT>(),
        16usize,
        concat!("Size of: ", stringify!(ClAmsPGNotificationBufferT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsPGNotificationBufferT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClAmsPGNotificationBufferT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numItems) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsPGNotificationBufferT),
            "::",
            stringify!(numItems)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).notification) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsPGNotificationBufferT),
            "::",
            stringify!(notification)
        )
    );
}
#[doc = " Types for handles used by various AMS APIs"]
pub type ClAmsMgmtCCBHandleT = ClHandleT;
pub type ClAmsMgmtHandleT = ClHandleT;
pub type ClAmsClientHandleT = ClHandleT;
pub type ClAmsEventHandleT = ClHandleT;
pub type ClAmsFaultHandleT = ClHandleT;
pub type ClAmsEntityHandleT = ClHandleT;
pub type ClAmsMgmtDBHandleT = ClPtrT;
pub type ClAmsMgmtCCBBatchHandleT = ClPtrT;
#[doc = " Temporary Notes"]
#[repr(C)]
pub struct ClAmsSIDescriptorT {
    pub numberOfItems: ClUint32T,
    pub csiDefinition: *mut ClAmsCSITypeDescriptorT,
}
#[test]
fn bindgen_test_layout_ClAmsSIDescriptorT() {
    const UNINIT: ::std::mem::MaybeUninit<ClAmsSIDescriptorT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClAmsSIDescriptorT>(),
        16usize,
        concat!("Size of: ", stringify!(ClAmsSIDescriptorT))
    );
    assert_eq!(
        ::std::mem::align_of::<ClAmsSIDescriptorT>(),
        8usize,
        concat!("Alignment of ", stringify!(ClAmsSIDescriptorT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).numberOfItems) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsSIDescriptorT),
            "::",
            stringify!(numberOfItems)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).csiDefinition) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClAmsSIDescriptorT),
            "::",
            stringify!(csiDefinition)
        )
    );
}