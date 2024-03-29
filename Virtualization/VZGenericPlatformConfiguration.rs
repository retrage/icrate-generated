//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Virtualization_VZPlatformConfiguration")]
    pub struct VZGenericPlatformConfiguration;

    #[cfg(feature = "Virtualization_VZPlatformConfiguration")]
    unsafe impl ClassType for VZGenericPlatformConfiguration {
        #[inherits(NSObject)]
        type Super = VZPlatformConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Virtualization_VZPlatformConfiguration"
))]
unsafe impl NSCopying for VZGenericPlatformConfiguration {}

#[cfg(feature = "Virtualization_VZPlatformConfiguration")]
unsafe impl NSObjectProtocol for VZGenericPlatformConfiguration {}

extern_methods!(
    #[cfg(feature = "Virtualization_VZPlatformConfiguration")]
    unsafe impl VZGenericPlatformConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Virtualization_VZGenericMachineIdentifier")]
        #[method_id(@__retain_semantics Other machineIdentifier)]
        pub unsafe fn machineIdentifier(&self) -> Id<VZGenericMachineIdentifier>;

        #[cfg(feature = "Virtualization_VZGenericMachineIdentifier")]
        #[method(setMachineIdentifier:)]
        pub unsafe fn setMachineIdentifier(&self, machine_identifier: &VZGenericMachineIdentifier);
    }
);

extern_methods!(
    /// Methods declared on superclass `VZPlatformConfiguration`
    #[cfg(feature = "Virtualization_VZPlatformConfiguration")]
    unsafe impl VZGenericPlatformConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
