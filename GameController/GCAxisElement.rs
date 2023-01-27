//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub unsafe trait GCAxisElement: GCPhysicalInputElement {
        #[method_id(@__retain_semantics Other absoluteInput)]
        unsafe fn absoluteInput(&self) -> Option<Id<ProtocolObject<dyn GCAxisInput>, Shared>>;

        #[method_id(@__retain_semantics Other relativeInput)]
        unsafe fn relativeInput(&self) -> Id<ProtocolObject<dyn GCRelativeInput>, Shared>;
    }

    unsafe impl ProtocolType for dyn GCAxisElement {}
);