//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLFormElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLFormElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLFormElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLFormElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLFormElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLFormElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other acceptCharset)]
        pub unsafe fn acceptCharset(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setAcceptCharset:)]
        pub unsafe fn setAcceptCharset(&self, accept_charset: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other action)]
        pub unsafe fn action(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other enctype)]
        pub unsafe fn enctype(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setEnctype:)]
        pub unsafe fn setEnctype(&self, enctype: Option<&NSString>);

        #[method_id(@__retain_semantics Other encoding)]
        pub unsafe fn encoding(&self) -> Id<NSString>;

        #[method(setEncoding:)]
        pub unsafe fn setEncoding(&self, encoding: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other method)]
        pub unsafe fn method(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setMethod:)]
        pub unsafe fn setMethod(&self, method: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSString>);

        #[cfg(feature = "DOMHTMLCollection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other elements)]
        pub unsafe fn elements(&self) -> Option<Id<DOMHTMLCollection>>;

        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_int;

        #[deprecated]
        #[method(submit)]
        pub unsafe fn submit(&self);

        #[deprecated]
        #[method(reset)]
        pub unsafe fn reset(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLFormElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLFormElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
