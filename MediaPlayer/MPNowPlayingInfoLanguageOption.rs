//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicIsMainProgramContent: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicIsAuxiliaryContent: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicTranscribesSpokenDialog: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicDescribesMusicAndSound: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicEasyToRead: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicDescribesVideo: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicLanguageTranslation: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicDubbedTranslation: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPLanguageOptionCharacteristicVoiceOverTranslation: &'static NSString);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MPNowPlayingInfoLanguageOptionType {
        #[doc(alias = "MPNowPlayingInfoLanguageOptionTypeAudible")]
        Audible = 0,
        #[doc(alias = "MPNowPlayingInfoLanguageOptionTypeLegible")]
        Legible = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPNowPlayingInfoLanguageOption;

    unsafe impl ClassType for MPNowPlayingInfoLanguageOption {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPNowPlayingInfoLanguageOption {}

extern_methods!(
    unsafe impl MPNowPlayingInfoLanguageOption {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithType:languageTag:characteristics:displayName:identifier:)]
        pub unsafe fn initWithType_languageTag_characteristics_displayName_identifier(
            this: Allocated<Self>,
            language_option_type: MPNowPlayingInfoLanguageOptionType,
            language_tag: &NSString,
            language_option_characteristics: Option<&NSArray<NSString>>,
            display_name: &NSString,
            identifier: &NSString,
        ) -> Id<Self>;

        #[method(isAutomaticLegibleLanguageOption)]
        pub unsafe fn isAutomaticLegibleLanguageOption(&self) -> bool;

        #[method(isAutomaticAudibleLanguageOption)]
        pub unsafe fn isAutomaticAudibleLanguageOption(&self) -> bool;

        #[method(languageOptionType)]
        pub unsafe fn languageOptionType(&self) -> MPNowPlayingInfoLanguageOptionType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other languageTag)]
        pub unsafe fn languageTag(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other languageOptionCharacteristics)]
        pub unsafe fn languageOptionCharacteristics(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPNowPlayingInfoLanguageOption {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPNowPlayingInfoLanguageOptionGroup;

    unsafe impl ClassType for MPNowPlayingInfoLanguageOptionGroup {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPNowPlayingInfoLanguageOptionGroup {}

extern_methods!(
    unsafe impl MPNowPlayingInfoLanguageOptionGroup {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithLanguageOptions:defaultLanguageOption:allowEmptySelection:)]
        pub unsafe fn initWithLanguageOptions_defaultLanguageOption_allowEmptySelection(
            this: Allocated<Self>,
            language_options: &NSArray<MPNowPlayingInfoLanguageOption>,
            default_language_option: Option<&MPNowPlayingInfoLanguageOption>,
            allow_empty_selection: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other languageOptions)]
        pub unsafe fn languageOptions(&self) -> Id<NSArray<MPNowPlayingInfoLanguageOption>>;

        #[method_id(@__retain_semantics Other defaultLanguageOption)]
        pub unsafe fn defaultLanguageOption(&self) -> Option<Id<MPNowPlayingInfoLanguageOption>>;

        #[method(allowEmptySelection)]
        pub unsafe fn allowEmptySelection(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPNowPlayingInfoLanguageOptionGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
