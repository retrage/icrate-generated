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
    pub struct DOMImplementation;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMImplementation {
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
unsafe impl NSCopying for DOMImplementation {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMImplementation {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[cfg(feature = "Foundation_NSString")]
        #[method(hasFeature:version:)]
        pub unsafe fn hasFeature_version(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMDocumentType",
            feature = "WebKit_DOMNode"
        ))]
        #[method_id(@__retain_semantics Other createDocumentType:publicId:systemId:)]
        pub unsafe fn createDocumentType_publicId_systemId(
            &self,
            qualified_name: Option<&NSString>,
            public_id: Option<&NSString>,
            system_id: Option<&NSString>,
        ) -> Option<Id<DOMDocumentType>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMDocument",
            feature = "WebKit_DOMDocumentType",
            feature = "WebKit_DOMNode"
        ))]
        #[method_id(@__retain_semantics Other createDocument:qualifiedName:doctype:)]
        pub unsafe fn createDocument_qualifiedName_doctype(
            &self,
            namespace_uri: Option<&NSString>,
            qualified_name: Option<&NSString>,
            doctype: Option<&DOMDocumentType>,
        ) -> Option<Id<DOMDocument>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMCSSStyleSheet",
            feature = "WebKit_DOMStyleSheet"
        ))]
        #[method_id(@__retain_semantics Other createCSSStyleSheet:media:)]
        pub unsafe fn createCSSStyleSheet_media(
            &self,
            title: Option<&NSString>,
            media: Option<&NSString>,
        ) -> Option<Id<DOMCSSStyleSheet>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMDocument",
            feature = "WebKit_DOMHTMLDocument",
            feature = "WebKit_DOMNode"
        ))]
        #[method_id(@__retain_semantics Other createHTMLDocument:)]
        pub unsafe fn createHTMLDocument(
            &self,
            title: Option<&NSString>,
        ) -> Option<Id<DOMHTMLDocument>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// DOMImplementationDeprecated
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(hasFeature::)]
        pub unsafe fn hasFeature(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMDocumentType",
            feature = "WebKit_DOMNode"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other createDocumentType:::)]
        pub unsafe fn createDocumentType(
            &self,
            qualified_name: Option<&NSString>,
            public_id: Option<&NSString>,
            system_id: Option<&NSString>,
        ) -> Option<Id<DOMDocumentType>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMDocument",
            feature = "WebKit_DOMDocumentType",
            feature = "WebKit_DOMNode"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other createDocument:::)]
        pub unsafe fn createDocument(
            &self,
            namespace_uri: Option<&NSString>,
            qualified_name: Option<&NSString>,
            doctype: Option<&DOMDocumentType>,
        ) -> Option<Id<DOMDocument>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMCSSStyleSheet",
            feature = "WebKit_DOMStyleSheet"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other createCSSStyleSheet::)]
        pub unsafe fn createCSSStyleSheet(
            &self,
            title: Option<&NSString>,
            media: Option<&NSString>,
        ) -> Option<Id<DOMCSSStyleSheet>>;
    }
);
