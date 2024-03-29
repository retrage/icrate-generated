//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Virtualization_VZStorageDeviceAttachment")]
    pub struct VZDiskBlockDeviceStorageDeviceAttachment;

    #[cfg(feature = "Virtualization_VZStorageDeviceAttachment")]
    unsafe impl ClassType for VZDiskBlockDeviceStorageDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceAttachment;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Virtualization_VZStorageDeviceAttachment")]
unsafe impl NSObjectProtocol for VZDiskBlockDeviceStorageDeviceAttachment {}

extern_methods!(
    #[cfg(feature = "Virtualization_VZStorageDeviceAttachment")]
    unsafe impl VZDiskBlockDeviceStorageDeviceAttachment {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileHandle",
            feature = "Virtualization_VZDiskSynchronizationMode"
        ))]
        #[method_id(@__retain_semantics Init initWithFileHandle:readOnly:synchronizationMode:error:_)]
        pub unsafe fn initWithFileHandle_readOnly_synchronizationMode_error(
            this: Allocated<Self>,
            file_handle: &NSFileHandle,
            read_only: bool,
            synchronization_mode: VZDiskSynchronizationMode,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSFileHandle")]
        #[method_id(@__retain_semantics Other fileHandle)]
        pub unsafe fn fileHandle(&self) -> Id<NSFileHandle>;

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[cfg(feature = "Virtualization_VZDiskSynchronizationMode")]
        #[method(synchronizationMode)]
        pub unsafe fn synchronizationMode(&self) -> VZDiskSynchronizationMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZStorageDeviceAttachment`
    #[cfg(feature = "Virtualization_VZStorageDeviceAttachment")]
    unsafe impl VZDiskBlockDeviceStorageDeviceAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
