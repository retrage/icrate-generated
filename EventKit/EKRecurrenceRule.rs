//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKObject")]
    pub struct EKRecurrenceRule;

    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl ClassType for EKRecurrenceRule {
        #[inherits(NSObject)]
        type Super = EKObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "EventKit_EKObject", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for EKRecurrenceRule {}

#[cfg(feature = "EventKit_EKObject")]
unsafe impl NSObjectProtocol for EKRecurrenceRule {}

extern_methods!(
    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl EKRecurrenceRule {
        #[cfg(all(feature = "EventKit_EKRecurrenceEnd", feature = "EventKit_EKTypes"))]
        #[method_id(@__retain_semantics Init initRecurrenceWithFrequency:interval:end:)]
        pub unsafe fn initRecurrenceWithFrequency_interval_end(
            this: Allocated<Self>,
            r#type: EKRecurrenceFrequency,
            interval: NSInteger,
            end: Option<&EKRecurrenceEnd>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "EventKit_EKRecurrenceDayOfWeek",
            feature = "EventKit_EKRecurrenceEnd",
            feature = "EventKit_EKTypes",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSValue"
        ))]
        #[method_id(@__retain_semantics Init initRecurrenceWithFrequency:interval:daysOfTheWeek:daysOfTheMonth:monthsOfTheYear:weeksOfTheYear:daysOfTheYear:setPositions:end:)]
        pub unsafe fn initRecurrenceWithFrequency_interval_daysOfTheWeek_daysOfTheMonth_monthsOfTheYear_weeksOfTheYear_daysOfTheYear_setPositions_end(
            this: Allocated<Self>,
            r#type: EKRecurrenceFrequency,
            interval: NSInteger,
            days: Option<&NSArray<EKRecurrenceDayOfWeek>>,
            month_days: Option<&NSArray<NSNumber>>,
            months: Option<&NSArray<NSNumber>>,
            weeks_of_the_year: Option<&NSArray<NSNumber>>,
            days_of_the_year: Option<&NSArray<NSNumber>>,
            set_positions: Option<&NSArray<NSNumber>>,
            end: Option<&EKRecurrenceEnd>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "EventKit_EKRecurrenceEnd")]
        #[method_id(@__retain_semantics Other recurrenceEnd)]
        pub unsafe fn recurrenceEnd(&self) -> Option<Id<EKRecurrenceEnd>>;

        #[cfg(feature = "EventKit_EKRecurrenceEnd")]
        #[method(setRecurrenceEnd:)]
        pub unsafe fn setRecurrenceEnd(&self, recurrence_end: Option<&EKRecurrenceEnd>);

        #[cfg(feature = "EventKit_EKTypes")]
        #[method(frequency)]
        pub unsafe fn frequency(&self) -> EKRecurrenceFrequency;

        #[method(interval)]
        pub unsafe fn interval(&self) -> NSInteger;

        #[method(firstDayOfTheWeek)]
        pub unsafe fn firstDayOfTheWeek(&self) -> NSInteger;

        #[cfg(all(
            feature = "EventKit_EKRecurrenceDayOfWeek",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other daysOfTheWeek)]
        pub unsafe fn daysOfTheWeek(&self) -> Option<Id<NSArray<EKRecurrenceDayOfWeek>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other daysOfTheMonth)]
        pub unsafe fn daysOfTheMonth(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other daysOfTheYear)]
        pub unsafe fn daysOfTheYear(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other weeksOfTheYear)]
        pub unsafe fn weeksOfTheYear(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other monthsOfTheYear)]
        pub unsafe fn monthsOfTheYear(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other setPositions)]
        pub unsafe fn setPositions(&self) -> Option<Id<NSArray<NSNumber>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl EKRecurrenceRule {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
