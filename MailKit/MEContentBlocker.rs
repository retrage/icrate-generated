//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_protocol!(
    pub unsafe trait MEContentBlocker: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other contentRulesJSON)]
        unsafe fn contentRulesJSON(&self) -> Id<NSData, Shared>;
    }

    unsafe impl ProtocolType for dyn MEContentBlocker {}
);
