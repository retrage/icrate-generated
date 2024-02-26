//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageAlignment {
        NSImageAlignCenter = 0,
        NSImageAlignTop = 1,
        NSImageAlignTopLeft = 2,
        NSImageAlignTopRight = 3,
        NSImageAlignLeft = 4,
        NSImageAlignBottom = 5,
        NSImageAlignBottomLeft = 6,
        NSImageAlignBottomRight = 7,
        NSImageAlignRight = 8,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageFrameStyle {
        NSImageFrameNone = 0,
        NSImageFramePhoto = 1,
        NSImageFrameGrayBezel = 2,
        NSImageFrameGroove = 3,
        NSImageFrameButton = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCell")]
    pub struct NSImageCell;

    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl ClassType for NSImageCell {
        #[inherits(NSObject)]
        type Super = NSCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "AppKit_NSAccessibilityProtocols", feature = "AppKit_NSCell"))]
unsafe impl NSAccessibility for NSImageCell {}

#[cfg(all(feature = "AppKit_NSAccessibilityProtocols", feature = "AppKit_NSCell"))]
unsafe impl NSAccessibilityElementProtocol for NSImageCell {}

#[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for NSImageCell {}

#[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for NSImageCell {}

#[cfg(feature = "AppKit_NSCell")]
unsafe impl NSObjectProtocol for NSImageCell {}

#[cfg(all(
    feature = "AppKit_NSCell",
    feature = "AppKit_NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSImageCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSImageCell {
        #[method(imageAlignment)]
        pub unsafe fn imageAlignment(&self) -> NSImageAlignment;

        #[method(setImageAlignment:)]
        pub unsafe fn setImageAlignment(&self, image_alignment: NSImageAlignment);

        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

        #[method(imageFrameStyle)]
        pub unsafe fn imageFrameStyle(&self) -> NSImageFrameStyle;

        #[method(setImageFrameStyle:)]
        pub unsafe fn setImageFrameStyle(&self, image_frame_style: NSImageFrameStyle);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSImageCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(this: Allocated<Self>, image: Option<&NSImage>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSImageCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
