//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::OSAKit::*;

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
        #[cfg(feature = "OSAKit_OSALanguage")]
        #[method_id(@__retain_semantics Other languageInstanceWithLanguage:)]
        pub unsafe fn languageInstanceWithLanguage(language: &OSALanguage) -> Id<Self>;

        #[cfg(feature = "OSAKit_OSALanguage")]
        #[method_id(@__retain_semantics Init initWithLanguage:)]
        pub unsafe fn initWithLanguage(this: Allocated<Self>, language: &OSALanguage) -> Id<Self>;

        #[cfg(feature = "OSAKit_OSALanguage")]
        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Id<OSALanguage>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other defaultTarget)]
        pub unsafe fn defaultTarget(&self) -> Option<Id<NSAppleEventDescriptor>>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method(setDefaultTarget:)]
        pub unsafe fn setDefaultTarget(&self, default_target: Option<&NSAppleEventDescriptor>);

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSAttributedString"
        ))]
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
