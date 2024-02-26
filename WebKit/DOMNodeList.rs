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
    pub struct DOMNodeList;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMNodeList {
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
unsafe impl NSCopying for DOMNodeList {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMNodeList {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNodeList {
        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNodeList {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNodeList {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
