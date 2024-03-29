//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Virtualization_VZNetworkDeviceAttachment")]
    pub struct VZNATNetworkDeviceAttachment;

    #[cfg(feature = "Virtualization_VZNetworkDeviceAttachment")]
    unsafe impl ClassType for VZNATNetworkDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZNetworkDeviceAttachment;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Virtualization_VZNetworkDeviceAttachment")]
unsafe impl NSObjectProtocol for VZNATNetworkDeviceAttachment {}

extern_methods!(
    #[cfg(feature = "Virtualization_VZNetworkDeviceAttachment")]
    unsafe impl VZNATNetworkDeviceAttachment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZNetworkDeviceAttachment`
    #[cfg(feature = "Virtualization_VZNetworkDeviceAttachment")]
    unsafe impl VZNATNetworkDeviceAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
