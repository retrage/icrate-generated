//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKStoreReviewController;

    unsafe impl ClassType for SKStoreReviewController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for SKStoreReviewController {}

unsafe impl Sync for SKStoreReviewController {}

unsafe impl NSObjectProtocol for SKStoreReviewController {}

extern_methods!(
    unsafe impl SKStoreReviewController {
        #[deprecated]
        #[method(requestReview)]
        pub unsafe fn requestReview();
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKStoreReviewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
