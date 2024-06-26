//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSALanguageInstance;

    unsafe impl ClassType for OSALanguageInstance {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for OSALanguageInstance {}

extern_methods!(
    unsafe impl OSALanguageInstance {
        #[cfg(feature = "OSALanguage")]
        #[method_id(@__retain_semantics Other languageInstanceWithLanguage:)]
        pub unsafe fn languageInstanceWithLanguage(language: &OSALanguage) -> Id<Self>;

        #[cfg(feature = "OSALanguage")]
        #[method_id(@__retain_semantics Init initWithLanguage:)]
        pub unsafe fn initWithLanguage(this: Allocated<Self>, language: &OSALanguage) -> Id<Self>;

        #[cfg(feature = "OSALanguage")]
        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Id<OSALanguage>;

        #[method_id(@__retain_semantics Other defaultTarget)]
        pub unsafe fn defaultTarget(&self) -> Option<Id<NSAppleEventDescriptor>>;

        #[method(setDefaultTarget:)]
        pub unsafe fn setDefaultTarget(&self, default_target: Option<&NSAppleEventDescriptor>);

        #[method_id(@__retain_semantics Other richTextFromDescriptor:)]
        pub unsafe fn richTextFromDescriptor(
            &self,
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Id<NSAttributedString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl OSALanguageInstance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
