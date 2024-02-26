//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    #[deprecated]
    pub struct DOMCounter;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMCounter {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMCounter {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMCounter {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMCounter {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other listStyle)]
        pub unsafe fn listStyle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other separator)]
        pub unsafe fn separator(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMCounter {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMCounter {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
