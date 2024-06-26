//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKStorefront;

    unsafe impl ClassType for SKStorefront {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for SKStorefront {}

unsafe impl Sync for SKStorefront {}

unsafe impl NSObjectProtocol for SKStorefront {}

extern_methods!(
    unsafe impl SKStorefront {
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKStorefront {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
