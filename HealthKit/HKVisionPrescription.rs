//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum HKVisionPrescriptionType {
        #[doc(alias = "HKVisionPrescriptionTypeGlasses")]
        Glasses = 1,
        #[doc(alias = "HKVisionPrescriptionTypeContacts")]
        Contacts = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    pub struct HKVisionPrescription;

    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl ClassType for HKVisionPrescription {
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
unsafe impl NSCoding for HKVisionPrescription {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSCopying for HKVisionPrescription {}

#[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
unsafe impl NSObjectProtocol for HKVisionPrescription {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "HealthKit_HKObject",
    feature = "HealthKit_HKSample"
))]
unsafe impl NSSecureCoding for HKVisionPrescription {}

extern_methods!(
    #[cfg(all(feature = "HealthKit_HKObject", feature = "HealthKit_HKSample"))]
    unsafe impl HKVisionPrescription {
        #[method(prescriptionType)]
        pub unsafe fn prescriptionType(&self) -> HKVisionPrescriptionType;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateIssued)]
        pub unsafe fn dateIssued(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other expirationDate)]
        pub unsafe fn expirationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKDevice"
        ))]
        #[method_id(@__retain_semantics Other prescriptionWithType:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata(
            r#type: HKVisionPrescriptionType,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
