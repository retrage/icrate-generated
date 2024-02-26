//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSScripting
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSWindow"))]
    unsafe impl NSWindow {
        #[method(hasCloseBox)]
        pub fn hasCloseBox(&self) -> bool;

        #[method(hasTitleBar)]
        pub unsafe fn hasTitleBar(&self) -> bool;

        #[method(isFloatingPanel)]
        pub unsafe fn isFloatingPanel(&self) -> bool;

        #[method(isMiniaturizable)]
        pub fn isMiniaturizable(&self) -> bool;

        #[method(isModalPanel)]
        pub unsafe fn isModalPanel(&self) -> bool;

        #[method(isResizable)]
        pub fn isResizable(&self) -> bool;

        #[method(isZoomable)]
        pub unsafe fn isZoomable(&self) -> bool;

        #[method(orderedIndex)]
        pub unsafe fn orderedIndex(&self) -> NSInteger;

        #[method(setOrderedIndex:)]
        pub unsafe fn setOrderedIndex(&self, ordered_index: NSInteger);

        #[method(setIsMiniaturized:)]
        pub unsafe fn setIsMiniaturized(&self, flag: bool);

        #[method(setIsVisible:)]
        pub unsafe fn setIsVisible(&self, flag: bool);

        #[method(setIsZoomed:)]
        pub unsafe fn setIsZoomed(&self, flag: bool);

        #[cfg(all(
            feature = "Foundation_NSScriptCommand",
            feature = "Foundation_NSScriptStandardSuiteCommands"
        ))]
        #[method_id(@__retain_semantics Other handleCloseScriptCommand:)]
        pub unsafe fn handleCloseScriptCommand(
            &self,
            command: &NSCloseCommand,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSScriptCommand")]
        #[method_id(@__retain_semantics Other handlePrintScriptCommand:)]
        pub unsafe fn handlePrintScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSScriptCommand")]
        #[method_id(@__retain_semantics Other handleSaveScriptCommand:)]
        pub unsafe fn handleSaveScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<AnyObject>>;
    }
);
