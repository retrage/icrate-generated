//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    pub struct HKSeriesSample;

    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl ClassType for HKSeriesSample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSCoding for HKSeriesSample {}

#[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
unsafe impl NSObjectProtocol for HKSeriesSample {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSSecureCoding for HKSeriesSample {}

extern_methods!(
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl HKSeriesSample {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl HKSeriesSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl HKSeriesSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
