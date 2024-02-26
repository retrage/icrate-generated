//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LARightState {
        #[doc(alias = "LARightStateUnknown")]
        Unknown = 0,
        #[doc(alias = "LARightStateAuthorizing")]
        Authorizing = 1,
        #[doc(alias = "LARightStateAuthorized")]
        Authorized = 2,
        #[doc(alias = "LARightStateNotAuthorized")]
        NotAuthorized = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LARight;

    unsafe impl ClassType for LARight {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LARight {}

extern_methods!(
    unsafe impl LARight {
        #[method(state)]
        pub unsafe fn state(&self) -> LARightState;

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "LocalAuthentication_LARequirement")]
        #[method_id(@__retain_semantics Init initWithRequirement:)]
        pub unsafe fn initWithRequirement(
            this: Allocated<Self>,
            requirement: &LAAuthenticationRequirement,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(authorizeWithLocalizedReason:completion:)]
        pub unsafe fn authorizeWithLocalizedReason_completion(
            &self,
            localized_reason: &NSString,
            handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(checkCanAuthorizeWithCompletion:)]
        pub unsafe fn checkCanAuthorizeWithCompletion(&self, handler: &Block<dyn Fn(*mut NSError)>);

        #[method(deauthorizeWithCompletion:)]
        pub unsafe fn deauthorizeWithCompletion(&self, handler: &Block<dyn Fn()>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LARight {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
