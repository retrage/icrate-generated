//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKQueryAnchor;

    unsafe impl ClassType for HKQueryAnchor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKQueryAnchor {}

unsafe impl NSCopying for HKQueryAnchor {}

unsafe impl NSObjectProtocol for HKQueryAnchor {}

unsafe impl NSSecureCoding for HKQueryAnchor {}

extern_methods!(
    unsafe impl HKQueryAnchor {
        #[method_id(@__retain_semantics Other anchorFromValue:)]
        pub unsafe fn anchorFromValue(value: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKQueryAnchor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
