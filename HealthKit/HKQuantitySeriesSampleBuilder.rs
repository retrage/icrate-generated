//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKQuantitySeriesSampleBuilder;

    unsafe impl ClassType for HKQuantitySeriesSampleBuilder {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for HKQuantitySeriesSampleBuilder {}

extern_methods!(
    unsafe impl HKQuantitySeriesSampleBuilder {
        #[cfg(all(
            feature = "HKDevice",
            feature = "HKHealthStore",
            feature = "HKObjectType"
        ))]
        #[method_id(@__retain_semantics Init initWithHealthStore:quantityType:startDate:device:)]
        pub unsafe fn initWithHealthStore_quantityType_startDate_device(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
            quantity_type: &HKQuantityType,
            start_date: &NSDate,
            device: Option<&HKDevice>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other quantityType)]
        pub unsafe fn quantityType(&self) -> Id<HKQuantityType>;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[cfg(feature = "HKDevice")]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<HKDevice>>;

        #[cfg(feature = "HKQuantity")]
        #[method(insertQuantity:dateInterval:error:_)]
        pub unsafe fn insertQuantity_dateInterval_error(
            &self,
            quantity: &HKQuantity,
            date_interval: &NSDateInterval,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "HKQuantity")]
        #[method(insertQuantity:date:error:_)]
        pub unsafe fn insertQuantity_date_error(
            &self,
            quantity: &HKQuantity,
            date: &NSDate,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKQuantitySample",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method(finishSeriesWithMetadata:endDate:completion:)]
        pub unsafe fn finishSeriesWithMetadata_endDate_completion(
            &self,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            end_date: Option<&NSDate>,
            completion: &Block<dyn Fn(*mut NSArray<HKQuantitySample>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "HKObject",
            feature = "HKQuantitySample",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method(finishSeriesWithMetadata:completion:)]
        pub unsafe fn finishSeriesWithMetadata_completion(
            &self,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
            completion: &Block<dyn Fn(*mut NSArray<HKQuantitySample>, *mut NSError)>,
        );

        #[method(discard)]
        pub unsafe fn discard(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKQuantitySeriesSampleBuilder {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
