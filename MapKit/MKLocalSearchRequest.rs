//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MKLocalSearchResultType {
        #[doc(alias = "MKLocalSearchResultTypeAddress")]
        Address = 1 << 0,
        #[doc(alias = "MKLocalSearchResultTypePointOfInterest")]
        PointOfInterest = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalSearchRequest;

    unsafe impl ClassType for MKLocalSearchRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MKLocalSearchRequest {}

unsafe impl NSObjectProtocol for MKLocalSearchRequest {}

extern_methods!(
    unsafe impl MKLocalSearchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithNaturalLanguageQuery:)]
        pub unsafe fn initWithNaturalLanguageQuery(
            this: Allocated<Self>,
            natural_language_query: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSString",
            feature = "MapKit_MKGeometry"
        ))]
        #[method_id(@__retain_semantics Init initWithNaturalLanguageQuery:region:)]
        pub unsafe fn initWithNaturalLanguageQuery_region(
            this: Allocated<Self>,
            natural_language_query: &NSString,
            region: MKCoordinateRegion,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other naturalLanguageQuery)]
        pub unsafe fn naturalLanguageQuery(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNaturalLanguageQuery:)]
        pub unsafe fn setNaturalLanguageQuery(&self, natural_language_query: Option<&NSString>);

        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "MapKit_MKGeometry"))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "MapKit_MKGeometry"))]
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[method(resultTypes)]
        pub unsafe fn resultTypes(&self) -> MKLocalSearchResultType;

        #[method(setResultTypes:)]
        pub unsafe fn setResultTypes(&self, result_types: MKLocalSearchResultType);

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalSearchRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
