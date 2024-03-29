//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_protocol!(
    pub unsafe trait VZVirtioConsoleDeviceDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "Virtualization_VZConsoleDevice",
            feature = "Virtualization_VZVirtioConsolePort"
        ))]
        #[optional]
        #[method(consoleDevice:didOpenPort:)]
        unsafe fn consoleDevice_didOpenPort(
            &self,
            console_device: &VZVirtioConsoleDevice,
            console_port: &VZVirtioConsolePort,
        );

        #[cfg(all(
            feature = "Virtualization_VZConsoleDevice",
            feature = "Virtualization_VZVirtioConsolePort"
        ))]
        #[optional]
        #[method(consoleDevice:didClosePort:)]
        unsafe fn consoleDevice_didClosePort(
            &self,
            console_device: &VZVirtioConsoleDevice,
            console_port: &VZVirtioConsolePort,
        );
    }

    unsafe impl ProtocolType for dyn VZVirtioConsoleDeviceDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Virtualization_VZConsoleDevice")]
    pub struct VZVirtioConsoleDevice;

    #[cfg(feature = "Virtualization_VZConsoleDevice")]
    unsafe impl ClassType for VZVirtioConsoleDevice {
        #[inherits(NSObject)]
        type Super = VZConsoleDevice;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Virtualization_VZConsoleDevice")]
unsafe impl NSObjectProtocol for VZVirtioConsoleDevice {}

extern_methods!(
    #[cfg(feature = "Virtualization_VZConsoleDevice")]
    unsafe impl VZVirtioConsoleDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn VZVirtioConsoleDeviceDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn VZVirtioConsoleDeviceDelegate>>,
        );

        #[cfg(feature = "Virtualization_VZVirtioConsolePortArray")]
        #[method_id(@__retain_semantics Other ports)]
        pub unsafe fn ports(&self) -> Id<VZVirtioConsolePortArray>;
    }
);
