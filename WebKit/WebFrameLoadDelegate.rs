//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebFrameLoadDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didStartProvisionalLoadForFrame:)]
        unsafe fn webView_didStartProvisionalLoadForFrame(
            &self,
            sender: Option<&WebView>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didReceiveServerRedirectForProvisionalLoadForFrame:)]
        unsafe fn webView_didReceiveServerRedirectForProvisionalLoadForFrame(
            &self,
            sender: Option<&WebView>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSError",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didFailProvisionalLoadWithError:forFrame:)]
        unsafe fn webView_didFailProvisionalLoadWithError_forFrame(
            &self,
            sender: Option<&WebView>,
            error: Option<&NSError>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didCommitLoadForFrame:)]
        unsafe fn webView_didCommitLoadForFrame(
            &self,
            sender: Option<&WebView>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didReceiveTitle:forFrame:)]
        unsafe fn webView_didReceiveTitle_forFrame(
            &self,
            sender: Option<&WebView>,
            title: Option<&NSString>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didReceiveIcon:forFrame:)]
        unsafe fn webView_didReceiveIcon_forFrame(
            &self,
            sender: Option<&WebView>,
            image: Option<&NSImage>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didFinishLoadForFrame:)]
        unsafe fn webView_didFinishLoadForFrame(
            &self,
            sender: Option<&WebView>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSError",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didFailLoadWithError:forFrame:)]
        unsafe fn webView_didFailLoadWithError_forFrame(
            &self,
            sender: Option<&WebView>,
            error: Option<&NSError>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didChangeLocationWithinPageForFrame:)]
        unsafe fn webView_didChangeLocationWithinPageForFrame(
            &self,
            sender: Option<&WebView>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSDate",
            feature = "Foundation_NSURL",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:willPerformClientRedirectToURL:delay:fireDate:forFrame:)]
        unsafe fn webView_willPerformClientRedirectToURL_delay_fireDate_forFrame(
            &self,
            sender: Option<&WebView>,
            url: Option<&NSURL>,
            seconds: NSTimeInterval,
            date: Option<&NSDate>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didCancelClientRedirectForFrame:)]
        unsafe fn webView_didCancelClientRedirectForFrame(
            &self,
            sender: Option<&WebView>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:willCloseFrame:)]
        unsafe fn webView_willCloseFrame(&self, sender: Option<&WebView>, frame: Option<&WebFrame>);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebScriptObject",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:didClearWindowObject:forFrame:)]
        unsafe fn webView_didClearWindowObject_forFrame(
            &self,
            web_view: Option<&WebView>,
            window_object: Option<&WebScriptObject>,
            frame: Option<&WebFrame>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebScriptObject",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:windowScriptObjectAvailable:)]
        unsafe fn webView_windowScriptObjectAvailable(
            &self,
            web_view: Option<&WebView>,
            window_script_object: Option<&WebScriptObject>,
        );
    }

    unsafe impl ProtocolType for dyn WebFrameLoadDelegate {}
);
