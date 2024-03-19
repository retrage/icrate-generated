//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKSpan(pub NSInteger);
impl EKSpan {
    #[doc(alias = "EKSpanThisEvent")]
    pub const ThisEvent: Self = Self(0);
    #[doc(alias = "EKSpanFutureEvents")]
    pub const FutureEvents: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for EKSpan {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for EKSpan {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(
    feature = "EventKit_EKCalendarItem",
    feature = "EventKit_EKEvent",
    feature = "EventKit_EKObject"
))]
pub type EKEventSearchCallback = *mut Block<dyn Fn(NonNull<EKEvent>, NonNull<Bool>)>;

#[cfg(feature = "Foundation_NSError")]
pub type EKEventStoreRequestAccessCompletionHandler = *mut Block<dyn Fn(Bool, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EKEventStore;

    unsafe impl ClassType for EKEventStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for EKEventStore {}

extern_methods!(
    unsafe impl EKEventStore {
        #[cfg(feature = "EventKit_EKTypes")]
        #[method(authorizationStatusForEntityType:)]
        pub unsafe fn authorizationStatusForEntityType(
            entity_type: EKEntityType,
        ) -> EKAuthorizationStatus;

        #[cfg(feature = "EventKit_EKTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithAccessToEntityTypes:)]
        pub unsafe fn initWithAccessToEntityTypes(
            this: Allocated<Self>,
            entity_types: EKEntityMask,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "EventKit_EKObject",
            feature = "EventKit_EKSource",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithSources:)]
        pub unsafe fn initWithSources(
            this: Allocated<Self>,
            sources: &NSArray<EKSource>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestFullAccessToEventsWithCompletion:)]
        pub unsafe fn requestFullAccessToEventsWithCompletion(
            &self,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestWriteOnlyAccessToEventsWithCompletion:)]
        pub unsafe fn requestWriteOnlyAccessToEventsWithCompletion(
            &self,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestFullAccessToRemindersWithCompletion:)]
        pub unsafe fn requestFullAccessToRemindersWithCompletion(
            &self,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[cfg(all(feature = "EventKit_EKTypes", feature = "Foundation_NSError"))]
        #[deprecated = "Use -requestFullAccessToEventsWithCompletion:, -requestWriteOnlyAccessToEventsWithCompletion:, or -requestFullAccessToRemindersWithCompletion:"]
        #[method(requestAccessToEntityType:completion:)]
        pub unsafe fn requestAccessToEntityType_completion(
            &self,
            entity_type: EKEntityType,
            completion: EKEventStoreRequestAccessCompletionHandler,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other eventStoreIdentifier)]
        pub unsafe fn eventStoreIdentifier(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "EventKit_EKObject",
            feature = "EventKit_EKSource",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other delegateSources)]
        pub unsafe fn delegateSources(&self) -> Id<NSArray<EKSource>>;

        #[cfg(all(
            feature = "EventKit_EKObject",
            feature = "EventKit_EKSource",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other sources)]
        pub unsafe fn sources(&self) -> Id<NSArray<EKSource>>;

        #[cfg(all(
            feature = "EventKit_EKObject",
            feature = "EventKit_EKSource",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other sourceWithIdentifier:)]
        pub unsafe fn sourceWithIdentifier(&self, identifier: &NSString) -> Option<Id<EKSource>>;

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other calendars)]
        pub unsafe fn calendars(&self) -> Id<NSArray<EKCalendar>>;

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "EventKit_EKTypes",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other calendarsForEntityType:)]
        pub unsafe fn calendarsForEntityType(
            &self,
            entity_type: EKEntityType,
        ) -> Id<NSArray<EKCalendar>>;

        #[cfg(all(feature = "EventKit_EKCalendar", feature = "EventKit_EKObject"))]
        #[method_id(@__retain_semantics Other defaultCalendarForNewEvents)]
        pub unsafe fn defaultCalendarForNewEvents(&self) -> Option<Id<EKCalendar>>;

        #[cfg(all(feature = "EventKit_EKCalendar", feature = "EventKit_EKObject"))]
        #[method_id(@__retain_semantics Other defaultCalendarForNewReminders)]
        pub unsafe fn defaultCalendarForNewReminders(&self) -> Option<Id<EKCalendar>>;

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other calendarWithIdentifier:)]
        pub unsafe fn calendarWithIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Id<EKCalendar>>;

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSError"
        ))]
        #[method(saveCalendar:commit:error:_)]
        pub unsafe fn saveCalendar_commit_error(
            &self,
            calendar: &EKCalendar,
            commit: bool,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSError"
        ))]
        #[method(removeCalendar:commit:error:_)]
        pub unsafe fn removeCalendar_commit_error(
            &self,
            calendar: &EKCalendar,
            commit: bool,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other calendarItemWithIdentifier:)]
        pub unsafe fn calendarItemWithIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Id<EKCalendarItem>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other calendarItemsWithExternalIdentifier:)]
        pub unsafe fn calendarItemsWithExternalIdentifier(
            &self,
            external_identifier: &NSString,
        ) -> Id<NSArray<EKCalendarItem>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKEvent",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSError"
        ))]
        #[method(saveEvent:span:error:_)]
        pub unsafe fn saveEvent_span_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKEvent",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSError"
        ))]
        #[method(removeEvent:span:error:_)]
        pub unsafe fn removeEvent_span_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKEvent",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSError"
        ))]
        #[method(saveEvent:span:commit:error:_)]
        pub unsafe fn saveEvent_span_commit_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
            commit: bool,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKEvent",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSError"
        ))]
        #[method(removeEvent:span:commit:error:_)]
        pub unsafe fn removeEvent_span_commit_error(
            &self,
            event: &EKEvent,
            span: EKSpan,
            commit: bool,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKEvent",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other eventWithIdentifier:)]
        pub unsafe fn eventWithIdentifier(&self, identifier: &NSString) -> Option<Id<EKEvent>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKEvent",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other eventsMatchingPredicate:)]
        pub unsafe fn eventsMatchingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Id<NSArray<EKEvent>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKEvent",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSPredicate"
        ))]
        #[method(enumerateEventsMatchingPredicate:usingBlock:)]
        pub unsafe fn enumerateEventsMatchingPredicate_usingBlock(
            &self,
            predicate: &NSPredicate,
            block: EKEventSearchCallback,
        );

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDate",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other predicateForEventsWithStartDate:endDate:calendars:)]
        pub unsafe fn predicateForEventsWithStartDate_endDate_calendars(
            &self,
            start_date: &NSDate,
            end_date: &NSDate,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Id<NSPredicate>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKObject",
            feature = "EventKit_EKReminder",
            feature = "Foundation_NSError"
        ))]
        #[method(saveReminder:commit:error:_)]
        pub unsafe fn saveReminder_commit_error(
            &self,
            reminder: &EKReminder,
            commit: bool,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKObject",
            feature = "EventKit_EKReminder",
            feature = "Foundation_NSError"
        ))]
        #[method(removeReminder:commit:error:_)]
        pub unsafe fn removeReminder_commit_error(
            &self,
            reminder: &EKReminder,
            commit: bool,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "EventKit_EKCalendarItem",
            feature = "EventKit_EKObject",
            feature = "EventKit_EKReminder",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other fetchRemindersMatchingPredicate:completion:)]
        pub unsafe fn fetchRemindersMatchingPredicate_completion(
            &self,
            predicate: &NSPredicate,
            completion: &Block<dyn Fn(*mut NSArray<EKReminder>)>,
        ) -> Id<AnyObject>;

        #[method(cancelFetchRequest:)]
        pub unsafe fn cancelFetchRequest(&self, fetch_identifier: &AnyObject);

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other predicateForRemindersInCalendars:)]
        pub unsafe fn predicateForRemindersInCalendars(
            &self,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Id<NSPredicate>;

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDate",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other predicateForIncompleteRemindersWithDueDateStarting:ending:calendars:)]
        pub unsafe fn predicateForIncompleteRemindersWithDueDateStarting_ending_calendars(
            &self,
            start_date: Option<&NSDate>,
            end_date: Option<&NSDate>,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Id<NSPredicate>;

        #[cfg(all(
            feature = "EventKit_EKCalendar",
            feature = "EventKit_EKObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDate",
            feature = "Foundation_NSPredicate"
        ))]
        #[method_id(@__retain_semantics Other predicateForCompletedRemindersWithCompletionDateStarting:ending:calendars:)]
        pub unsafe fn predicateForCompletedRemindersWithCompletionDateStarting_ending_calendars(
            &self,
            start_date: Option<&NSDate>,
            end_date: Option<&NSDate>,
            calendars: Option<&NSArray<EKCalendar>>,
        ) -> Id<NSPredicate>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(commit:_)]
        pub unsafe fn commit(&self) -> Result<(), Id<NSError>>;

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(refreshSourcesIfNecessary)]
        pub unsafe fn refreshSourcesIfNecessary(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EKEventStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static EKEventStoreChangedNotification: &'static NSString;
}
