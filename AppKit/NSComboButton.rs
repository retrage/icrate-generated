//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSComboButtonStyle {
        #[doc(alias = "NSComboButtonStyleSplit")]
        Split = 0,
        #[doc(alias = "NSComboButtonStyleUnified")]
        Unified = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    pub struct NSComboButton;

    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ClassType for NSComboButton {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSComboButton {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSComboButton {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSComboButton {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSComboButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSComboButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSComboButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSObjectProtocol for NSComboButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSComboButton {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSComboButton {
        #[cfg(all(feature = "AppKit_NSMenu", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other comboButtonWithTitle:menu:target:action:)]
        pub unsafe fn comboButtonWithTitle_menu_target_action(
            title: &NSString,
            menu: Option<&NSMenu>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSMenu"))]
        #[method_id(@__retain_semantics Other comboButtonWithImage:menu:target:action:)]
        pub unsafe fn comboButtonWithImage_menu_target_action(
            image: &NSImage,
            menu: Option<&NSMenu>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "AppKit_NSMenu",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other comboButtonWithTitle:image:menu:target:action:)]
        pub unsafe fn comboButtonWithTitle_image_menu_target_action(
            title: &NSString,
            image: &NSImage,
            menu: Option<&NSMenu>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Id<NSMenu>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: &NSMenu);

        #[method(style)]
        pub unsafe fn style(&self) -> NSComboButtonStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSComboButtonStyle);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSComboButton {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSComboButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSComboButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
