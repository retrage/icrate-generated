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
    pub struct DOMNamedNodeMap;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMNamedNodeMap {
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
unsafe impl NSCopying for DOMNamedNodeMap {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMNamedNodeMap {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other getNamedItem:)]
        pub unsafe fn getNamedItem(&self, name: Option<&NSString>) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other setNamedItem:)]
        pub unsafe fn setNamedItem(&self, node: Option<&DOMNode>) -> Option<Id<DOMNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other removeNamedItem:)]
        pub unsafe fn removeNamedItem(&self, name: Option<&NSString>) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[method_id(@__retain_semantics Other getNamedItemNS:localName:)]
        pub unsafe fn getNamedItemNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other setNamedItemNS:)]
        pub unsafe fn setNamedItemNS(&self, node: Option<&DOMNode>) -> Option<Id<DOMNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[method_id(@__retain_semantics Other removeNamedItemNS:localName:)]
        pub unsafe fn removeNamedItemNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// DOMNamedNodeMapDeprecated
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other getNamedItemNS::)]
        pub unsafe fn getNamedItemNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other removeNamedItemNS::)]
        pub unsafe fn removeNamedItemNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Id<DOMNode>>;
    }
);
