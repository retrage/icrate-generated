//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAccountAuthenticationModificationRequest")]
    pub struct ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest;

    #[cfg(feature = "ASAccountAuthenticationModificationRequest")]
    unsafe impl ClassType
        for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
    {
        #[inherits(NSObject)]
        type Super = ASAccountAuthenticationModificationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ASAccountAuthenticationModificationRequest")]
unsafe impl NSObjectProtocol
    for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
{
}

extern_methods!(
    #[cfg(feature = "ASAccountAuthenticationModificationRequest")]
    unsafe impl ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {
        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Init initWithUser:serviceIdentifier:userInfo:)]
        pub unsafe fn initWithUser_serviceIdentifier_userInfo(
            this: Allocated<Self>,
            user: &NSString,
            service_identifier: &ASCredentialServiceIdentifier,
            user_info: Option<&NSDictionary>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString>;

        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other serviceIdentifier)]
        pub unsafe fn serviceIdentifier(&self) -> Id<ASCredentialServiceIdentifier>;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "ASAccountAuthenticationModificationRequest")]
    unsafe impl ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
