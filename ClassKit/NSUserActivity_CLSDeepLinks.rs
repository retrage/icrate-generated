//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "CLSDeepLinks" on [`NSUserActivity`].
    #[doc(alias = "CLSDeepLinks")]
    pub unsafe trait NSUserActivityCLSDeepLinks {
        #[method(isClassKitDeepLink)]
        unsafe fn isClassKitDeepLink(&self) -> bool;

        #[method_id(@__retain_semantics Other contextIdentifierPath)]
        unsafe fn contextIdentifierPath(&self) -> Option<Id<NSArray<NSString>>>;
    }

    unsafe impl NSUserActivityCLSDeepLinks for NSUserActivity {}
);
