//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextCheckingType(pub u64);
impl NSTextCheckingType {
    #[doc(alias = "NSTextCheckingTypeOrthography")]
    pub const Orthography: Self = Self(1 << 0);
    #[doc(alias = "NSTextCheckingTypeSpelling")]
    pub const Spelling: Self = Self(1 << 1);
    #[doc(alias = "NSTextCheckingTypeGrammar")]
    pub const Grammar: Self = Self(1 << 2);
    #[doc(alias = "NSTextCheckingTypeDate")]
    pub const Date: Self = Self(1 << 3);
    #[doc(alias = "NSTextCheckingTypeAddress")]
    pub const Address: Self = Self(1 << 4);
    #[doc(alias = "NSTextCheckingTypeLink")]
    pub const Link: Self = Self(1 << 5);
    #[doc(alias = "NSTextCheckingTypeQuote")]
    pub const Quote: Self = Self(1 << 6);
    #[doc(alias = "NSTextCheckingTypeDash")]
    pub const Dash: Self = Self(1 << 7);
    #[doc(alias = "NSTextCheckingTypeReplacement")]
    pub const Replacement: Self = Self(1 << 8);
    #[doc(alias = "NSTextCheckingTypeCorrection")]
    pub const Correction: Self = Self(1 << 9);
    #[doc(alias = "NSTextCheckingTypeRegularExpression")]
    pub const RegularExpression: Self = Self(1 << 10);
    #[doc(alias = "NSTextCheckingTypePhoneNumber")]
    pub const PhoneNumber: Self = Self(1 << 11);
    #[doc(alias = "NSTextCheckingTypeTransitInformation")]
    pub const TransitInformation: Self = Self(1 << 12);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTextCheckingType {
    const ENCODING: Encoding = u64::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTextCheckingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub type NSTextCheckingTypes = u64;

pub const NSTextCheckingAllSystemTypes: NSTextCheckingTypes = 0xffffffff;
pub const NSTextCheckingAllCustomTypes: NSTextCheckingTypes = 0xffffffff << 32;
pub const NSTextCheckingAllTypes: NSTextCheckingTypes =
    NSTextCheckingAllSystemTypes | NSTextCheckingAllCustomTypes;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSTextCheckingKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextCheckingResult;

    unsafe impl ClassType for NSTextCheckingResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSTextCheckingResult {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSTextCheckingResult {}

unsafe impl NSObjectProtocol for NSTextCheckingResult {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSTextCheckingResult {}

extern_methods!(
    unsafe impl NSTextCheckingResult {
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSTextCheckingType;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextCheckingResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSTextCheckingResultOptional
    unsafe impl NSTextCheckingResult {
        #[cfg(feature = "Foundation_NSOrthography")]
        #[method_id(@__retain_semantics Other orthography)]
        pub unsafe fn orthography(&self) -> Option<Id<NSOrthography>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other grammarDetails)]
        pub unsafe fn grammarDetails(
            &self,
        ) -> Option<Id<NSArray<NSDictionary<NSString, AnyObject>>>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other components)]
        pub unsafe fn components(&self) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other replacementString)]
        pub unsafe fn replacementString(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other alternativeStrings)]
        pub unsafe fn alternativeStrings(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSRegularExpression")]
        #[method_id(@__retain_semantics Other regularExpression)]
        pub unsafe fn regularExpression(&self) -> Option<Id<NSRegularExpression>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Id<NSString>>;

        #[method(numberOfRanges)]
        pub unsafe fn numberOfRanges(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(rangeAtIndex:)]
        pub unsafe fn rangeAtIndex(&self, idx: NSUInteger) -> NSRange;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method(rangeWithName:)]
        pub unsafe fn rangeWithName(&self, name: &NSString) -> NSRange;

        #[method_id(@__retain_semantics Other resultByAdjustingRangesWithOffset:)]
        pub unsafe fn resultByAdjustingRangesWithOffset(
            &self,
            offset: NSInteger,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addressComponents)]
        pub unsafe fn addressComponents(
            &self,
        ) -> Option<Id<NSDictionary<NSTextCheckingKey, NSString>>>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingNameKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingJobTitleKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingOrganizationKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingStreetKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingCityKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingStateKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingZIPKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingCountryKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingPhoneKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingAirlineKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextCheckingFlightKey: &'static NSTextCheckingKey;
}

extern_methods!(
    /// NSTextCheckingResultCreation
    unsafe impl NSTextCheckingResult {
        #[cfg(all(feature = "Foundation_NSOrthography", feature = "Foundation_NSRange"))]
        #[method_id(@__retain_semantics Other orthographyCheckingResultWithRange:orthography:)]
        pub unsafe fn orthographyCheckingResultWithRange_orthography(
            range: NSRange,
            orthography: &NSOrthography,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(feature = "Foundation_NSRange")]
        #[method_id(@__retain_semantics Other spellCheckingResultWithRange:)]
        pub unsafe fn spellCheckingResultWithRange(range: NSRange) -> Id<NSTextCheckingResult>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other grammarCheckingResultWithRange:details:)]
        pub unsafe fn grammarCheckingResultWithRange_details(
            range: NSRange,
            details: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSRange"))]
        #[method_id(@__retain_semantics Other dateCheckingResultWithRange:date:)]
        pub unsafe fn dateCheckingResultWithRange_date(
            range: NSRange,
            date: &NSDate,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSTimeZone"
        ))]
        #[method_id(@__retain_semantics Other dateCheckingResultWithRange:date:timeZone:duration:)]
        pub unsafe fn dateCheckingResultWithRange_date_timeZone_duration(
            range: NSRange,
            date: &NSDate,
            time_zone: &NSTimeZone,
            duration: NSTimeInterval,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other addressCheckingResultWithRange:components:)]
        pub unsafe fn addressCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other linkCheckingResultWithRange:URL:)]
        pub unsafe fn linkCheckingResultWithRange_URL(
            range: NSRange,
            url: &NSURL,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other quoteCheckingResultWithRange:replacementString:)]
        pub unsafe fn quoteCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dashCheckingResultWithRange:replacementString:)]
        pub unsafe fn dashCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other replacementCheckingResultWithRange:replacementString:)]
        pub unsafe fn replacementCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other correctionCheckingResultWithRange:replacementString:)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other correctionCheckingResultWithRange:replacementString:alternativeStrings:)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString_alternativeStrings(
            range: NSRange,
            replacement_string: &NSString,
            alternative_strings: &NSArray<NSString>,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(
            feature = "Foundation_NSRange",
            feature = "Foundation_NSRegularExpression"
        ))]
        #[method_id(@__retain_semantics Other regularExpressionCheckingResultWithRanges:count:regularExpression:)]
        pub unsafe fn regularExpressionCheckingResultWithRanges_count_regularExpression(
            ranges: NSRangePointer,
            count: NSUInteger,
            regular_expression: &NSRegularExpression,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other phoneNumberCheckingResultWithRange:phoneNumber:)]
        pub unsafe fn phoneNumberCheckingResultWithRange_phoneNumber(
            range: NSRange,
            phone_number: &NSString,
        ) -> Id<NSTextCheckingResult>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other transitInformationCheckingResultWithRange:components:)]
        pub unsafe fn transitInformationCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Id<NSTextCheckingResult>;
    }
);
