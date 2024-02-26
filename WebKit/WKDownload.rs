//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKDownload;

    unsafe impl ClassType for WKDownload {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKDownload {}

#[cfg(feature = "Foundation_NSProgress")]
unsafe impl NSProgressReporting for WKDownload {}

extern_methods!(
    unsafe impl WKDownload {
        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other originalRequest)]
        pub unsafe fn originalRequest(&self) -> Option<Id<NSURLRequest>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WKWebView"
        ))]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self, mtm: MainThreadMarker) -> Option<Id<WKWebView>>;

        #[cfg(feature = "WebKit_WKDownloadDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn WKDownloadDelegate>>>;

        #[cfg(feature = "WebKit_WKDownloadDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn WKDownloadDelegate>>);

        #[cfg(feature = "Foundation_NSData")]
        #[method(cancel:)]
        pub unsafe fn cancel(&self, completion_handler: Option<&Block<dyn Fn(*mut NSData)>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKDownload {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
