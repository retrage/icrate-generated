//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_protocol!(
    pub unsafe trait VZVirtualMachineDelegate: NSObjectProtocol {
        #[cfg(feature = "Virtualization_VZVirtualMachine")]
        #[optional]
        #[method(guestDidStopVirtualMachine:)]
        unsafe fn guestDidStopVirtualMachine(&self, virtual_machine: &VZVirtualMachine);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Virtualization_VZVirtualMachine"
        ))]
        #[optional]
        #[method(virtualMachine:didStopWithError:)]
        unsafe fn virtualMachine_didStopWithError(
            &self,
            virtual_machine: &VZVirtualMachine,
            error: &NSError,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Virtualization_VZNetworkDevice",
            feature = "Virtualization_VZVirtualMachine"
        ))]
        #[optional]
        #[method(virtualMachine:networkDevice:attachmentWasDisconnectedWithError:)]
        unsafe fn virtualMachine_networkDevice_attachmentWasDisconnectedWithError(
            &self,
            virtual_machine: &VZVirtualMachine,
            network_device: &VZNetworkDevice,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn VZVirtualMachineDelegate {}
);
