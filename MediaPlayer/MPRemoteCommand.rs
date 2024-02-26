//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPRemoteCommandHandlerStatus {
        #[doc(alias = "MPRemoteCommandHandlerStatusSuccess")]
        Success = 0,
        #[doc(alias = "MPRemoteCommandHandlerStatusNoSuchContent")]
        NoSuchContent = 100,
        #[doc(alias = "MPRemoteCommandHandlerStatusNoActionableNowPlayingItem")]
        NoActionableNowPlayingItem = 110,
        #[doc(alias = "MPRemoteCommandHandlerStatusDeviceNotFound")]
        DeviceNotFound = 120,
        #[doc(alias = "MPRemoteCommandHandlerStatusCommandFailed")]
        CommandFailed = 200,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPRemoteCommand;

    unsafe impl ClassType for MPRemoteCommand {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPRemoteCommand {}

extern_methods!(
    unsafe impl MPRemoteCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(addTarget:action:)]
        pub unsafe fn addTarget_action(&self, target: &AnyObject, action: Sel);

        #[method(removeTarget:action:)]
        pub unsafe fn removeTarget_action(&self, target: &AnyObject, action: Option<Sel>);

        #[method(removeTarget:)]
        pub unsafe fn removeTarget(&self, target: Option<&AnyObject>);

        #[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
        #[method_id(@__retain_semantics Other addTargetWithHandler:)]
        pub unsafe fn addTargetWithHandler(
            &self,
            handler: &Block<dyn Fn(NonNull<MPRemoteCommandEvent>) -> MPRemoteCommandHandlerStatus>,
        ) -> Id<AnyObject>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSkipIntervalCommand;

    unsafe impl ClassType for MPSkipIntervalCommand {
        #[inherits(NSObject)]
        type Super = MPRemoteCommand;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPSkipIntervalCommand {}

extern_methods!(
    unsafe impl MPSkipIntervalCommand {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other preferredIntervals)]
        pub unsafe fn preferredIntervals(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method(setPreferredIntervals:)]
        pub unsafe fn setPreferredIntervals(&self, preferred_intervals: &NSArray<NSNumber>);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPRemoteCommand`
    unsafe impl MPSkipIntervalCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPFeedbackCommand;

    unsafe impl ClassType for MPFeedbackCommand {
        #[inherits(NSObject)]
        type Super = MPRemoteCommand;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPFeedbackCommand {}

extern_methods!(
    unsafe impl MPFeedbackCommand {
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedTitle:)]
        pub unsafe fn setLocalizedTitle(&self, localized_title: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedShortTitle)]
        pub unsafe fn localizedShortTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedShortTitle:)]
        pub unsafe fn setLocalizedShortTitle(&self, localized_short_title: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPRemoteCommand`
    unsafe impl MPFeedbackCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPRatingCommand;

    unsafe impl ClassType for MPRatingCommand {
        #[inherits(NSObject)]
        type Super = MPRemoteCommand;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPRatingCommand {}

extern_methods!(
    unsafe impl MPRatingCommand {
        #[method(minimumRating)]
        pub unsafe fn minimumRating(&self) -> c_float;

        #[method(setMinimumRating:)]
        pub unsafe fn setMinimumRating(&self, minimum_rating: c_float);

        #[method(maximumRating)]
        pub unsafe fn maximumRating(&self) -> c_float;

        #[method(setMaximumRating:)]
        pub unsafe fn setMaximumRating(&self, maximum_rating: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPRemoteCommand`
    unsafe impl MPRatingCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangePlaybackRateCommand;

    unsafe impl ClassType for MPChangePlaybackRateCommand {
        #[inherits(NSObject)]
        type Super = MPRemoteCommand;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPChangePlaybackRateCommand {}

extern_methods!(
    unsafe impl MPChangePlaybackRateCommand {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other supportedPlaybackRates)]
        pub unsafe fn supportedPlaybackRates(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method(setSupportedPlaybackRates:)]
        pub unsafe fn setSupportedPlaybackRates(
            &self,
            supported_playback_rates: &NSArray<NSNumber>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPRemoteCommand`
    unsafe impl MPChangePlaybackRateCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangePlaybackPositionCommand;

    unsafe impl ClassType for MPChangePlaybackPositionCommand {
        #[inherits(NSObject)]
        type Super = MPRemoteCommand;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPChangePlaybackPositionCommand {}

extern_methods!(
    unsafe impl MPChangePlaybackPositionCommand {}
);

extern_methods!(
    /// Methods declared on superclass `MPRemoteCommand`
    unsafe impl MPChangePlaybackPositionCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangeShuffleModeCommand;

    unsafe impl ClassType for MPChangeShuffleModeCommand {
        #[inherits(NSObject)]
        type Super = MPRemoteCommand;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPChangeShuffleModeCommand {}

extern_methods!(
    unsafe impl MPChangeShuffleModeCommand {
        #[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
        #[method(currentShuffleType)]
        pub unsafe fn currentShuffleType(&self) -> MPShuffleType;

        #[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
        #[method(setCurrentShuffleType:)]
        pub unsafe fn setCurrentShuffleType(&self, current_shuffle_type: MPShuffleType);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPRemoteCommand`
    unsafe impl MPChangeShuffleModeCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPChangeRepeatModeCommand;

    unsafe impl ClassType for MPChangeRepeatModeCommand {
        #[inherits(NSObject)]
        type Super = MPRemoteCommand;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPChangeRepeatModeCommand {}

extern_methods!(
    unsafe impl MPChangeRepeatModeCommand {
        #[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
        #[method(currentRepeatType)]
        pub unsafe fn currentRepeatType(&self) -> MPRepeatType;

        #[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
        #[method(setCurrentRepeatType:)]
        pub unsafe fn setCurrentRepeatType(&self, current_repeat_type: MPRepeatType);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPRemoteCommand`
    unsafe impl MPChangeRepeatModeCommand {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
