//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKBackForwardList;

    unsafe impl ClassType for WKBackForwardList {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKBackForwardList {}

extern_methods!(
    unsafe impl WKBackForwardList {
        #[cfg(feature = "WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other backItem)]
        pub unsafe fn backItem(&self) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other forwardItem)]
        pub unsafe fn forwardItem(&self) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other backList)]
        pub unsafe fn backList(&self) -> Id<NSArray<WKBackForwardListItem>>;

        #[cfg(feature = "WKBackForwardListItem")]
        #[method_id(@__retain_semantics Other forwardList)]
        pub unsafe fn forwardList(&self) -> Id<NSArray<WKBackForwardListItem>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKBackForwardList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
