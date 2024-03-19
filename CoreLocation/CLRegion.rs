//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CLRegionState {
    #[doc(alias = "CLRegionStateUnknown")]
    Unknown = 0,
    #[doc(alias = "CLRegionStateInside")]
    Inside = 1,
    #[doc(alias = "CLRegionStateOutside")]
    Outside = 2,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CLRegionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CLRegionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLProximity(pub NSInteger);
impl CLProximity {
    #[doc(alias = "CLProximityUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CLProximityImmediate")]
    pub const Immediate: Self = Self(1);
    #[doc(alias = "CLProximityNear")]
    pub const Near: Self = Self(2);
    #[doc(alias = "CLProximityFar")]
    pub const Far: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CLProximity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CLProximity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLRegion;

    unsafe impl ClassType for CLRegion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CLRegion {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CLRegion {}

unsafe impl NSObjectProtocol for CLRegion {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CLRegion {}

extern_methods!(
    unsafe impl CLRegion {
        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "Foundation_NSString"))]
        #[deprecated = "Please see CLCircularRegion"]
        #[method_id(@__retain_semantics Init initCircularRegionWithCenter:radius:identifier:)]
        pub unsafe fn initCircularRegionWithCenter_radius_identifier(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method(center)]
        pub unsafe fn center(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(notifyOnEntry)]
        pub unsafe fn notifyOnEntry(&self) -> bool;

        #[method(setNotifyOnEntry:)]
        pub unsafe fn setNotifyOnEntry(&self, notify_on_entry: bool);

        #[method(notifyOnExit)]
        pub unsafe fn notifyOnExit(&self) -> bool;

        #[method(setNotifyOnExit:)]
        pub unsafe fn setNotifyOnExit(&self, notify_on_exit: bool);

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method(containsCoordinate:)]
        pub unsafe fn containsCoordinate(&self, coordinate: CLLocationCoordinate2D) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLRegion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
