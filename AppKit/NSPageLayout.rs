//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPageLayoutResult {
        #[doc(alias = "NSPageLayoutResultCancelled")]
        Cancelled = 0,
        #[doc(alias = "NSPageLayoutResultChanged")]
        Changed = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPageLayout;

    unsafe impl ClassType for NSPageLayout {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSPageLayout {}

extern_methods!(
    unsafe impl NSPageLayout {
        #[method_id(@__retain_semantics Other pageLayout)]
        pub unsafe fn pageLayout(mtm: MainThreadMarker) -> Id<NSPageLayout>;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
        #[method(addAccessoryController:)]
        pub unsafe fn addAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
        #[method(removeAccessoryController:)]
        pub unsafe fn removeAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSViewController",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other accessoryControllers)]
        pub unsafe fn accessoryControllers(&self) -> Id<NSArray<NSViewController>>;

        #[cfg(all(
            feature = "AppKit_NSPrintInfo",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSWindow"
        ))]
        #[method(beginSheetUsingPrintInfo:onWindow:completionHandler:)]
        pub unsafe fn beginSheetUsingPrintInfo_onWindow_completionHandler(
            &self,
            print_info: &NSPrintInfo,
            parent_window: &NSWindow,
            handler: Option<&Block<dyn Fn(NSPageLayoutResult)>>,
        );

        #[cfg(all(
            feature = "AppKit_NSPrintInfo",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSWindow"
        ))]
        #[deprecated]
        #[method(beginSheetWithPrintInfo:modalForWindow:delegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetWithPrintInfo_modalForWindow_delegate_didEndSelector_contextInfo(
            &self,
            print_info: &NSPrintInfo,
            doc_window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(runModalWithPrintInfo:)]
        pub unsafe fn runModalWithPrintInfo(&self, print_info: &NSPrintInfo) -> NSInteger;

        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Option<Id<NSPrintInfo>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPageLayout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPageLayout {
        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[deprecated]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[deprecated]
        #[method(readPrintInfo)]
        pub unsafe fn readPrintInfo(&self);

        #[deprecated]
        #[method(writePrintInfo)]
        pub unsafe fn writePrintInfo(&self);
    }
);

extern_methods!(
    /// NSPageLayoutPanel
    #[cfg(all(feature = "AppKit_NSApplication", feature = "AppKit_NSResponder"))]
    unsafe impl NSApplication {
        #[method(runPageLayout:)]
        pub unsafe fn runPageLayout(&self, sender: Option<&AnyObject>);
    }
);
