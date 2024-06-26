//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPressureConfiguration;

    unsafe impl ClassType for NSPressureConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSPressureConfiguration {}

extern_methods!(
    unsafe impl NSPressureConfiguration {
        #[cfg(feature = "NSEvent")]
        #[method(pressureBehavior)]
        pub unsafe fn pressureBehavior(&self) -> NSPressureBehavior;

        #[cfg(feature = "NSEvent")]
        #[method_id(@__retain_semantics Init initWithPressureBehavior:)]
        pub unsafe fn initWithPressureBehavior(
            this: Allocated<Self>,
            pressure_behavior: NSPressureBehavior,
        ) -> Id<Self>;

        #[method(set)]
        pub unsafe fn set(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPressureConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSPressureConfiguration
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method_id(@__retain_semantics Other pressureConfiguration)]
        pub unsafe fn pressureConfiguration(&self) -> Option<Id<NSPressureConfiguration>>;

        #[method(setPressureConfiguration:)]
        pub unsafe fn setPressureConfiguration(
            &self,
            pressure_configuration: Option<&NSPressureConfiguration>,
        );
    }
);
