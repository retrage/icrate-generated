//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAPrivateKey;

    unsafe impl ClassType for LAPrivateKey {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LAPrivateKey {}

extern_methods!(
    unsafe impl LAPrivateKey {
        #[cfg(feature = "LAPublicKey")]
        #[method_id(@__retain_semantics Other publicKey)]
        pub unsafe fn publicKey(&self) -> Id<LAPublicKey>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
