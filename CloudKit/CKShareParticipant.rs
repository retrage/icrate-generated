//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKShareParticipantAcceptanceStatus {
        #[doc(alias = "CKShareParticipantAcceptanceStatusUnknown")]
        Unknown = 0,
        #[doc(alias = "CKShareParticipantAcceptanceStatusPending")]
        Pending = 1,
        #[doc(alias = "CKShareParticipantAcceptanceStatusAccepted")]
        Accepted = 2,
        #[doc(alias = "CKShareParticipantAcceptanceStatusRemoved")]
        Removed = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKShareParticipantPermission {
        #[doc(alias = "CKShareParticipantPermissionUnknown")]
        Unknown = 0,
        #[doc(alias = "CKShareParticipantPermissionNone")]
        None = 1,
        #[doc(alias = "CKShareParticipantPermissionReadOnly")]
        ReadOnly = 2,
        #[doc(alias = "CKShareParticipantPermissionReadWrite")]
        ReadWrite = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKShareParticipantRole {
        #[doc(alias = "CKShareParticipantRoleUnknown")]
        Unknown = 0,
        #[doc(alias = "CKShareParticipantRoleOwner")]
        Owner = 1,
        #[doc(alias = "CKShareParticipantRolePrivateUser")]
        PrivateUser = 3,
        #[doc(alias = "CKShareParticipantRolePublicUser")]
        PublicUser = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated]
    pub enum CKShareParticipantType {
        #[deprecated]
        #[doc(alias = "CKShareParticipantTypeUnknown")]
        Unknown = 0,
        #[deprecated]
        #[doc(alias = "CKShareParticipantTypeOwner")]
        Owner = 1,
        #[deprecated]
        #[doc(alias = "CKShareParticipantTypePrivateUser")]
        PrivateUser = 3,
        #[deprecated]
        #[doc(alias = "CKShareParticipantTypePublicUser")]
        PublicUser = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKShareParticipant;

    unsafe impl ClassType for CKShareParticipant {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CKShareParticipant {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CKShareParticipant {}

unsafe impl NSObjectProtocol for CKShareParticipant {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CKShareParticipant {}

extern_methods!(
    unsafe impl CKShareParticipant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKUserIdentity")]
        #[method_id(@__retain_semantics Other userIdentity)]
        pub unsafe fn userIdentity(&self) -> Id<CKUserIdentity>;

        #[method(role)]
        pub unsafe fn role(&self) -> CKShareParticipantRole;

        #[method(setRole:)]
        pub unsafe fn setRole(&self, role: CKShareParticipantRole);

        #[deprecated]
        #[method(type)]
        pub unsafe fn r#type(&self) -> CKShareParticipantType;

        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: CKShareParticipantType);

        #[method(acceptanceStatus)]
        pub unsafe fn acceptanceStatus(&self) -> CKShareParticipantAcceptanceStatus;

        #[method(permission)]
        pub unsafe fn permission(&self) -> CKShareParticipantPermission;

        #[method(setPermission:)]
        pub unsafe fn setPermission(&self, permission: CKShareParticipantPermission);
    }
);
