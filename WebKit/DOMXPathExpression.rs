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
    pub struct DOMXPathExpression;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMXPathExpression {
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
unsafe impl NSCopying for DOMXPathExpression {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMXPathExpression {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[cfg(all(feature = "WebKit_DOMNode", feature = "WebKit_DOMXPathResult"))]
        #[method_id(@__retain_semantics Other evaluate:type:inResult:)]
        pub unsafe fn evaluate_type_inResult(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Id<DOMXPathResult>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// DOMXPathExpressionDeprecated
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMXPathExpression {
        #[cfg(all(feature = "WebKit_DOMNode", feature = "WebKit_DOMXPathResult"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other evaluate:::)]
        pub unsafe fn evaluate(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Id<DOMXPathResult>>;
    }
);
