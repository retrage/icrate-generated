//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebResourceLoadDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:identifierForInitialRequest:fromDataSource:)]
        unsafe fn webView_identifierForInitialRequest_fromDataSource(
            &self,
            sender: Option<&WebView>,
            request: Option<&NSURLRequest>,
            data_source: Option<&WebDataSource>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:resource:willSendRequest:redirectResponse:fromDataSource:)]
        unsafe fn webView_resource_willSendRequest_redirectResponse_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            request: Option<&NSURLRequest>,
            redirect_response: Option<&NSURLResponse>,
            data_source: Option<&WebDataSource>,
        ) -> Option<Id<NSURLRequest>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didReceiveAuthenticationChallenge:fromDataSource:)]
        unsafe fn webView_resource_didReceiveAuthenticationChallenge_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            challenge: Option<&NSURLAuthenticationChallenge>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didCancelAuthenticationChallenge:fromDataSource:)]
        unsafe fn webView_resource_didCancelAuthenticationChallenge_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            challenge: Option<&NSURLAuthenticationChallenge>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSURLResponse",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didReceiveResponse:fromDataSource:)]
        unsafe fn webView_resource_didReceiveResponse_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            response: Option<&NSURLResponse>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didReceiveContentLength:fromDataSource:)]
        unsafe fn webView_resource_didReceiveContentLength_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            length: NSInteger,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didFinishLoadingFromDataSource:)]
        unsafe fn webView_resource_didFinishLoadingFromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSError",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:resource:didFailLoadingWithError:fromDataSource:)]
        unsafe fn webView_resource_didFailLoadingWithError_fromDataSource(
            &self,
            sender: Option<&WebView>,
            identifier: Option<&AnyObject>,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSError",
            feature = "WebKit_WebDataSource",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:plugInFailedWithError:dataSource:)]
        unsafe fn webView_plugInFailedWithError_dataSource(
            &self,
            sender: Option<&WebView>,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );
    }

    unsafe impl ProtocolType for dyn WebResourceLoadDelegate {}
);
