//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTintConfiguration;

    unsafe impl ClassType for NSTintConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSTintConfiguration {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSTintConfiguration {}

unsafe impl NSObjectProtocol for NSTintConfiguration {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSTintConfiguration {}

extern_methods!(
    unsafe impl NSTintConfiguration {
        #[method_id(@__retain_semantics Other defaultTintConfiguration)]
        pub unsafe fn defaultTintConfiguration() -> Id<NSTintConfiguration>;

        #[method_id(@__retain_semantics Other monochromeTintConfiguration)]
        pub unsafe fn monochromeTintConfiguration() -> Id<NSTintConfiguration>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other tintConfigurationWithPreferredColor:)]
        pub unsafe fn tintConfigurationWithPreferredColor(color: &NSColor) -> Id<Self>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other tintConfigurationWithFixedColor:)]
        pub unsafe fn tintConfigurationWithFixedColor(color: &NSColor) -> Id<Self>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other baseTintColor)]
        pub unsafe fn baseTintColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other equivalentContentTintColor)]
        pub unsafe fn equivalentContentTintColor(&self) -> Option<Id<NSColor>>;

        #[method(adaptsToUserAccentColor)]
        pub unsafe fn adaptsToUserAccentColor(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTintConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
