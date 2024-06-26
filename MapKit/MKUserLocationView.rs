//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    pub struct MKUserLocationView;

    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    unsafe impl ClassType for MKUserLocationView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = MKAnnotationView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSAccessibility for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSAccessibilityElementProtocol for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSAnimatablePropertyContainer for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSAppearanceCustomization for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSCoding for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSDraggingDestination for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSObjectProtocol for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
unsafe impl NSUserInterfaceItemIdentification for MKUserLocationView {}

extern_methods!(
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    unsafe impl MKUserLocationView {}
);

extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    unsafe impl MKUserLocationView {
        #[cfg(feature = "MKAnnotation")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Allocated<Self>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder)
            -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
