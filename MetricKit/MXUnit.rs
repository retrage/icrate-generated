//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUnit")]
    pub struct MXUnitSignalBars;

    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl ClassType for MXUnitSignalBars {
        #[inherits(NSUnit, NSObject)]
        type Super = NSDimension;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSUnit"))]
unsafe impl NSCoding for MXUnitSignalBars {}

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSUnit"))]
unsafe impl NSCopying for MXUnitSignalBars {}

#[cfg(feature = "Foundation_NSUnit")]
unsafe impl NSObjectProtocol for MXUnitSignalBars {}

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSUnit"))]
unsafe impl NSSecureCoding for MXUnitSignalBars {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl MXUnitSignalBars {
        #[method_id(@__retain_semantics Other bars)]
        pub unsafe fn bars() -> Id<MXUnitSignalBars>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDimension`
    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl MXUnitSignalBars {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSymbol:converter:)]
        pub unsafe fn initWithSymbol_converter(
            this: Allocated<Self>,
            symbol: &NSString,
            converter: &NSUnitConverter,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other baseUnit)]
        pub unsafe fn baseUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUnit`
    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl MXUnitSignalBars {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSymbol:)]
        pub unsafe fn initWithSymbol(this: Allocated<Self>, symbol: &NSString) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUnit")]
    pub struct MXUnitAveragePixelLuminance;

    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl ClassType for MXUnitAveragePixelLuminance {
        #[inherits(NSUnit, NSObject)]
        type Super = NSDimension;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSUnit"))]
unsafe impl NSCoding for MXUnitAveragePixelLuminance {}

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSUnit"))]
unsafe impl NSCopying for MXUnitAveragePixelLuminance {}

#[cfg(feature = "Foundation_NSUnit")]
unsafe impl NSObjectProtocol for MXUnitAveragePixelLuminance {}

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSUnit"))]
unsafe impl NSSecureCoding for MXUnitAveragePixelLuminance {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl MXUnitAveragePixelLuminance {
        #[method_id(@__retain_semantics Other apl)]
        pub unsafe fn apl() -> Id<MXUnitAveragePixelLuminance>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDimension`
    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl MXUnitAveragePixelLuminance {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSymbol:converter:)]
        pub unsafe fn initWithSymbol_converter(
            this: Allocated<Self>,
            symbol: &NSString,
            converter: &NSUnitConverter,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other baseUnit)]
        pub unsafe fn baseUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUnit`
    #[cfg(feature = "Foundation_NSUnit")]
    unsafe impl MXUnitAveragePixelLuminance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSymbol:)]
        pub unsafe fn initWithSymbol(this: Allocated<Self>, symbol: &NSString) -> Id<Self>;
    }
);
