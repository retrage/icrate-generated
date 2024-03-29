//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioSocketConnection;

    unsafe impl ClassType for VZVirtioSocketConnection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for VZVirtioSocketConnection {}

extern_methods!(
    unsafe impl VZVirtioSocketConnection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(destinationPort)]
        pub unsafe fn destinationPort(&self) -> u32;

        #[method(sourcePort)]
        pub unsafe fn sourcePort(&self) -> u32;

        #[method(fileDescriptor)]
        pub unsafe fn fileDescriptor(&self) -> c_int;

        #[method(close)]
        pub unsafe fn close(&self);
    }
);
