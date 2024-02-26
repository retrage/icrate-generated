//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;
use crate::LocalAuthenticationEmbeddedUI::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    pub struct LAAuthenticationView;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl ClassType for LAAuthenticationView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for LAAuthenticationView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for LAAuthenticationView {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for LAAuthenticationView {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for LAAuthenticationView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for LAAuthenticationView {}

#[cfg(all(
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for LAAuthenticationView {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
unsafe impl NSObjectProtocol for LAAuthenticationView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for LAAuthenticationView {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl LAAuthenticationView {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "LocalAuthentication_LAContext")]
        #[method_id(@__retain_semantics Init initWithContext:)]
        pub unsafe fn initWithContext(this: Allocated<Self>, context: &LAContext) -> Id<Self>;

        #[cfg(all(feature = "AppKit_NSCell", feature = "LocalAuthentication_LAContext"))]
        #[method_id(@__retain_semantics Init initWithContext:controlSize:)]
        pub unsafe fn initWithContext_controlSize(
            this: Allocated<Self>,
            context: &LAContext,
            control_size: NSControlSize,
        ) -> Id<Self>;

        #[cfg(feature = "LocalAuthentication_LAContext")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<LAContext>;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl LAAuthenticationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl LAAuthenticationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
