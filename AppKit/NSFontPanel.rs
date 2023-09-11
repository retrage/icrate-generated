//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFontPanelModeMask {
        NSFontPanelModeMaskFace = 1 << 0,
        NSFontPanelModeMaskSize = 1 << 1,
        NSFontPanelModeMaskCollection = 1 << 2,
        NSFontPanelModeMaskUnderlineEffect = 1 << 8,
        NSFontPanelModeMaskStrikethroughEffect = 1 << 9,
        NSFontPanelModeMaskTextColorEffect = 1 << 10,
        NSFontPanelModeMaskDocumentColorEffect = 1 << 11,
        NSFontPanelModeMaskShadowEffect = 1 << 12,
        NSFontPanelModeMaskAllEffects = 0xFFF00,
        NSFontPanelModesMaskStandardModes = 0xFFFF,
        NSFontPanelModesMaskAllModes = 0xFFFFFFFF,
    }
);

extern_protocol!(
    pub unsafe trait NSFontChanging: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "AppKit_NSFontManager")]
        #[optional]
        #[method(changeFont:)]
        unsafe fn changeFont(&self, sender: Option<&NSFontManager>);

        #[cfg(feature = "AppKit_NSFontPanel")]
        #[optional]
        #[method(validModesForFontPanel:)]
        unsafe fn validModesForFontPanel(&self, font_panel: &NSFontPanel) -> NSFontPanelModeMask;
    }

    unsafe impl ProtocolType for dyn NSFontChanging {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFontPanel")]
    pub struct NSFontPanel;

    #[cfg(feature = "AppKit_NSFontPanel")]
    unsafe impl ClassType for NSFontPanel {
        #[inherits(NSWindow, NSResponder, NSObject)]
        type Super = NSPanel;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSAccessibility for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSAccessibilityElementProtocol for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSAnimatablePropertyContainer for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSAppearanceCustomization for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSCoding for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSMenuItemValidation for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSObjectProtocol for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSUserInterfaceItemIdentification for NSFontPanel {}

#[cfg(feature = "AppKit_NSFontPanel")]
unsafe impl NSUserInterfaceValidations for NSFontPanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFontPanel")]
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics Other sharedFontPanel)]
        pub unsafe fn sharedFontPanel(mtm: MainThreadMarker) -> Id<NSFontPanel>;

        #[method(sharedFontPanelExists)]
        pub unsafe fn sharedFontPanelExists(mtm: MainThreadMarker) -> bool;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setPanelFont:isMultiple:)]
        pub unsafe fn setPanelFont_isMultiple(&self, font_obj: &NSFont, flag: bool);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other panelConvertFont:)]
        pub unsafe fn panelConvertFont(&self, font_obj: &NSFont) -> Id<NSFont>;

        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;

        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, works_when_modal: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(reloadDefaultFontFamilies)]
        pub unsafe fn reloadDefaultFontFamilies(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(feature = "AppKit_NSFontPanel")]
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Option<Allocated<Self>>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScreen")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Option<Allocated<Self>>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSFontPanel")]
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSFontPanel")]
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSFontPanelFaceModeMask = 1 << 0,
        NSFontPanelSizeModeMask = 1 << 1,
        NSFontPanelCollectionModeMask = 1 << 2,
        NSFontPanelUnderlineEffectModeMask = 1 << 8,
        NSFontPanelStrikethroughEffectModeMask = 1 << 9,
        NSFontPanelTextColorEffectModeMask = 1 << 10,
        NSFontPanelDocumentColorEffectModeMask = 1 << 11,
        NSFontPanelShadowEffectModeMask = 1 << 12,
        NSFontPanelAllEffectsModeMask = 0xFFF00,
        NSFontPanelStandardModesMask = 0xFFFF,
        NSFontPanelAllModesMask = 0xFFFFFFFF,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        NSFPPreviewButton = 131,
        NSFPRevertButton = 130,
        NSFPSetButton = 132,
        NSFPPreviewField = 128,
        NSFPSizeField = 129,
        NSFPSizeTitle = 133,
        NSFPCurrentField = 134,
    }
);
