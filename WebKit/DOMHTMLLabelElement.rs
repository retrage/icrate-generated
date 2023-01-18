//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLLabelElement")]
    #[deprecated]
    pub struct DOMHTMLLabelElement;

    #[cfg(feature = "WebKit_DOMHTMLLabelElement")]
    unsafe impl ClassType for DOMHTMLLabelElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLLabelElement")]
    unsafe impl DOMHTMLLabelElement {
        #[cfg(feature = "WebKit_DOMHTMLFormElement")]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Id<DOMHTMLFormElement, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other htmlFor)]
        pub unsafe fn htmlFor(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHtmlFor:)]
        pub unsafe fn setHtmlFor(&self, html_for: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);
    }
);