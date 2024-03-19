//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEnergyFormatterUnit(pub NSInteger);
impl NSEnergyFormatterUnit {
    #[doc(alias = "NSEnergyFormatterUnitJoule")]
    pub const Joule: Self = Self(11);
    #[doc(alias = "NSEnergyFormatterUnitKilojoule")]
    pub const Kilojoule: Self = Self(14);
    #[doc(alias = "NSEnergyFormatterUnitCalorie")]
    pub const Calorie: Self = Self((7 << 8) + 1);
    #[doc(alias = "NSEnergyFormatterUnitKilocalorie")]
    pub const Kilocalorie: Self = Self((7 << 8) + 2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSEnergyFormatterUnit {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSEnergyFormatterUnit {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFormatter")]
    pub struct NSEnergyFormatter;

    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl ClassType for NSEnergyFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for NSEnergyFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for NSEnergyFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl NSObjectProtocol for NSEnergyFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSEnergyFormatter {
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

        #[method(isForFoodEnergyUse)]
        pub unsafe fn isForFoodEnergyUse(&self) -> bool;

        #[method(setForFoodEnergyUse:)]
        pub unsafe fn setForFoodEnergyUse(&self, for_food_energy_use: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromValue:unit:)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromJoules:)]
        pub unsafe fn stringFromJoules(&self, number_in_joules: c_double) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromValue:unit:)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromJoules:usedUnit:)]
        pub unsafe fn unitStringFromJoules_usedUnit(
            &self,
            number_in_joules: c_double,
            unitp: *mut NSEnergyFormatterUnit,
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
    unsafe impl NSEnergyFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
