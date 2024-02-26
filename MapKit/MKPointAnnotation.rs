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
    #[cfg(feature = "MapKit_MKShape")]
    pub struct MKPointAnnotation;

    #[cfg(feature = "MapKit_MKShape")]
    unsafe impl ClassType for MKPointAnnotation {
        #[inherits(NSObject)]
        type Super = MKShape;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "MapKit_MKAnnotation", feature = "MapKit_MKShape"))]
unsafe impl MKAnnotation for MKPointAnnotation {}

#[cfg(feature = "MapKit_MKShape")]
unsafe impl NSObjectProtocol for MKPointAnnotation {}

extern_methods!(
    #[cfg(feature = "MapKit_MKShape")]
    unsafe impl MKPointAnnotation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Init initWithCoordinate:)]
        pub unsafe fn initWithCoordinate(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
        ) -> Id<Self>;

        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithCoordinate:title:subtitle:)]
        pub unsafe fn initWithCoordinate_title_subtitle(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            title: Option<&NSString>,
            subtitle: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(setCoordinate:)]
        pub unsafe fn setCoordinate(&self, coordinate: CLLocationCoordinate2D);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKShape")]
    unsafe impl MKPointAnnotation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
