//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Virtualization_VZStorageDeviceConfiguration")]
    pub struct VZUSBMassStorageDeviceConfiguration;

    #[cfg(feature = "Virtualization_VZStorageDeviceConfiguration")]
    unsafe impl ClassType for VZUSBMassStorageDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Virtualization_VZStorageDeviceConfiguration"
))]
unsafe impl NSCopying for VZUSBMassStorageDeviceConfiguration {}

#[cfg(feature = "Virtualization_VZStorageDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZUSBMassStorageDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "Virtualization_VZStorageDeviceConfiguration")]
    unsafe impl VZUSBMassStorageDeviceConfiguration {
        #[cfg(feature = "Virtualization_VZStorageDeviceAttachment")]
        #[method_id(@__retain_semantics Init initWithAttachment:)]
        pub unsafe fn initWithAttachment(
            this: Allocated<Self>,
            attachment: &VZStorageDeviceAttachment,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZStorageDeviceConfiguration`
    #[cfg(feature = "Virtualization_VZStorageDeviceConfiguration")]
    unsafe impl VZUSBMassStorageDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
