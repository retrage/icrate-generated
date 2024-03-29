//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZConsolePortConfiguration;

    unsafe impl ClassType for VZConsolePortConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for VZConsolePortConfiguration {}

unsafe impl NSObjectProtocol for VZConsolePortConfiguration {}

extern_methods!(
    unsafe impl VZConsolePortConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Virtualization_VZSerialPortAttachment")]
        #[method_id(@__retain_semantics Other attachment)]
        pub unsafe fn attachment(&self) -> Option<Id<VZSerialPortAttachment>>;

        #[cfg(feature = "Virtualization_VZSerialPortAttachment")]
        #[method(setAttachment:)]
        pub unsafe fn setAttachment(&self, attachment: Option<&VZSerialPortAttachment>);
    }
);
