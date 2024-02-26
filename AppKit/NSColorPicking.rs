//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSColorPickingDefault: IsMainThreadOnly {
        #[cfg(all(
            feature = "AppKit_NSColorPanel",
            feature = "AppKit_NSPanel",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSWindow"
        ))]
        #[method_id(@__retain_semantics Init initWithPickerMask:colorPanel:)]
        unsafe fn initWithPickerMask_colorPanel(
            this: Allocated<Self>,
            mask: NSUInteger,
            owning_color_panel: &NSColorPanel,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other provideNewButtonImage)]
        unsafe fn provideNewButtonImage(&self) -> Id<NSImage>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSButtonCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSImage"
        ))]
        #[method(insertNewButtonImage:in:)]
        unsafe fn insertNewButtonImage_in(
            &self,
            new_button_image: &NSImage,
            button_cell: &NSButtonCell,
        );

        #[method(viewSizeChanged:)]
        unsafe fn viewSizeChanged(&self, sender: Option<&AnyObject>);

        #[method(alphaControlAddedOrRemoved:)]
        unsafe fn alphaControlAddedOrRemoved(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "AppKit_NSColorList")]
        #[method(attachColorList:)]
        unsafe fn attachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "AppKit_NSColorList")]
        #[method(detachColorList:)]
        unsafe fn detachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "AppKit_NSColorPanel")]
        #[method(setMode:)]
        unsafe fn setMode(&self, mode: NSColorPanelMode);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonToolTip)]
        unsafe fn buttonToolTip(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(minContentSize)]
        unsafe fn minContentSize(&self) -> NSSize;
    }

    unsafe impl ProtocolType for dyn NSColorPickingDefault {}
);

extern_protocol!(
    pub unsafe trait NSColorPickingCustom: NSColorPickingDefault + IsMainThreadOnly {
        #[cfg(feature = "AppKit_NSColorPanel")]
        #[method(supportsMode:)]
        unsafe fn supportsMode(&self, mode: NSColorPanelMode) -> bool;

        #[cfg(feature = "AppKit_NSColorPanel")]
        #[method(currentMode)]
        unsafe fn currentMode(&self) -> NSColorPanelMode;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other provideNewView:)]
        unsafe fn provideNewView(&self, initial_request: bool) -> Id<NSView>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        unsafe fn setColor(&self, new_color: &NSColor);
    }

    unsafe impl ProtocolType for dyn NSColorPickingCustom {}
);
