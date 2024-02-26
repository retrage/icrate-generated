//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAAuthenticationRequirement;

    unsafe impl ClassType for LAAuthenticationRequirement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LAAuthenticationRequirement {}

extern_methods!(
    unsafe impl LAAuthenticationRequirement {
        #[method_id(@__retain_semantics Other defaultRequirement)]
        pub unsafe fn defaultRequirement() -> Id<LAAuthenticationRequirement>;

        #[method_id(@__retain_semantics Other biometryRequirement)]
        pub unsafe fn biometryRequirement() -> Id<LAAuthenticationRequirement>;

        #[method_id(@__retain_semantics Other biometryCurrentSetRequirement)]
        pub unsafe fn biometryCurrentSetRequirement() -> Id<LAAuthenticationRequirement>;

        #[method_id(@__retain_semantics Other biometryRequirementWithFallback:)]
        pub unsafe fn biometryRequirementWithFallback(
            fallback: &LABiometryFallbackRequirement,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LAAuthenticationRequirement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LABiometryFallbackRequirement;

    unsafe impl ClassType for LABiometryFallbackRequirement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LABiometryFallbackRequirement {}

extern_methods!(
    unsafe impl LABiometryFallbackRequirement {
        #[method_id(@__retain_semantics Other defaultRequirement)]
        pub unsafe fn defaultRequirement() -> Id<LABiometryFallbackRequirement>;

        #[method_id(@__retain_semantics Other devicePasscodeRequirement)]
        pub unsafe fn devicePasscodeRequirement() -> Id<LABiometryFallbackRequirement>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LABiometryFallbackRequirement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
