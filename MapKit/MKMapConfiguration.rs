//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKMapElevationStyle(pub NSInteger);
impl MKMapElevationStyle {
    #[doc(alias = "MKMapElevationStyleFlat")]
    pub const Flat: Self = Self(0);
    #[doc(alias = "MKMapElevationStyleRealistic")]
    pub const Realistic: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MKMapElevationStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MKMapElevationStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapConfiguration;

    unsafe impl ClassType for MKMapConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MKMapConfiguration {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MKMapConfiguration {}

unsafe impl NSObjectProtocol for MKMapConfiguration {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MKMapConfiguration {}

extern_methods!(
    unsafe impl MKMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(elevationStyle)]
        pub unsafe fn elevationStyle(&self) -> MKMapElevationStyle;

        #[method(setElevationStyle:)]
        pub unsafe fn setElevationStyle(&self, elevation_style: MKMapElevationStyle);
    }
);
