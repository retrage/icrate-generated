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
    pub struct DOMTreeWalker;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMTreeWalker {
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
unsafe impl NSCopying for DOMTreeWalker {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMTreeWalker {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMTreeWalker {
        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other root)]
        pub unsafe fn root(&self) -> Option<Id<DOMNode>>;

        #[deprecated]
        #[method(whatToShow)]
        pub unsafe fn whatToShow(&self) -> c_uint;

        #[cfg(feature = "WebKit_DOMNodeFilter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Id<ProtocolObject<dyn DOMNodeFilter>>>;

        #[deprecated]
        #[method(expandEntityReferences)]
        pub unsafe fn expandEntityReferences(&self) -> bool;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other currentNode)]
        pub unsafe fn currentNode(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method(setCurrentNode:)]
        pub unsafe fn setCurrentNode(&self, current_node: Option<&DOMNode>);

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other firstChild)]
        pub unsafe fn firstChild(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other lastChild)]
        pub unsafe fn lastChild(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousSibling)]
        pub unsafe fn previousSibling(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other nextSibling)]
        pub unsafe fn nextSibling(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Id<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMTreeWalker {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMTreeWalker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
