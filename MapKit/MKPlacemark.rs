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
    #[cfg(feature = "CoreLocation_CLPlacemark")]
    pub struct MKPlacemark;

    #[cfg(feature = "CoreLocation_CLPlacemark")]
    unsafe impl ClassType for MKPlacemark {
        #[inherits(NSObject)]
        type Super = CLPlacemark;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CoreLocation_CLPlacemark", feature = "MapKit_MKAnnotation"))]
unsafe impl MKAnnotation for MKPlacemark {}

#[cfg(all(feature = "CoreLocation_CLPlacemark", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for MKPlacemark {}

#[cfg(all(feature = "CoreLocation_CLPlacemark", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for MKPlacemark {}

#[cfg(feature = "CoreLocation_CLPlacemark")]
unsafe impl NSObjectProtocol for MKPlacemark {}

#[cfg(all(feature = "CoreLocation_CLPlacemark", feature = "Foundation_NSObject"))]
unsafe impl NSSecureCoding for MKPlacemark {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLPlacemark")]
    unsafe impl MKPlacemark {
        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Init initWithCoordinate:)]
        pub unsafe fn initWithCoordinate(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithCoordinate:addressDictionary:)]
        pub unsafe fn initWithCoordinate_addressDictionary(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            address_dictionary: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Contacts_CNPostalAddress",
            feature = "CoreLocation_CLLocation"
        ))]
        #[method_id(@__retain_semantics Init initWithCoordinate:postalAddress:)]
        pub unsafe fn initWithCoordinate_postalAddress(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            postal_address: &CNPostalAddress,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLPlacemark`
    #[cfg(feature = "CoreLocation_CLPlacemark")]
    unsafe impl MKPlacemark {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithPlacemark:)]
        pub unsafe fn initWithPlacemark(this: Allocated<Self>, placemark: &CLPlacemark)
            -> Id<Self>;
    }
);
