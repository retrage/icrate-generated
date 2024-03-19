//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSAppearanceName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppearance;

    unsafe impl ClassType for NSAppearance {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSAppearance {}

unsafe impl NSObjectProtocol for NSAppearance {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSAppearance {}

extern_methods!(
    unsafe impl NSAppearance {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSAppearanceName>;

        #[deprecated = "Use -performAsCurrentDrawingAppearance: to temporarily set the drawing appearance, or +currentDrawingAppearance to access the currently drawing appearance."]
        #[method_id(@__retain_semantics Other currentAppearance)]
        pub unsafe fn currentAppearance() -> Option<Id<NSAppearance>>;

        #[deprecated = "Use -performAsCurrentDrawingAppearance: to temporarily set the drawing appearance, or +currentDrawingAppearance to access the currently drawing appearance."]
        #[method(setCurrentAppearance:)]
        pub unsafe fn setCurrentAppearance(current_appearance: Option<&NSAppearance>);

        #[method_id(@__retain_semantics Other currentDrawingAppearance)]
        pub unsafe fn currentDrawingAppearance() -> Id<NSAppearance>;

        #[method(performAsCurrentDrawingAppearance:)]
        pub unsafe fn performAsCurrentDrawingAppearance(&self, block: &Block<dyn Fn() + '_>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other appearanceNamed:)]
        pub fn appearanceNamed(name: &NSAppearanceName) -> Option<Id<NSAppearance>>;

        #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithAppearanceNamed:bundle:)]
        pub unsafe fn initWithAppearanceNamed_bundle(
            this: Allocated<Self>,
            name: &NSAppearanceName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method(allowsVibrancy)]
        pub unsafe fn allowsVibrancy(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other bestMatchFromAppearancesWithNames:)]
        pub fn bestMatchFromAppearancesWithNames(
            &self,
            appearances: &NSArray<NSAppearanceName>,
        ) -> Option<Id<NSAppearanceName>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAppearance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameAqua: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameDarkAqua: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameLightContent: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameVibrantDark: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameVibrantLight: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameAccessibilityHighContrastAqua: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameAccessibilityHighContrastDarkAqua: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameAccessibilityHighContrastVibrantLight: &'static NSAppearanceName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAppearanceNameAccessibilityHighContrastVibrantDark: &'static NSAppearanceName;
}

extern_protocol!(
    pub unsafe trait NSAppearanceCustomization: NSObjectProtocol {
        #[method_id(@__retain_semantics Other appearance)]
        unsafe fn appearance(&self) -> Option<Id<NSAppearance>>;

        #[method(setAppearance:)]
        unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);

        #[method_id(@__retain_semantics Other effectiveAppearance)]
        unsafe fn effectiveAppearance(&self) -> Id<NSAppearance>;
    }

    unsafe impl ProtocolType for dyn NSAppearanceCustomization {}
);
