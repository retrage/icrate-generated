//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMDocumentFragment;

    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl ClassType for DOMDocumentFragment {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "WebKit_DOMEventTarget",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMDocumentFragment {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMDocumentFragment {}

#[cfg(all(
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMDocumentFragment {}

extern_methods!(
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMDocumentFragment {}
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMDocumentFragment {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMDocumentFragment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
