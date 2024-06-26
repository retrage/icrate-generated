//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static GCRacingWheelDidConnectNotification: &'static NSString;
}

extern "C" {
    pub static GCRacingWheelDidDisconnectNotification: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCRacingWheel;

    unsafe impl ClassType for GCRacingWheel {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GCDevice")]
unsafe impl GCDevice for GCRacingWheel {}

unsafe impl NSObjectProtocol for GCRacingWheel {}

extern_methods!(
    unsafe impl GCRacingWheel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other connectedRacingWheels)]
        pub unsafe fn connectedRacingWheels() -> Id<NSSet<GCRacingWheel>>;

        #[method(acquireDeviceWithError:_)]
        pub unsafe fn acquireDeviceWithError(&self) -> Result<(), Id<NSError>>;

        #[method(relinquishDevice)]
        pub unsafe fn relinquishDevice(&self);

        #[method(isAcquired)]
        pub unsafe fn isAcquired(&self) -> bool;

        #[cfg(feature = "GCRacingWheelInput")]
        #[method_id(@__retain_semantics Other wheelInput)]
        pub unsafe fn wheelInput(&self) -> Id<GCRacingWheelInput>;

        #[method(isSnapshot)]
        pub unsafe fn isSnapshot(&self) -> bool;

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCRacingWheel>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCRacingWheel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
