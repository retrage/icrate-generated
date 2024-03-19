//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type HKClinicalTypeIdentifier = NSString;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierAllergyRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierClinicalNoteRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierConditionRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierImmunizationRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierLabResultRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierMedicationRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierProcedureRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierVitalSignRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKClinicalTypeIdentifierCoverageRecord: &'static HKClinicalTypeIdentifier;
}

extern_methods!(
    /// ClinicalType
    #[cfg(feature = "HealthKit_HKObjectType")]
    unsafe impl HKObjectType {
        #[cfg(all(feature = "Foundation_NSString", feature = "HealthKit_HKClinicalType"))]
        #[method_id(@__retain_semantics Other clinicalTypeForIdentifier:)]
        pub unsafe fn clinicalTypeForIdentifier(
            identifier: &HKClinicalTypeIdentifier,
        ) -> Option<Id<HKClinicalType>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKObjectType")]
    pub struct HKClinicalType;

    #[cfg(feature = "HealthKit_HKObjectType")]
    unsafe impl ClassType for HKClinicalType {
        #[inherits(HKObjectType, NSObject)]
        type Super = HKSampleType;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSObject", feature = "HealthKit_HKObjectType"))]
unsafe impl NSCoding for HKClinicalType {}

#[cfg(all(feature = "Foundation_NSObject", feature = "HealthKit_HKObjectType"))]
unsafe impl NSCopying for HKClinicalType {}

#[cfg(feature = "HealthKit_HKObjectType")]
unsafe impl NSObjectProtocol for HKClinicalType {}

#[cfg(all(feature = "Foundation_NSObject", feature = "HealthKit_HKObjectType"))]
unsafe impl NSSecureCoding for HKClinicalType {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKObjectType")]
    unsafe impl HKClinicalType {}
);

extern_methods!(
    /// Methods declared on superclass `HKObjectType`
    #[cfg(feature = "HealthKit_HKObjectType")]
    unsafe impl HKClinicalType {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKObjectType")]
    unsafe impl HKClinicalType {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
