//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationAppleIDButtonType(pub NSInteger);
impl ASAuthorizationAppleIDButtonType {
    #[doc(alias = "ASAuthorizationAppleIDButtonTypeSignIn")]
    pub const SignIn: Self = Self(0);
    #[doc(alias = "ASAuthorizationAppleIDButtonTypeContinue")]
    pub const Continue: Self = Self(1);
    #[doc(alias = "ASAuthorizationAppleIDButtonTypeSignUp")]
    pub const SignUp: Self = Self(2);
    #[doc(alias = "ASAuthorizationAppleIDButtonTypeDefault")]
    pub const Default: Self = Self(ASAuthorizationAppleIDButtonType::SignIn.0);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ASAuthorizationAppleIDButtonType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ASAuthorizationAppleIDButtonType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationAppleIDButtonStyle(pub NSInteger);
impl ASAuthorizationAppleIDButtonStyle {
    #[doc(alias = "ASAuthorizationAppleIDButtonStyleWhite")]
    pub const White: Self = Self(0);
    #[doc(alias = "ASAuthorizationAppleIDButtonStyleWhiteOutline")]
    pub const WhiteOutline: Self = Self(1);
    #[doc(alias = "ASAuthorizationAppleIDButtonStyleBlack")]
    pub const Black: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ASAuthorizationAppleIDButtonStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ASAuthorizationAppleIDButtonStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityButton for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSObjectProtocol for ASAuthorizationAppleIDButton {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for ASAuthorizationAppleIDButton {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ASAuthorizationAppleIDButton {
        #[method_id(@__retain_semantics Other buttonWithType:style:)]
        pub unsafe fn buttonWithType_style(
            r#type: ASAuthorizationAppleIDButtonType,
            style: ASAuthorizationAppleIDButtonStyle,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithAuthorizationButtonType:authorizationButtonStyle:)]
        pub unsafe fn initWithAuthorizationButtonType_authorizationButtonStyle(
            this: Allocated<Self>,
            r#type: ASAuthorizationAppleIDButtonType,
            style: ASAuthorizationAppleIDButtonStyle,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ASAuthorizationAppleIDButton {
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
    unsafe impl ASAuthorizationAppleIDButton {
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
    unsafe impl ASAuthorizationAppleIDButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
