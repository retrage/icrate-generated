//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSCompositingOperation {
        #[doc(alias = "NSCompositingOperationClear")]
        Clear = 0,
        #[doc(alias = "NSCompositingOperationCopy")]
        Copy = 1,
        #[doc(alias = "NSCompositingOperationSourceOver")]
        SourceOver = 2,
        #[doc(alias = "NSCompositingOperationSourceIn")]
        SourceIn = 3,
        #[doc(alias = "NSCompositingOperationSourceOut")]
        SourceOut = 4,
        #[doc(alias = "NSCompositingOperationSourceAtop")]
        SourceAtop = 5,
        #[doc(alias = "NSCompositingOperationDestinationOver")]
        DestinationOver = 6,
        #[doc(alias = "NSCompositingOperationDestinationIn")]
        DestinationIn = 7,
        #[doc(alias = "NSCompositingOperationDestinationOut")]
        DestinationOut = 8,
        #[doc(alias = "NSCompositingOperationDestinationAtop")]
        DestinationAtop = 9,
        #[doc(alias = "NSCompositingOperationXOR")]
        XOR = 10,
        #[doc(alias = "NSCompositingOperationPlusDarker")]
        PlusDarker = 11,
        #[deprecated = "Use NSCompositingOperationSourceOver instead"]
        #[doc(alias = "NSCompositingOperationHighlight")]
        Highlight = 12,
        #[doc(alias = "NSCompositingOperationPlusLighter")]
        PlusLighter = 13,
        #[doc(alias = "NSCompositingOperationMultiply")]
        Multiply = 14,
        #[doc(alias = "NSCompositingOperationScreen")]
        Screen = 15,
        #[doc(alias = "NSCompositingOperationOverlay")]
        Overlay = 16,
        #[doc(alias = "NSCompositingOperationDarken")]
        Darken = 17,
        #[doc(alias = "NSCompositingOperationLighten")]
        Lighten = 18,
        #[doc(alias = "NSCompositingOperationColorDodge")]
        ColorDodge = 19,
        #[doc(alias = "NSCompositingOperationColorBurn")]
        ColorBurn = 20,
        #[doc(alias = "NSCompositingOperationSoftLight")]
        SoftLight = 21,
        #[doc(alias = "NSCompositingOperationHardLight")]
        HardLight = 22,
        #[doc(alias = "NSCompositingOperationDifference")]
        Difference = 23,
        #[doc(alias = "NSCompositingOperationExclusion")]
        Exclusion = 24,
        #[doc(alias = "NSCompositingOperationHue")]
        Hue = 25,
        #[doc(alias = "NSCompositingOperationSaturation")]
        Saturation = 26,
        #[doc(alias = "NSCompositingOperationColor")]
        Color = 27,
        #[doc(alias = "NSCompositingOperationLuminosity")]
        Luminosity = 28,
    }
);

extern_static!(NSCompositeClear: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Clear.0));

extern_static!(NSCompositeCopy: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Copy.0));

extern_static!(NSCompositeSourceOver: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::SourceOver.0));

extern_static!(NSCompositeSourceIn: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::SourceIn.0));

extern_static!(NSCompositeSourceOut: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::SourceOut.0));

extern_static!(NSCompositeSourceAtop: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::SourceAtop.0));

extern_static!(NSCompositeDestinationOver: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::DestinationOver.0));

extern_static!(NSCompositeDestinationIn: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::DestinationIn.0));

extern_static!(NSCompositeDestinationOut: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::DestinationOut.0));

extern_static!(NSCompositeDestinationAtop: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::DestinationAtop.0));

extern_static!(NSCompositeXOR: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::XOR.0));

extern_static!(NSCompositePlusDarker: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::PlusDarker.0));

extern_static!(NSCompositeHighlight: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Highlight.0));

extern_static!(NSCompositePlusLighter: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::PlusLighter.0));

extern_static!(NSCompositeMultiply: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Multiply.0));

extern_static!(NSCompositeScreen: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Screen.0));

extern_static!(NSCompositeOverlay: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Overlay.0));

extern_static!(NSCompositeDarken: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Darken.0));

extern_static!(NSCompositeLighten: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Lighten.0));

extern_static!(NSCompositeColorDodge: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::ColorDodge.0));

extern_static!(NSCompositeColorBurn: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::ColorBurn.0));

extern_static!(NSCompositeSoftLight: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::SoftLight.0));

extern_static!(NSCompositeHardLight: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::HardLight.0));

extern_static!(NSCompositeDifference: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Difference.0));

extern_static!(NSCompositeExclusion: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Exclusion.0));

extern_static!(NSCompositeHue: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Hue.0));

extern_static!(NSCompositeSaturation: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Saturation.0));

extern_static!(NSCompositeColor: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Color.0));

extern_static!(NSCompositeLuminosity: NSCompositingOperation = NSCompositingOperation(NSCompositingOperation::Luminosity.0));

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBackingStoreType {
        #[deprecated]
        NSBackingStoreRetained = 0,
        #[deprecated]
        NSBackingStoreNonretained = 1,
        NSBackingStoreBuffered = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSWindowOrderingMode {
        NSWindowAbove = 1,
        NSWindowBelow = -1,
        NSWindowOut = 0,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFocusRingPlacement {
        NSFocusRingOnly = 0,
        NSFocusRingBelow = 1,
        NSFocusRingAbove = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFocusRingType {
        #[doc(alias = "NSFocusRingTypeDefault")]
        Default = 0,
        #[doc(alias = "NSFocusRingTypeNone")]
        None = 1,
        #[doc(alias = "NSFocusRingTypeExterior")]
        Exterior = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorRenderingIntent {
        #[doc(alias = "NSColorRenderingIntentDefault")]
        Default = 0,
        #[doc(alias = "NSColorRenderingIntentAbsoluteColorimetric")]
        AbsoluteColorimetric = 1,
        #[doc(alias = "NSColorRenderingIntentRelativeColorimetric")]
        RelativeColorimetric = 2,
        #[doc(alias = "NSColorRenderingIntentPerceptual")]
        Perceptual = 3,
        #[doc(alias = "NSColorRenderingIntentSaturation")]
        Saturation = 4,
    }
);

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type NSColorSpaceName = NSString;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSCalibratedWhiteColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSCalibratedRGBColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceWhiteColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceRGBColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceCMYKColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSNamedColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPatternColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSCustomColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSCalibratedBlackColorSpace: &'static NSColorSpaceName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceBlackColorSpace: &'static NSColorSpaceName);

ns_enum!(
    #[underlying(i32)]
    pub enum NSWindowDepth {
        #[doc(alias = "NSWindowDepthTwentyfourBitRGB")]
        TwentyfourBitRGB = 0x208,
        #[doc(alias = "NSWindowDepthSixtyfourBitRGB")]
        SixtyfourBitRGB = 0x210,
        #[doc(alias = "NSWindowDepthOnehundredtwentyeightBitRGB")]
        OnehundredtwentyeightBitRGB = 0x220,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSBestDepth(
        color_space: &NSColorSpaceName,
        bps: NSInteger,
        bpp: NSInteger,
        planar: Bool,
        exact_match: *mut Bool,
    ) -> NSWindowDepth;
);

extern_fn!(
    pub unsafe fn NSPlanarFromDepth(depth: NSWindowDepth) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSColorSpaceFromDepth(depth: NSWindowDepth) -> *mut NSColorSpaceName;
);

extern_fn!(
    pub unsafe fn NSBitsPerSampleFromDepth(depth: NSWindowDepth) -> NSInteger;
);

extern_fn!(
    pub unsafe fn NSBitsPerPixelFromDepth(depth: NSWindowDepth) -> NSInteger;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSNumberOfColorComponents(color_space_name: &NSColorSpaceName) -> NSInteger;
);

extern_fn!(
    pub unsafe fn NSAvailableWindowDepths() -> NonNull<NSWindowDepth>;
);

#[cfg(feature = "Foundation_NSGeometry")]
extern_static!(NSWhite: CGFloat);

#[cfg(feature = "Foundation_NSGeometry")]
extern_static!(NSLightGray: CGFloat);

#[cfg(feature = "Foundation_NSGeometry")]
extern_static!(NSDarkGray: CGFloat);

#[cfg(feature = "Foundation_NSGeometry")]
extern_static!(NSBlack: CGFloat);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDisplayGamut {
        #[doc(alias = "NSDisplayGamutSRGB")]
        SRGB = 1,
        #[doc(alias = "NSDisplayGamutP3")]
        P3 = 2,
    }
);

#[cfg(feature = "Foundation_NSString")]
typed_extensible_enum!(
    pub type NSDeviceDescriptionKey = NSString;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceResolution: &'static NSDeviceDescriptionKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceColorSpaceName: &'static NSDeviceDescriptionKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceBitsPerSample: &'static NSDeviceDescriptionKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceIsScreen: &'static NSDeviceDescriptionKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceIsPrinter: &'static NSDeviceDescriptionKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDeviceSize: &'static NSDeviceDescriptionKey);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSRectFill(rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSRectFillList(rects: NonNull<NSRect>, count: NSInteger);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSRectFillListWithGrays(
        rects: NonNull<NSRect>,
        grays: NonNull<CGFloat>,
        num: NSInteger,
    );
);

extern_fn!(
    #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSGeometry"))]
    pub unsafe fn NSRectFillListWithColors(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
    );
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSRectFillUsingOperation(rect: NSRect, op: NSCompositingOperation);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSRectFillListUsingOperation(
        rects: NonNull<NSRect>,
        count: NSInteger,
        op: NSCompositingOperation,
    );
);

extern_fn!(
    #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSGeometry"))]
    pub unsafe fn NSRectFillListWithColorsUsingOperation(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
        op: NSCompositingOperation,
    );
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSFrameRect(rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSFrameRectWithWidth(rect: NSRect, frame_width: CGFloat);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSFrameRectWithWidthUsingOperation(
        rect: NSRect,
        frame_width: CGFloat,
        op: NSCompositingOperation,
    );
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSRectClip(rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSRectClipList(rects: NonNull<NSRect>, count: NSInteger);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        grays: NonNull<CGFloat>,
        count: NSInteger,
    ) -> NSRect;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawGrayBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawGroove(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawWhiteBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawButton(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSEraseRect(rect: NSRect);
);

extern_fn!(
    #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSGeometry"))]
    #[deprecated = "Use -[NSBitmapImageRep colorAtX:y:] to interrogate pixel values.  If necessary, use -[NSView cacheDisplayInRect:toBitmapImageRep:] to snapshot a view hierarchy into an NSBitmapImageRep."]
    pub unsafe fn NSReadPixel(passed_point: NSPoint) -> *mut NSColor;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    #[deprecated]
    pub unsafe fn NSHighlightRect(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSBeep();
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    #[deprecated = "Doesn't return anything useful since 10.0"]
    pub unsafe fn NSGetWindowServerMemory(
        context: NSInteger,
        virtual_memory: NonNull<NSInteger>,
        window_backing_memory: NonNull<NSInteger>,
        window_dump_string: NonNull<NonNull<NSString>>,
    ) -> NSInteger;
);

extern_fn!(
    #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSGeometry"))]
    pub unsafe fn NSDrawColorTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        colors: NonNull<NonNull<NSColor>>,
        count: NSInteger,
    ) -> NSRect;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawDarkBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawLightBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDottedFrameRect(rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    pub unsafe fn NSDrawWindowBackground(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSSetFocusRingStyle(placement: NSFocusRingPlacement);
);

extern_fn!(
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSDisableScreenUpdates."]
    pub unsafe fn NSDisableScreenUpdates();
);

extern_fn!(
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSEnableScreenUpdates."]
    pub unsafe fn NSEnableScreenUpdates();
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated = "Use +[NSCursor disappearingItemCursor] instead"]
    pub enum NSAnimationEffect {
        #[doc(alias = "NSAnimationEffectDisappearingItemDefault")]
        DisappearingItemDefault = 0,
        #[doc(alias = "NSAnimationEffectPoof")]
        Poof = 10,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    #[deprecated = "Use +[NSCursor disappearingItemCursor] instead"]
    pub unsafe fn NSShowAnimationEffect(
        animation_effect: NSAnimationEffect,
        center_location: NSPoint,
        size: NSSize,
        animation_delegate: Option<&AnyObject>,
        did_end_selector: Option<Sel>,
        context_info: *mut c_void,
    );
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSCountWindows(count: NonNull<NSInteger>);
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSWindowList(size: NSInteger, list: NonNull<NSInteger>);
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSCountWindowsForContext(context: NSInteger, count: NonNull<NSInteger>);
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSWindowListForContext(
        context: NSInteger,
        size: NSInteger,
        list: NonNull<NSInteger>,
    );
);

extern_fn!(
    #[cfg(feature = "Foundation_NSGeometry")]
    #[deprecated]
    pub unsafe fn NSCopyBits(src_g_state: NSInteger, src_rect: NSRect, dest_point: NSPoint);
);
