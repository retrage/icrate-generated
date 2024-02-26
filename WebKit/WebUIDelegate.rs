//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[deprecated]
pub const WebMenuItemTagOpenLinkInNewWindow: c_uint = 1;
#[deprecated]
pub const WebMenuItemTagDownloadLinkToDisk: c_uint = 2;
#[deprecated]
pub const WebMenuItemTagCopyLinkToClipboard: c_uint = 3;
#[deprecated]
pub const WebMenuItemTagOpenImageInNewWindow: c_uint = 4;
#[deprecated]
pub const WebMenuItemTagDownloadImageToDisk: c_uint = 5;
#[deprecated]
pub const WebMenuItemTagCopyImageToClipboard: c_uint = 6;
#[deprecated]
pub const WebMenuItemTagOpenFrameInNewWindow: c_uint = 7;
#[deprecated]
pub const WebMenuItemTagCopy: c_uint = 8;
#[deprecated]
pub const WebMenuItemTagGoBack: c_uint = 9;
#[deprecated]
pub const WebMenuItemTagGoForward: c_uint = 10;
#[deprecated]
pub const WebMenuItemTagStop: c_uint = 11;
#[deprecated]
pub const WebMenuItemTagReload: c_uint = 12;
#[deprecated]
pub const WebMenuItemTagCut: c_uint = 13;
#[deprecated]
pub const WebMenuItemTagPaste: c_uint = 14;
#[deprecated]
pub const WebMenuItemTagSpellingGuess: c_uint = 15;
#[deprecated]
pub const WebMenuItemTagNoGuessesFound: c_uint = 16;
#[deprecated]
pub const WebMenuItemTagIgnoreSpelling: c_uint = 17;
#[deprecated]
pub const WebMenuItemTagLearnSpelling: c_uint = 18;
#[deprecated]
pub const WebMenuItemTagOther: c_uint = 19;
#[deprecated]
pub const WebMenuItemTagSearchInSpotlight: c_uint = 20;
#[deprecated]
pub const WebMenuItemTagSearchWeb: c_uint = 21;
#[deprecated]
pub const WebMenuItemTagLookUpInDictionary: c_uint = 22;
#[deprecated]
pub const WebMenuItemTagOpenWithDefaultApplication: c_uint = 23;
#[deprecated]
pub const WebMenuItemPDFActualSize: c_uint = 24;
#[deprecated]
pub const WebMenuItemPDFZoomIn: c_uint = 25;
#[deprecated]
pub const WebMenuItemPDFZoomOut: c_uint = 26;
#[deprecated]
pub const WebMenuItemPDFAutoSize: c_uint = 27;
#[deprecated]
pub const WebMenuItemPDFSinglePage: c_uint = 28;
#[deprecated]
pub const WebMenuItemPDFFacingPages: c_uint = 29;
#[deprecated]
pub const WebMenuItemPDFContinuous: c_uint = 30;
#[deprecated]
pub const WebMenuItemPDFNextPage: c_uint = 31;
#[deprecated]
pub const WebMenuItemPDFPreviousPage: c_uint = 32;

ns_options!(
    #[underlying(NSUInteger)]
    #[deprecated]
    pub enum WebDragDestinationAction {
        #[deprecated]
        #[doc(alias = "WebDragDestinationActionNone")]
        None = 0,
        #[deprecated]
        #[doc(alias = "WebDragDestinationActionDHTML")]
        DHTML = 1,
        #[deprecated]
        #[doc(alias = "WebDragDestinationActionEdit")]
        Edit = 2,
        #[deprecated]
        #[doc(alias = "WebDragDestinationActionLoad")]
        Load = 4,
        #[deprecated]
        #[doc(alias = "WebDragDestinationActionAny")]
        Any = 4294967295,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    #[deprecated]
    pub enum WebDragSourceAction {
        #[deprecated]
        #[doc(alias = "WebDragSourceActionNone")]
        None = 0,
        #[deprecated]
        #[doc(alias = "WebDragSourceActionDHTML")]
        DHTML = 1,
        #[deprecated]
        #[doc(alias = "WebDragSourceActionImage")]
        Image = 2,
        #[deprecated]
        #[doc(alias = "WebDragSourceActionLink")]
        Link = 4,
        #[deprecated]
        #[doc(alias = "WebDragSourceActionSelection")]
        Selection = 8,
        #[deprecated]
        #[doc(alias = "WebDragSourceActionAny")]
        Any = 4294967295,
    }
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebOpenPanelResultListener: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(chooseFilename:)]
        unsafe fn chooseFilename(&self, file_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(chooseFilenames:)]
        unsafe fn chooseFilenames(&self, file_names: Option<&NSArray>);

        #[deprecated]
        #[method(cancel)]
        unsafe fn cancel(&self);
    }

    unsafe impl ProtocolType for dyn WebOpenPanelResultListener {}
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebUIDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:createWebViewWithRequest:)]
        unsafe fn webView_createWebViewWithRequest(
            &self,
            sender: Option<&WebView>,
            request: Option<&NSURLRequest>,
            mtm: MainThreadMarker,
        ) -> Option<Id<WebView>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewShow:)]
        unsafe fn webViewShow(&self, sender: Option<&WebView>);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:createWebViewModalDialogWithRequest:)]
        unsafe fn webView_createWebViewModalDialogWithRequest(
            &self,
            sender: Option<&WebView>,
            request: Option<&NSURLRequest>,
            mtm: MainThreadMarker,
        ) -> Option<Id<WebView>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewRunModal:)]
        unsafe fn webViewRunModal(&self, sender: Option<&WebView>);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewClose:)]
        unsafe fn webViewClose(&self, sender: Option<&WebView>);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewFocus:)]
        unsafe fn webViewFocus(&self, sender: Option<&WebView>);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewUnfocus:)]
        unsafe fn webViewUnfocus(&self, sender: Option<&WebView>);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webViewFirstResponder:)]
        unsafe fn webViewFirstResponder(
            &self,
            sender: Option<&WebView>,
            mtm: MainThreadMarker,
        ) -> Option<Id<NSResponder>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:makeFirstResponder:)]
        unsafe fn webView_makeFirstResponder(
            &self,
            sender: Option<&WebView>,
            responder: Option<&NSResponder>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:setStatusText:)]
        unsafe fn webView_setStatusText(&self, sender: Option<&WebView>, text: Option<&NSString>);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webViewStatusText:)]
        unsafe fn webViewStatusText(&self, sender: Option<&WebView>) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewAreToolbarsVisible:)]
        unsafe fn webViewAreToolbarsVisible(&self, sender: Option<&WebView>) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:setToolbarsVisible:)]
        unsafe fn webView_setToolbarsVisible(&self, sender: Option<&WebView>, visible: bool);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewIsStatusBarVisible:)]
        unsafe fn webViewIsStatusBarVisible(&self, sender: Option<&WebView>) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:setStatusBarVisible:)]
        unsafe fn webView_setStatusBarVisible(&self, sender: Option<&WebView>, visible: bool);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewIsResizable:)]
        unsafe fn webViewIsResizable(&self, sender: Option<&WebView>) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:setResizable:)]
        unsafe fn webView_setResizable(&self, sender: Option<&WebView>, resizable: bool);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:setFrame:)]
        unsafe fn webView_setFrame(&self, sender: Option<&WebView>, frame: NSRect);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewFrame:)]
        unsafe fn webViewFrame(&self, sender: Option<&WebView>) -> NSRect;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:runJavaScriptAlertPanelWithMessage:initiatedByFrame:)]
        unsafe fn webView_runJavaScriptAlertPanelWithMessage_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
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
        #[method(webView:runJavaScriptConfirmPanelWithMessage:initiatedByFrame:)]
        unsafe fn webView_runJavaScriptConfirmPanelWithMessage_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
            frame: Option<&WebFrame>,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:runJavaScriptTextInputPanelWithPrompt:defaultText:initiatedByFrame:)]
        unsafe fn webView_runJavaScriptTextInputPanelWithPrompt_defaultText_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            prompt: Option<&NSString>,
            default_text: Option<&NSString>,
            frame: Option<&WebFrame>,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:runBeforeUnloadConfirmPanelWithMessage:initiatedByFrame:)]
        unsafe fn webView_runBeforeUnloadConfirmPanelWithMessage_initiatedByFrame(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
            frame: Option<&WebFrame>,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:runOpenPanelForFileButtonWithResultListener:)]
        unsafe fn webView_runOpenPanelForFileButtonWithResultListener(
            &self,
            sender: Option<&WebView>,
            result_listener: Option<&ProtocolObject<dyn WebOpenPanelResultListener>>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:runOpenPanelForFileButtonWithResultListener:allowMultipleFiles:)]
        unsafe fn webView_runOpenPanelForFileButtonWithResultListener_allowMultipleFiles(
            &self,
            sender: Option<&WebView>,
            result_listener: Option<&ProtocolObject<dyn WebOpenPanelResultListener>>,
            allow_multiple_files: bool,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSDictionary",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:mouseDidMoveOverElement:modifierFlags:)]
        unsafe fn webView_mouseDidMoveOverElement_modifierFlags(
            &self,
            sender: Option<&WebView>,
            element_information: Option<&NSDictionary>,
            modifier_flags: NSUInteger,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:contextMenuItemsForElement:defaultMenuItems:)]
        unsafe fn webView_contextMenuItemsForElement_defaultMenuItems(
            &self,
            sender: Option<&WebView>,
            element: Option<&NSDictionary>,
            default_menu_items: Option<&NSArray>,
        ) -> Option<Id<NSArray>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSUserInterfaceValidation",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:validateUserInterfaceItem:defaultValidation:)]
        unsafe fn webView_validateUserInterfaceItem_defaultValidation(
            &self,
            web_view: Option<&WebView>,
            item: Option<&ProtocolObject<dyn NSValidatedUserInterfaceItem>>,
            default_validation: bool,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:shouldPerformAction:fromSender:)]
        unsafe fn webView_shouldPerformAction_fromSender(
            &self,
            web_view: Option<&WebView>,
            action: Option<Sel>,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSDragging",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:dragDestinationActionMaskForDraggingInfo:)]
        unsafe fn webView_dragDestinationActionMaskForDraggingInfo(
            &self,
            web_view: Option<&WebView>,
            dragging_info: Option<&ProtocolObject<dyn NSDraggingInfo>>,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "AppKit_NSDragging",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:willPerformDragDestinationAction:forDraggingInfo:)]
        unsafe fn webView_willPerformDragDestinationAction_forDraggingInfo(
            &self,
            web_view: Option<&WebView>,
            action: WebDragDestinationAction,
            dragging_info: Option<&ProtocolObject<dyn NSDraggingInfo>>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:dragSourceActionMaskForPoint:)]
        unsafe fn webView_dragSourceActionMaskForPoint(
            &self,
            web_view: Option<&WebView>,
            point: NSPoint,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:willPerformDragSourceAction:fromPoint:withPasteboard:)]
        unsafe fn webView_willPerformDragSourceAction_fromPoint_withPasteboard(
            &self,
            web_view: Option<&WebView>,
            action: WebDragSourceAction,
            point: NSPoint,
            pasteboard: Option<&NSPasteboard>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebFrameView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:printFrameView:)]
        unsafe fn webView_printFrameView(
            &self,
            sender: Option<&WebView>,
            frame_view: Option<&WebFrameView>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewHeaderHeight:)]
        unsafe fn webViewHeaderHeight(&self, sender: Option<&WebView>) -> c_float;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewFooterHeight:)]
        unsafe fn webViewFooterHeight(&self, sender: Option<&WebView>) -> c_float;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:drawHeaderInRect:)]
        unsafe fn webView_drawHeaderInRect(&self, sender: Option<&WebView>, rect: NSRect);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:drawFooterInRect:)]
        unsafe fn webView_drawFooterInRect(&self, sender: Option<&WebView>, rect: NSRect);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:runJavaScriptAlertPanelWithMessage:)]
        unsafe fn webView_runJavaScriptAlertPanelWithMessage(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:runJavaScriptConfirmPanelWithMessage:)]
        unsafe fn webView_runJavaScriptConfirmPanelWithMessage(
            &self,
            sender: Option<&WebView>,
            message: Option<&NSString>,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other webView:runJavaScriptTextInputPanelWithPrompt:defaultText:)]
        unsafe fn webView_runJavaScriptTextInputPanelWithPrompt_defaultText(
            &self,
            sender: Option<&WebView>,
            prompt: Option<&NSString>,
            default_text: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webView:setContentRect:)]
        unsafe fn webView_setContentRect(&self, sender: Option<&WebView>, frame: NSRect);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[optional]
        #[method(webViewContentRect:)]
        unsafe fn webViewContentRect(&self, sender: Option<&WebView>) -> NSRect;
    }

    unsafe impl ProtocolType for dyn WebUIDelegate {}
);
