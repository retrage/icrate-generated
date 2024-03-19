//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKAppleWalkingSteadinessClassification(pub NSInteger);
impl HKAppleWalkingSteadinessClassification {
    #[doc(alias = "HKAppleWalkingSteadinessClassificationOK")]
    pub const OK: Self = Self(1);
    #[doc(alias = "HKAppleWalkingSteadinessClassificationLow")]
    pub const Low: Self = Self(2);
    #[doc(alias = "HKAppleWalkingSteadinessClassificationVeryLow")]
    pub const VeryLow: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for HKAppleWalkingSteadinessClassification {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for HKAppleWalkingSteadinessClassification {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "HealthKit_HKQuantity"))]
    pub fn HKAppleWalkingSteadinessClassificationForQuantity(
        value: &HKQuantity,
        classification_out: NonNull<HKAppleWalkingSteadinessClassification>,
        error_out: *mut *mut NSError,
    ) -> Bool;
}

extern "C" {
    #[cfg(feature = "HealthKit_HKQuantity")]
    pub fn HKAppleWalkingSteadinessMinimumQuantityForClassification(
        classification: HKAppleWalkingSteadinessClassification,
    ) -> NonNull<HKQuantity>;
}

extern "C" {
    #[cfg(feature = "HealthKit_HKQuantity")]
    pub fn HKAppleWalkingSteadinessMaximumQuantityForClassification(
        classification: HKAppleWalkingSteadinessClassification,
    ) -> NonNull<HKQuantity>;
}
