//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Virtualization_VZMemoryBalloonDeviceConfiguration")]
    pub struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration;

    #[cfg(feature = "Virtualization_VZMemoryBalloonDeviceConfiguration")]
    unsafe impl ClassType for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZMemoryBalloonDeviceConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Virtualization_VZMemoryBalloonDeviceConfiguration"
))]
unsafe impl NSCopying for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}

#[cfg(feature = "Virtualization_VZMemoryBalloonDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "Virtualization_VZMemoryBalloonDeviceConfiguration")]
    unsafe impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZMemoryBalloonDeviceConfiguration`
    #[cfg(feature = "Virtualization_VZMemoryBalloonDeviceConfiguration")]
    unsafe impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
