//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLengthFormatterUnit(pub NSInteger);
impl NSLengthFormatterUnit {
    #[doc(alias = "NSLengthFormatterUnitMillimeter")]
    pub const Millimeter: Self = Self(8);
    #[doc(alias = "NSLengthFormatterUnitCentimeter")]
    pub const Centimeter: Self = Self(9);
    #[doc(alias = "NSLengthFormatterUnitMeter")]
    pub const Meter: Self = Self(11);
    #[doc(alias = "NSLengthFormatterUnitKilometer")]
    pub const Kilometer: Self = Self(14);
    #[doc(alias = "NSLengthFormatterUnitInch")]
    pub const Inch: Self = Self((5 << 8) + 1);
    #[doc(alias = "NSLengthFormatterUnitFoot")]
    pub const Foot: Self = Self((5 << 8) + 2);
    #[doc(alias = "NSLengthFormatterUnitYard")]
    pub const Yard: Self = Self((5 << 8) + 3);
    #[doc(alias = "NSLengthFormatterUnitMile")]
    pub const Mile: Self = Self((5 << 8) + 4);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSLengthFormatterUnit {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSLengthFormatterUnit {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFormatter")]
    pub struct NSLengthFormatter;

    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl ClassType for NSLengthFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for NSLengthFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for NSLengthFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl NSObjectProtocol for NSLengthFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSLengthFormatter {
        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method_id(@__retain_semantics Other numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter>;

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, number_formatter: Option<&NSNumberFormatter>);

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: NSFormattingUnitStyle);

        #[method(isForPersonHeightUse)]
        pub unsafe fn isForPersonHeightUse(&self) -> bool;

        #[method(setForPersonHeightUse:)]
        pub unsafe fn setForPersonHeightUse(&self, for_person_height_use: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromValue:unit:)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSLengthFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromMeters:)]
        pub unsafe fn stringFromMeters(&self, number_in_meters: c_double) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromValue:unit:)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSLengthFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromMeters:usedUnit:)]
        pub unsafe fn unitStringFromMeters_usedUnit(
            &self,
            number_in_meters: c_double,
            unitp: *mut NSLengthFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSLengthFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
