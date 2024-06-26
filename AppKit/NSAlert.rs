//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAlertStyle(pub NSUInteger);
impl NSAlertStyle {
    #[doc(alias = "NSAlertStyleWarning")]
    pub const Warning: Self = Self(0);
    #[doc(alias = "NSAlertStyleInformational")]
    pub const Informational: Self = Self(1);
    #[doc(alias = "NSAlertStyleCritical")]
    pub const Critical: Self = Self(2);
}

unsafe impl Encode for NSAlertStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAlertStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "NSApplication")]
pub static NSAlertFirstButtonReturn: NSModalResponse = 1000;

#[cfg(feature = "NSApplication")]
pub static NSAlertSecondButtonReturn: NSModalResponse = 1001;

#[cfg(feature = "NSApplication")]
pub static NSAlertThirdButtonReturn: NSModalResponse = 1002;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAlert;

    unsafe impl ClassType for NSAlert {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSAlert {}

extern_methods!(
    unsafe impl NSAlert {
        #[method_id(@__retain_semantics Other alertWithError:)]
        pub unsafe fn alertWithError(error: &NSError, mtm: MainThreadMarker) -> Id<NSAlert>;

        #[method_id(@__retain_semantics Other messageText)]
        pub unsafe fn messageText(&self) -> Id<NSString>;

        #[method(setMessageText:)]
        pub unsafe fn setMessageText(&self, message_text: &NSString);

        #[method_id(@__retain_semantics Other informativeText)]
        pub unsafe fn informativeText(&self) -> Id<NSString>;

        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informative_text: &NSString);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setIcon:)]
        pub unsafe fn setIcon(&self, icon: Option<&NSImage>);

        #[cfg(all(
            feature = "NSButton",
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other addButtonWithTitle:)]
        pub unsafe fn addButtonWithTitle(&self, title: &NSString) -> Id<NSButton>;

        #[cfg(all(
            feature = "NSButton",
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other buttons)]
        pub unsafe fn buttons(&self) -> Id<NSArray<NSButton>>;

        #[method(alertStyle)]
        pub unsafe fn alertStyle(&self) -> NSAlertStyle;

        #[method(setAlertStyle:)]
        pub unsafe fn setAlertStyle(&self, alert_style: NSAlertStyle);

        #[method(showsHelp)]
        pub unsafe fn showsHelp(&self) -> bool;

        #[method(setShowsHelp:)]
        pub unsafe fn setShowsHelp(&self, shows_help: bool);

        #[cfg(feature = "NSHelpManager")]
        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSHelpAnchorName>>;

        #[cfg(feature = "NSHelpManager")]
        #[method(setHelpAnchor:)]
        pub unsafe fn setHelpAnchor(&self, help_anchor: Option<&NSHelpAnchorName>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSAlertDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSAlertDelegate>>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[method(layout)]
        pub unsafe fn layout(&self);

        #[cfg(feature = "NSApplication")]
        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSModalResponse;

        #[method(showsSuppressionButton)]
        pub unsafe fn showsSuppressionButton(&self) -> bool;

        #[method(setShowsSuppressionButton:)]
        pub unsafe fn setShowsSuppressionButton(&self, shows_suppression_button: bool);

        #[cfg(all(
            feature = "NSButton",
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other suppressionButton)]
        pub unsafe fn suppressionButton(&self) -> Option<Id<NSButton>>;

        #[cfg(all(
            feature = "NSApplication",
            feature = "NSResponder",
            feature = "NSWindow",
            feature = "block2"
        ))]
        #[method(beginSheetModalForWindow:completionHandler:)]
        pub unsafe fn beginSheetModalForWindow_completionHandler(
            &self,
            sheet_window: &NSWindow,
            handler: Option<&Block<dyn Fn(NSModalResponse)>>,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Id<NSWindow>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAlert {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSAlertDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(alertShowHelp:)]
        unsafe fn alertShowHelp(&self, alert: &NSAlert) -> bool;
    }

    unsafe impl ProtocolType for dyn NSAlertDelegate {}
);

extern_methods!(
    /// NSAlertDeprecated
    unsafe impl NSAlert {
        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[deprecated = "Use -beginSheetModalForWindow:completionHandler: instead"]
        #[method(beginSheetModalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetModalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );
    }
);

pub static NSWarningAlertStyle: NSAlertStyle = NSAlertStyle(NSAlertStyle::Warning.0);

pub static NSInformationalAlertStyle: NSAlertStyle = NSAlertStyle(NSAlertStyle::Informational.0);

pub static NSCriticalAlertStyle: NSAlertStyle = NSAlertStyle(NSAlertStyle::Critical.0);
