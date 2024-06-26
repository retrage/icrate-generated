//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSISO8601DateFormatOptions(pub NSUInteger);
impl NSISO8601DateFormatOptions {
    pub const NSISO8601DateFormatWithYear: Self = Self(1);
    pub const NSISO8601DateFormatWithMonth: Self = Self(2);
    pub const NSISO8601DateFormatWithWeekOfYear: Self = Self(4);
    pub const NSISO8601DateFormatWithDay: Self = Self(16);
    pub const NSISO8601DateFormatWithTime: Self = Self(32);
    pub const NSISO8601DateFormatWithTimeZone: Self = Self(64);
    pub const NSISO8601DateFormatWithSpaceBetweenDateAndTime: Self = Self(128);
    pub const NSISO8601DateFormatWithDashSeparatorInDate: Self = Self(256);
    pub const NSISO8601DateFormatWithColonSeparatorInTime: Self = Self(512);
    pub const NSISO8601DateFormatWithColonSeparatorInTimeZone: Self = Self(1024);
    pub const NSISO8601DateFormatWithFractionalSeconds: Self = Self(2048);
    pub const NSISO8601DateFormatWithFullDate: Self = Self(275);
    pub const NSISO8601DateFormatWithFullTime: Self = Self(1632);
    pub const NSISO8601DateFormatWithInternetDateTime: Self = Self(1907);
}

unsafe impl Encode for NSISO8601DateFormatOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSISO8601DateFormatOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSISO8601DateFormatter;

    #[cfg(feature = "NSFormatter")]
    unsafe impl ClassType for NSISO8601DateFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSISO8601DateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSISO8601DateFormatter {}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSISO8601DateFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSSecureCoding for NSISO8601DateFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSISO8601DateFormatter {
        #[cfg(feature = "NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone>;

        #[cfg(feature = "NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method(formatOptions)]
        pub unsafe fn formatOptions(&self) -> NSISO8601DateFormatOptions;

        #[method(setFormatOptions:)]
        pub unsafe fn setFormatOptions(&self, format_options: NSISO8601DateFormatOptions);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString>;

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dateFromString:)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "NSDate", feature = "NSString", feature = "NSTimeZone"))]
        #[method_id(@__retain_semantics Other stringFromDate:timeZone:formatOptions:)]
        pub unsafe fn stringFromDate_timeZone_formatOptions(
            date: &NSDate,
            time_zone: &NSTimeZone,
            format_options: NSISO8601DateFormatOptions,
        ) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSISO8601DateFormatter {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
