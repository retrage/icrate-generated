//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStringDrawingContext;

    unsafe impl ClassType for NSStringDrawingContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSStringDrawingContext {}

extern_methods!(
    unsafe impl NSStringDrawingContext {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimum_scale_factor: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(actualScaleFactor)]
        pub unsafe fn actualScaleFactor(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(totalBounds)]
        pub unsafe fn totalBounds(&self) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStringDrawingContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_category!(
    /// Category on [`NSString`].
    pub unsafe trait NSStringDrawing {
        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(sizeWithAttributes:)]
        unsafe fn sizeWithAttributes(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> NSSize;

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(drawAtPoint:withAttributes:)]
        unsafe fn drawAtPoint_withAttributes(
            &self,
            point: NSPoint,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(drawInRect:withAttributes:)]
        unsafe fn drawInRect_withAttributes(
            &self,
            rect: NSRect,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );
    }

    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSStringDrawing for NSString {}
);

extern_category!(
    /// Category "NSStringDrawing" on [`NSAttributedString`].
    #[doc(alias = "NSStringDrawing")]
    pub unsafe trait NSAttributedStringNSStringDrawing {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(size)]
        unsafe fn size(&self) -> NSSize;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(drawAtPoint:)]
        unsafe fn drawAtPoint(&self, point: NSPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(drawInRect:)]
        unsafe fn drawInRect(&self, rect: NSRect);
    }

    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSAttributedStringNSStringDrawing for NSAttributedString {}
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSStringDrawingOptions(pub NSInteger);
impl NSStringDrawingOptions {
    pub const NSStringDrawingUsesLineFragmentOrigin: Self = Self(1 << 0);
    pub const NSStringDrawingUsesFontLeading: Self = Self(1 << 1);
    pub const NSStringDrawingUsesDeviceMetrics: Self = Self(1 << 3);
    pub const NSStringDrawingTruncatesLastVisibleLine: Self = Self(1 << 5);
    #[deprecated]
    pub const NSStringDrawingDisableScreenFontSubstitution: Self = Self(1 << 2);
    #[deprecated]
    pub const NSStringDrawingOneShot: Self = Self(1 << 4);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSStringDrawingOptions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSStringDrawingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_category!(
    /// Category "NSExtendedStringDrawing" on [`NSString`].
    #[doc(alias = "NSExtendedStringDrawing")]
    pub unsafe trait NSStringNSExtendedStringDrawing {
        #[cfg(all(
            feature = "AppKit_NSStringDrawing",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(drawWithRect:options:attributes:context:)]
        unsafe fn drawWithRect_options_attributes_context(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            context: Option<&NSStringDrawingContext>,
        );

        #[cfg(all(
            feature = "AppKit_NSStringDrawing",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(boundingRectWithSize:options:attributes:context:)]
        unsafe fn boundingRectWithSize_options_attributes_context(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            context: Option<&NSStringDrawingContext>,
        ) -> NSRect;
    }

    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSStringNSExtendedStringDrawing for NSString {}
);

extern_category!(
    /// Category "NSExtendedStringDrawing" on [`NSAttributedString`].
    #[doc(alias = "NSExtendedStringDrawing")]
    pub unsafe trait NSAttributedStringNSExtendedStringDrawing {
        #[cfg(all(feature = "AppKit_NSStringDrawing", feature = "Foundation_NSGeometry"))]
        #[method(drawWithRect:options:context:)]
        unsafe fn drawWithRect_options_context(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        );

        #[cfg(all(feature = "AppKit_NSStringDrawing", feature = "Foundation_NSGeometry"))]
        #[method(boundingRectWithSize:options:context:)]
        unsafe fn boundingRectWithSize_options_context(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        ) -> NSRect;
    }

    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSAttributedStringNSExtendedStringDrawing for NSAttributedString {}
);

extern_category!(
    /// Category on [`NSString`].
    pub unsafe trait NSStringDrawingDeprecated {
        #[cfg(all(
            feature = "AppKit_NSStringDrawing",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(drawWithRect:options:attributes:)]
        unsafe fn drawWithRect_options_attributes(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[cfg(all(
            feature = "AppKit_NSStringDrawing",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(boundingRectWithSize:options:attributes:)]
        unsafe fn boundingRectWithSize_options_attributes(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> NSRect;
    }

    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSStringDrawingDeprecated for NSString {}
);

extern_category!(
    /// Category "NSStringDrawingDeprecated" on [`NSAttributedString`].
    #[doc(alias = "NSStringDrawingDeprecated")]
    pub unsafe trait NSAttributedStringNSStringDrawingDeprecated {
        #[cfg(all(feature = "AppKit_NSStringDrawing", feature = "Foundation_NSGeometry"))]
        #[method(drawWithRect:options:)]
        unsafe fn drawWithRect_options(&self, rect: NSRect, options: NSStringDrawingOptions);

        #[cfg(all(feature = "AppKit_NSStringDrawing", feature = "Foundation_NSGeometry"))]
        #[method(boundingRectWithSize:options:)]
        unsafe fn boundingRectWithSize_options(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
        ) -> NSRect;
    }

    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSAttributedStringNSStringDrawingDeprecated for NSAttributedString {}
);
