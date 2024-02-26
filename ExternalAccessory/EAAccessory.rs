//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ExternalAccessory::*;
use crate::Foundation::*;

pub const EAConnectionIDNone: c_uint = 0;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EAAccessory;

    unsafe impl ClassType for EAAccessory {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for EAAccessory {}

extern_methods!(
    unsafe impl EAAccessory {
        #[method(isConnected)]
        pub unsafe fn isConnected(&self) -> bool;

        #[method(connectionID)]
        pub unsafe fn connectionID(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other manufacturer)]
        pub unsafe fn manufacturer(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other modelNumber)]
        pub unsafe fn modelNumber(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serialNumber)]
        pub unsafe fn serialNumber(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other firmwareRevision)]
        pub unsafe fn firmwareRevision(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hardwareRevision)]
        pub unsafe fn hardwareRevision(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Not supported"]
        #[method_id(@__retain_semantics Other dockType)]
        pub unsafe fn dockType(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other protocolStrings)]
        pub unsafe fn protocolStrings(&self) -> Id<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn EAAccessoryDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EAAccessoryDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EAAccessory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait EAAccessoryDelegate: NSObjectProtocol {
        #[optional]
        #[method(accessoryDidDisconnect:)]
        unsafe fn accessoryDidDisconnect(&self, accessory: &EAAccessory);
    }

    unsafe impl ProtocolType for dyn EAAccessoryDelegate {}
);
