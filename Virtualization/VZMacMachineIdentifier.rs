//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacMachineIdentifier;

    unsafe impl ClassType for VZMacMachineIdentifier {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for VZMacMachineIdentifier {}

unsafe impl NSObjectProtocol for VZMacMachineIdentifier {}

extern_methods!(
    unsafe impl VZMacMachineIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithDataRepresentation:)]
        pub unsafe fn initWithDataRepresentation(
            this: Allocated<Self>,
            data_representation: &NSData,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Id<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VZMacMachineIdentifier {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
