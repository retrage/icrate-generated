//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKWorkoutSessionState {
        #[doc(alias = "HKWorkoutSessionStateNotStarted")]
        NotStarted = 1,
        #[doc(alias = "HKWorkoutSessionStateRunning")]
        Running = 2,
        #[doc(alias = "HKWorkoutSessionStateEnded")]
        Ended = 3,
        #[doc(alias = "HKWorkoutSessionStatePaused")]
        Paused = 4,
        #[doc(alias = "HKWorkoutSessionStatePrepared")]
        Prepared = 5,
        #[doc(alias = "HKWorkoutSessionStateStopped")]
        Stopped = 6,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKWorkoutSessionType {
        #[doc(alias = "HKWorkoutSessionTypePrimary")]
        Primary = 0,
        #[doc(alias = "HKWorkoutSessionTypeMirrored")]
        Mirrored = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKWorkoutSession;

    unsafe impl ClassType for HKWorkoutSession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKWorkoutSession {}

unsafe impl NSObjectProtocol for HKWorkoutSession {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKWorkoutSession {}

extern_methods!(
    unsafe impl HKWorkoutSession {
        #[cfg(feature = "HealthKit_HKWorkout")]
        #[deprecated]
        #[method(activityType)]
        pub unsafe fn activityType(&self) -> HKWorkoutActivityType;

        #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
        #[deprecated]
        #[method(locationType)]
        pub unsafe fn locationType(&self) -> HKWorkoutSessionLocationType;

        #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
        #[method_id(@__retain_semantics Other workoutConfiguration)]
        pub unsafe fn workoutConfiguration(&self) -> Id<HKWorkoutConfiguration>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn HKWorkoutSessionDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn HKWorkoutSessionDelegate>>,
        );

        #[method(state)]
        pub unsafe fn state(&self) -> HKWorkoutSessionState;

        #[method(type)]
        pub unsafe fn r#type(&self) -> HKWorkoutSessionType;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "HealthKit_HKWorkoutActivity")]
        #[method_id(@__retain_semantics Other currentActivity)]
        pub unsafe fn currentActivity(&self) -> Id<HKWorkoutActivity>;

        #[cfg(all(
            feature = "HealthKit_HKWorkout",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithActivityType:locationType:)]
        pub unsafe fn initWithActivityType_locationType(
            this: Allocated<Self>,
            activity_type: HKWorkoutActivityType,
            location_type: HKWorkoutSessionLocationType,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithConfiguration:error:_)]
        pub unsafe fn initWithConfiguration_error(
            this: Allocated<Self>,
            workout_configuration: &HKWorkoutConfiguration,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKHealthStore",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method_id(@__retain_semantics Init initWithHealthStore:configuration:error:_)]
        pub unsafe fn initWithHealthStore_configuration_error(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
            workout_configuration: &HKWorkoutConfiguration,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(prepare)]
        pub unsafe fn prepare(&self);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(startActivityWithDate:)]
        pub unsafe fn startActivityWithDate(&self, date: Option<&NSDate>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(stopActivityWithDate:)]
        pub unsafe fn stopActivityWithDate(&self, date: Option<&NSDate>);

        #[method(end)]
        pub unsafe fn end(&self);

        #[method(pause)]
        pub unsafe fn pause(&self);

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[cfg(all(
            feature = "HealthKit_HKLiveWorkoutBuilder",
            feature = "HealthKit_HKWorkoutBuilder"
        ))]
        #[method_id(@__retain_semantics Other associatedWorkoutBuilder)]
        pub unsafe fn associatedWorkoutBuilder(&self) -> Id<HKLiveWorkoutBuilder>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method(beginNewActivityWithConfiguration:date:metadata:)]
        pub unsafe fn beginNewActivityWithConfiguration_date_metadata(
            &self,
            workout_configuration: &HKWorkoutConfiguration,
            date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSDate")]
        #[method(endCurrentActivityOnDate:)]
        pub unsafe fn endCurrentActivityOnDate(&self, date: &NSDate);

        #[cfg(feature = "Foundation_NSError")]
        #[method(startMirroringToCompanionDeviceWithCompletion:)]
        pub unsafe fn startMirroringToCompanionDeviceWithCompletion(
            &self,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(stopMirroringToCompanionDeviceWithCompletion:)]
        pub unsafe fn stopMirroringToCompanionDeviceWithCompletion(
            &self,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(sendDataToRemoteWorkoutSession:completion:)]
        pub unsafe fn sendDataToRemoteWorkoutSession_completion(
            &self,
            data: &NSData,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKWorkoutSession {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait HKWorkoutSessionDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSDate")]
        #[method(workoutSession:didChangeToState:fromState:date:)]
        unsafe fn workoutSession_didChangeToState_fromState_date(
            &self,
            workout_session: &HKWorkoutSession,
            to_state: HKWorkoutSessionState,
            from_state: HKWorkoutSessionState,
            date: &NSDate,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(workoutSession:didFailWithError:)]
        unsafe fn workoutSession_didFailWithError(
            &self,
            workout_session: &HKWorkoutSession,
            error: &NSError,
        );

        #[cfg(feature = "HealthKit_HKWorkout")]
        #[optional]
        #[method(workoutSession:didGenerateEvent:)]
        unsafe fn workoutSession_didGenerateEvent(
            &self,
            workout_session: &HKWorkoutSession,
            event: &HKWorkoutEvent,
        );

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[optional]
        #[method(workoutSession:didBeginActivityWithConfiguration:date:)]
        unsafe fn workoutSession_didBeginActivityWithConfiguration_date(
            &self,
            workout_session: &HKWorkoutSession,
            workout_configuration: &HKWorkoutConfiguration,
            date: &NSDate,
        );

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[optional]
        #[method(workoutSession:didEndActivityWithConfiguration:date:)]
        unsafe fn workoutSession_didEndActivityWithConfiguration_date(
            &self,
            workout_session: &HKWorkoutSession,
            workout_configuration: &HKWorkoutConfiguration,
            date: &NSDate,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[optional]
        #[method(workoutSession:didReceiveDataFromRemoteWorkoutSession:)]
        unsafe fn workoutSession_didReceiveDataFromRemoteWorkoutSession(
            &self,
            workout_session: &HKWorkoutSession,
            data: &NSArray<NSData>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[optional]
        #[method(workoutSession:didDisconnectFromRemoteDeviceWithError:)]
        unsafe fn workoutSession_didDisconnectFromRemoteDeviceWithError(
            &self,
            workout_session: &HKWorkoutSession,
            error: Option<&NSError>,
        );
    }

    unsafe impl ProtocolType for dyn HKWorkoutSessionDelegate {}
);
