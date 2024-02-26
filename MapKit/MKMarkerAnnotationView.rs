//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "MapKit_MKAnnotationView"
    ))]
    pub struct MKMarkerAnnotationView;

    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "MapKit_MKAnnotationView"
    ))]
    unsafe impl ClassType for MKMarkerAnnotationView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = MKAnnotationView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSAccessibility for MKMarkerAnnotationView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSAccessibilityElementProtocol for MKMarkerAnnotationView {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSAnimatablePropertyContainer for MKMarkerAnnotationView {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSAppearanceCustomization for MKMarkerAnnotationView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSCoding for MKMarkerAnnotationView {}

#[cfg(all(
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSDraggingDestination for MKMarkerAnnotationView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSObjectProtocol for MKMarkerAnnotationView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView",
    feature = "MapKit_MKAnnotationView"
))]
unsafe impl NSUserInterfaceItemIdentification for MKMarkerAnnotationView {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "MapKit_MKAnnotationView"
    ))]
    unsafe impl MKMarkerAnnotationView {
        #[cfg(feature = "MapKit_MKTypes")]
        #[method(titleVisibility)]
        pub unsafe fn titleVisibility(&self) -> MKFeatureVisibility;

        #[cfg(feature = "MapKit_MKTypes")]
        #[method(setTitleVisibility:)]
        pub unsafe fn setTitleVisibility(&self, title_visibility: MKFeatureVisibility);

        #[cfg(feature = "MapKit_MKTypes")]
        #[method(subtitleVisibility)]
        pub unsafe fn subtitleVisibility(&self) -> MKFeatureVisibility;

        #[cfg(feature = "MapKit_MKTypes")]
        #[method(setSubtitleVisibility:)]
        pub unsafe fn setSubtitleVisibility(&self, subtitle_visibility: MKFeatureVisibility);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other markerTintColor)]
        pub unsafe fn markerTintColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setMarkerTintColor:)]
        pub unsafe fn setMarkerTintColor(&self, marker_tint_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other glyphTintColor)]
        pub unsafe fn glyphTintColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setGlyphTintColor:)]
        pub unsafe fn setGlyphTintColor(&self, glyph_tint_color: Option<&NSColor>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other glyphText)]
        pub unsafe fn glyphText(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setGlyphText:)]
        pub unsafe fn setGlyphText(&self, glyph_text: Option<&NSString>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other glyphImage)]
        pub unsafe fn glyphImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setGlyphImage:)]
        pub unsafe fn setGlyphImage(&self, glyph_image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other selectedGlyphImage)]
        pub unsafe fn selectedGlyphImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setSelectedGlyphImage:)]
        pub unsafe fn setSelectedGlyphImage(&self, selected_glyph_image: Option<&NSImage>);

        #[method(animatesWhenAdded)]
        pub unsafe fn animatesWhenAdded(&self) -> bool;

        #[method(setAnimatesWhenAdded:)]
        pub unsafe fn setAnimatesWhenAdded(&self, animates_when_added: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "MapKit_MKAnnotationView"
    ))]
    unsafe impl MKMarkerAnnotationView {
        #[cfg(all(feature = "Foundation_NSString", feature = "MapKit_MKAnnotation"))]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Allocated<Self>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder)
            -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "MapKit_MKAnnotationView"
    ))]
    unsafe impl MKMarkerAnnotationView {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "MapKit_MKAnnotationView"
    ))]
    unsafe impl MKMarkerAnnotationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView",
        feature = "MapKit_MKAnnotationView"
    ))]
    unsafe impl MKMarkerAnnotationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
