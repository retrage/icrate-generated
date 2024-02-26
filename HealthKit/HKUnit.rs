//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKUnit;

    unsafe impl ClassType for HKUnit {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKUnit {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKUnit {}

unsafe impl NSObjectProtocol for HKUnit {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKUnit {}

extern_methods!(
    unsafe impl HKUnit {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitString)]
        pub unsafe fn unitString(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitFromString:)]
        pub unsafe fn unitFromString(string: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSMassFormatter")]
        #[method_id(@__retain_semantics Other unitFromMassFormatterUnit:)]
        pub unsafe fn unitFromMassFormatterUnit(
            mass_formatter_unit: NSMassFormatterUnit,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSMassFormatter")]
        #[method(massFormatterUnitFromUnit:)]
        pub unsafe fn massFormatterUnitFromUnit(unit: &HKUnit) -> NSMassFormatterUnit;

        #[cfg(feature = "Foundation_NSLengthFormatter")]
        #[method_id(@__retain_semantics Other unitFromLengthFormatterUnit:)]
        pub unsafe fn unitFromLengthFormatterUnit(
            length_formatter_unit: NSLengthFormatterUnit,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSLengthFormatter")]
        #[method(lengthFormatterUnitFromUnit:)]
        pub unsafe fn lengthFormatterUnitFromUnit(unit: &HKUnit) -> NSLengthFormatterUnit;

        #[cfg(feature = "Foundation_NSEnergyFormatter")]
        #[method_id(@__retain_semantics Other unitFromEnergyFormatterUnit:)]
        pub unsafe fn unitFromEnergyFormatterUnit(
            energy_formatter_unit: NSEnergyFormatterUnit,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSEnergyFormatter")]
        #[method(energyFormatterUnitFromUnit:)]
        pub unsafe fn energyFormatterUnitFromUnit(unit: &HKUnit) -> NSEnergyFormatterUnit;

        #[method(isNull)]
        pub unsafe fn isNull(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKMetricPrefix {
        #[doc(alias = "HKMetricPrefixNone")]
        None = 0,
        #[doc(alias = "HKMetricPrefixFemto")]
        Femto = 13,
        #[doc(alias = "HKMetricPrefixPico")]
        Pico = 1,
        #[doc(alias = "HKMetricPrefixNano")]
        Nano = 2,
        #[doc(alias = "HKMetricPrefixMicro")]
        Micro = 3,
        #[doc(alias = "HKMetricPrefixMilli")]
        Milli = 4,
        #[doc(alias = "HKMetricPrefixCenti")]
        Centi = 5,
        #[doc(alias = "HKMetricPrefixDeci")]
        Deci = 6,
        #[doc(alias = "HKMetricPrefixDeca")]
        Deca = 7,
        #[doc(alias = "HKMetricPrefixHecto")]
        Hecto = 8,
        #[doc(alias = "HKMetricPrefixKilo")]
        Kilo = 9,
        #[doc(alias = "HKMetricPrefixMega")]
        Mega = 10,
        #[doc(alias = "HKMetricPrefixGiga")]
        Giga = 11,
        #[doc(alias = "HKMetricPrefixTera")]
        Tera = 12,
    }
);

extern_methods!(
    /// Mass
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other gramUnitWithMetricPrefix:)]
        pub unsafe fn gramUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other gramUnit)]
        pub unsafe fn gramUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other ounceUnit)]
        pub unsafe fn ounceUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other poundUnit)]
        pub unsafe fn poundUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other stoneUnit)]
        pub unsafe fn stoneUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other moleUnitWithMetricPrefix:molarMass:)]
        pub unsafe fn moleUnitWithMetricPrefix_molarMass(
            prefix: HKMetricPrefix,
            grams_per_mole: c_double,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other moleUnitWithMolarMass:)]
        pub unsafe fn moleUnitWithMolarMass(grams_per_mole: c_double) -> Id<Self>;
    }
);

extern_methods!(
    /// Length
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other meterUnitWithMetricPrefix:)]
        pub unsafe fn meterUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other meterUnit)]
        pub unsafe fn meterUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other inchUnit)]
        pub unsafe fn inchUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other footUnit)]
        pub unsafe fn footUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other yardUnit)]
        pub unsafe fn yardUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other mileUnit)]
        pub unsafe fn mileUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Volume
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other literUnitWithMetricPrefix:)]
        pub unsafe fn literUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other literUnit)]
        pub unsafe fn literUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other fluidOunceUSUnit)]
        pub unsafe fn fluidOunceUSUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other fluidOunceImperialUnit)]
        pub unsafe fn fluidOunceImperialUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other pintUSUnit)]
        pub unsafe fn pintUSUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other pintImperialUnit)]
        pub unsafe fn pintImperialUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other cupUSUnit)]
        pub unsafe fn cupUSUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other cupImperialUnit)]
        pub unsafe fn cupImperialUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Pressure
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other pascalUnitWithMetricPrefix:)]
        pub unsafe fn pascalUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other pascalUnit)]
        pub unsafe fn pascalUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other millimeterOfMercuryUnit)]
        pub unsafe fn millimeterOfMercuryUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other centimeterOfWaterUnit)]
        pub unsafe fn centimeterOfWaterUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other atmosphereUnit)]
        pub unsafe fn atmosphereUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other decibelAWeightedSoundPressureLevelUnit)]
        pub unsafe fn decibelAWeightedSoundPressureLevelUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other inchesOfMercuryUnit)]
        pub unsafe fn inchesOfMercuryUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Time
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other secondUnitWithMetricPrefix:)]
        pub unsafe fn secondUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other secondUnit)]
        pub unsafe fn secondUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other minuteUnit)]
        pub unsafe fn minuteUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other hourUnit)]
        pub unsafe fn hourUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other dayUnit)]
        pub unsafe fn dayUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Energy
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other jouleUnitWithMetricPrefix:)]
        pub unsafe fn jouleUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other jouleUnit)]
        pub unsafe fn jouleUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other kilocalorieUnit)]
        pub unsafe fn kilocalorieUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other smallCalorieUnit)]
        pub unsafe fn smallCalorieUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other largeCalorieUnit)]
        pub unsafe fn largeCalorieUnit() -> Id<Self>;

        #[deprecated = "Use smallCalorieUnit or largeCalorieUnit, depending on which you mean"]
        #[method_id(@__retain_semantics Other calorieUnit)]
        pub unsafe fn calorieUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Temperature
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other degreeCelsiusUnit)]
        pub unsafe fn degreeCelsiusUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other degreeFahrenheitUnit)]
        pub unsafe fn degreeFahrenheitUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other kelvinUnit)]
        pub unsafe fn kelvinUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Conductance
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other siemenUnitWithMetricPrefix:)]
        pub unsafe fn siemenUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other siemenUnit)]
        pub unsafe fn siemenUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Pharmacology
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other internationalUnit)]
        pub unsafe fn internationalUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Scalar
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other countUnit)]
        pub unsafe fn countUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other percentUnit)]
        pub unsafe fn percentUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// HearingSensitivity
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other decibelHearingLevelUnit)]
        pub unsafe fn decibelHearingLevelUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Math
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other unitMultipliedByUnit:)]
        pub unsafe fn unitMultipliedByUnit(&self, unit: &HKUnit) -> Id<HKUnit>;

        #[method_id(@__retain_semantics Other unitDividedByUnit:)]
        pub unsafe fn unitDividedByUnit(&self, unit: &HKUnit) -> Id<HKUnit>;

        #[method_id(@__retain_semantics Other unitRaisedToPower:)]
        pub unsafe fn unitRaisedToPower(&self, power: NSInteger) -> Id<HKUnit>;

        #[method_id(@__retain_semantics Other reciprocalUnit)]
        pub unsafe fn reciprocalUnit(&self) -> Id<HKUnit>;
    }
);

extern_methods!(
    /// Frequency
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other hertzUnitWithMetricPrefix:)]
        pub unsafe fn hertzUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other hertzUnit)]
        pub unsafe fn hertzUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// ElectricPotentialDifference
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other voltUnitWithMetricPrefix:)]
        pub unsafe fn voltUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other voltUnit)]
        pub unsafe fn voltUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Power
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other wattUnitWithMetricPrefix:)]
        pub unsafe fn wattUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other wattUnit)]
        pub unsafe fn wattUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// OpticalPower
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other diopterUnit)]
        pub unsafe fn diopterUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other prismDiopterUnit)]
        pub unsafe fn prismDiopterUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Angle
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other radianAngleUnitWithMetricPrefix:)]
        pub unsafe fn radianAngleUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other radianAngleUnit)]
        pub unsafe fn radianAngleUnit() -> Id<Self>;

        #[method_id(@__retain_semantics Other degreeAngleUnit)]
        pub unsafe fn degreeAngleUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Illuminance
    unsafe impl HKUnit {
        #[method_id(@__retain_semantics Other luxUnitWithMetricPrefix:)]
        pub unsafe fn luxUnitWithMetricPrefix(prefix: HKMetricPrefix) -> Id<Self>;

        #[method_id(@__retain_semantics Other luxUnit)]
        pub unsafe fn luxUnit() -> Id<Self>;
    }
);
