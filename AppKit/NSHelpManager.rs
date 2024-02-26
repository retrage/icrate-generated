//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
pub type NSHelpBookName = NSString;

#[cfg(feature = "Foundation_NSString")]
pub type NSHelpAnchorName = NSString;

#[cfg(feature = "Foundation_NSString")]
pub type NSHelpManagerContextHelpKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHelpManager;

    unsafe impl ClassType for NSHelpManager {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSHelpManager {}

extern_methods!(
    unsafe impl NSHelpManager {
        #[method_id(@__retain_semantics Other sharedHelpManager)]
        pub unsafe fn sharedHelpManager(mtm: MainThreadMarker) -> Id<NSHelpManager>;

        #[method(isContextHelpModeActive)]
        pub unsafe fn isContextHelpModeActive(mtm: MainThreadMarker) -> bool;

        #[method(setContextHelpModeActive:)]
        pub unsafe fn setContextHelpModeActive(
            context_help_mode_active: bool,
            mtm: MainThreadMarker,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setContextHelp:forObject:)]
        pub unsafe fn setContextHelp_forObject(
            &self,
            attr_string: &NSAttributedString,
            object: &AnyObject,
        );

        #[method(removeContextHelpForObject:)]
        pub unsafe fn removeContextHelpForObject(&self, object: &AnyObject);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other contextHelpForObject:)]
        pub unsafe fn contextHelpForObject(
            &self,
            object: &AnyObject,
        ) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(showContextHelpForObject:locationHint:)]
        pub unsafe fn showContextHelpForObject_locationHint(
            &self,
            object: &AnyObject,
            pt: NSPoint,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(openHelpAnchor:inBook:)]
        pub unsafe fn openHelpAnchor_inBook(
            &self,
            anchor: &NSHelpAnchorName,
            book: Option<&NSHelpBookName>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(findString:inBook:)]
        pub unsafe fn findString_inBook(&self, query: &NSString, book: Option<&NSHelpBookName>);

        #[cfg(feature = "Foundation_NSBundle")]
        #[method(registerBooksInBundle:)]
        pub unsafe fn registerBooksInBundle(&self, bundle: &NSBundle) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSHelpManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSContextHelpModeDidActivateNotification: &'static NSNotificationName);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSContextHelpModeDidDeactivateNotification: &'static NSNotificationName);

extern_category!(
    /// Category on [`NSBundle`].
    pub unsafe trait NSBundleHelpExtension {
        #[cfg(all(
            feature = "AppKit_NSHelpManager",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other contextHelpForKey:)]
        unsafe fn contextHelpForKey(
            &self,
            key: &NSHelpManagerContextHelpKey,
        ) -> Option<Id<NSAttributedString>>;
    }

    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundleHelpExtension for NSBundle {}
);

extern_methods!(
    /// NSApplicationHelpExtension
    #[cfg(all(feature = "AppKit_NSApplication", feature = "AppKit_NSResponder"))]
    unsafe impl NSApplication {
        #[method(activateContextHelpMode:)]
        pub unsafe fn activateContextHelpMode(&self, sender: Option<&AnyObject>);

        #[method(showHelp:)]
        pub unsafe fn showHelp(&self, sender: Option<&AnyObject>);
    }
);
