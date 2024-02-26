//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CLLocationDegrees = c_double;

pub type CLLocationAccuracy = c_double;

pub type CLLocationSpeed = c_double;

pub type CLLocationSpeedAccuracy = c_double;

pub type CLLocationDirection = c_double;

pub type CLLocationDirectionAccuracy = c_double;

extern_struct!(
    pub struct CLLocationCoordinate2D {
        pub latitude: CLLocationDegrees,
        pub longitude: CLLocationDegrees,
    }
);

pub type CLLocationDistance = c_double;

extern_static!(kCLDistanceFilterNone: CLLocationDistance);

extern_static!(kCLLocationAccuracyBestForNavigation: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyBest: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyNearestTenMeters: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyHundredMeters: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyKilometer: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyThreeKilometers: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyReduced: CLLocationAccuracy);

extern_static!(CLLocationDistanceMax: CLLocationDistance);

#[cfg(feature = "Foundation_NSDate")]
extern_static!(CLTimeIntervalMax: NSTimeInterval);

extern_static!(kCLLocationCoordinate2DInvalid: CLLocationCoordinate2D);

extern_fn!(
    pub unsafe fn CLLocationCoordinate2DIsValid(coord: CLLocationCoordinate2D) -> Bool;
);

extern_fn!(
    pub unsafe fn CLLocationCoordinate2DMake(
        latitude: CLLocationDegrees,
        longitude: CLLocationDegrees,
    ) -> CLLocationCoordinate2D;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLFloor;

    unsafe impl ClassType for CLFloor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CLFloor {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CLFloor {}

unsafe impl NSObjectProtocol for CLFloor {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CLFloor {}

extern_methods!(
    unsafe impl CLFloor {
        #[method(level)]
        pub unsafe fn level(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLFloor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocationSourceInformation;

    unsafe impl ClassType for CLLocationSourceInformation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CLLocationSourceInformation {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CLLocationSourceInformation {}

unsafe impl NSObjectProtocol for CLLocationSourceInformation {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CLLocationSourceInformation {}

extern_methods!(
    unsafe impl CLLocationSourceInformation {
        #[method_id(@__retain_semantics Init initWithSoftwareSimulationState:andExternalAccessoryState:)]
        pub unsafe fn initWithSoftwareSimulationState_andExternalAccessoryState(
            this: Allocated<Self>,
            is_software: bool,
            is_accessory: bool,
        ) -> Id<Self>;

        #[method(isSimulatedBySoftware)]
        pub unsafe fn isSimulatedBySoftware(&self) -> bool;

        #[method(isProducedByAccessory)]
        pub unsafe fn isProducedByAccessory(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLLocationSourceInformation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocation;

    unsafe impl ClassType for CLLocation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CLLocation {}

unsafe impl Sync for CLLocation {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CLLocation {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CLLocation {}

unsafe impl NSObjectProtocol for CLLocation {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CLLocation {}

extern_methods!(
    unsafe impl CLLocation {
        #[method_id(@__retain_semantics Init initWithLatitude:longitude:)]
        pub unsafe fn initWithLatitude_longitude(
            this: Allocated<Self>,
            latitude: CLLocationDegrees,
            longitude: CLLocationDegrees,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            timestamp: &NSDate,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:speed:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_speed_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            speed: CLLocationSpeed,
            timestamp: &NSDate,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:sourceInfo:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp_sourceInfo(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
            source_info: &CLLocationSourceInformation,
        ) -> Id<Self>;

        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[method(altitude)]
        pub unsafe fn altitude(&self) -> CLLocationDistance;

        #[method(ellipsoidalAltitude)]
        pub unsafe fn ellipsoidalAltitude(&self) -> CLLocationDistance;

        #[method(horizontalAccuracy)]
        pub unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy;

        #[method(verticalAccuracy)]
        pub unsafe fn verticalAccuracy(&self) -> CLLocationAccuracy;

        #[method(course)]
        pub unsafe fn course(&self) -> CLLocationDirection;

        #[method(courseAccuracy)]
        pub unsafe fn courseAccuracy(&self) -> CLLocationDirectionAccuracy;

        #[method(speed)]
        pub unsafe fn speed(&self) -> CLLocationSpeed;

        #[method(speedAccuracy)]
        pub unsafe fn speedAccuracy(&self) -> CLLocationSpeedAccuracy;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics Other floor)]
        pub unsafe fn floor(&self) -> Option<Id<CLFloor>>;

        #[method_id(@__retain_semantics Other sourceInformation)]
        pub unsafe fn sourceInformation(&self) -> Option<Id<CLLocationSourceInformation>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLLocation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
