//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKSeriesSample"
    ))]
    pub struct HKWorkoutRoute;

    #[cfg(all(
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKSeriesSample"
    ))]
    unsafe impl ClassType for HKWorkoutRoute {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKSeriesSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample",
    feature = "HealthKit_HKSeriesSample"
))]
unsafe impl NSCoding for HKWorkoutRoute {}

#[cfg(all(
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample",
    feature = "HealthKit_HKSeriesSample"
))]
unsafe impl NSObjectProtocol for HKWorkoutRoute {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample",
    feature = "HealthKit_HKSeriesSample"
))]
unsafe impl NSSecureCoding for HKWorkoutRoute {}

extern_methods!(
    #[cfg(all(
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKSeriesSample"
    ))]
    unsafe impl HKWorkoutRoute {}
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKSeriesSample"
    ))]
    unsafe impl HKWorkoutRoute {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "HealthKit_HKObject",
        feature = "HealthKit_HKSample",
        feature = "HealthKit_HKSeriesSample"
    ))]
    unsafe impl HKWorkoutRoute {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
