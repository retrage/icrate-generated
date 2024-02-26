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
    pub struct DOMHTMLOptionsCollection;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMHTMLOptionsCollection {
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
unsafe impl NSCopying for DOMHTMLOptionsCollection {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMHTMLOptionsCollection {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMHTMLOptionsCollection {
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> c_int;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: c_int);

        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[deprecated]
        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: c_uint);

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other namedItem:)]
        pub unsafe fn namedItem(&self, name: Option<&NSString>) -> Option<Id<DOMNode>>;

        #[cfg(all(
            feature = "WebKit_DOMElement",
            feature = "WebKit_DOMHTMLElement",
            feature = "WebKit_DOMHTMLOptionElement",
            feature = "WebKit_DOMNode"
        ))]
        #[method(add:index:)]
        pub unsafe fn add_index(&self, option: Option<&DOMHTMLOptionElement>, index: c_uint);

        #[method(remove:)]
        pub unsafe fn remove(&self, index: c_uint);

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMHTMLOptionsCollection {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMHTMLOptionsCollection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
