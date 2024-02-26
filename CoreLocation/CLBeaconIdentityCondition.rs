//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CLBeaconMajorValue = u16;

pub type CLBeaconMinorValue = u16;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLCondition")]
    pub struct CLBeaconIdentityCondition;

    #[cfg(feature = "CoreLocation_CLCondition")]
    unsafe impl ClassType for CLBeaconIdentityCondition {
        #[inherits(NSObject)]
        type Super = CLCondition;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CoreLocation_CLCondition", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for CLBeaconIdentityCondition {}

#[cfg(all(feature = "CoreLocation_CLCondition", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for CLBeaconIdentityCondition {}

#[cfg(feature = "CoreLocation_CLCondition")]
unsafe impl NSObjectProtocol for CLBeaconIdentityCondition {}

#[cfg(all(feature = "CoreLocation_CLCondition", feature = "Foundation_NSObject"))]
unsafe impl NSSecureCoding for CLBeaconIdentityCondition {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLCondition")]
    unsafe impl CLBeaconIdentityCondition {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithUUID:)]
        pub unsafe fn initWithUUID(this: Allocated<Self>, uuid: &NSUUID) -> Id<Self>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithUUID:major:)]
        pub unsafe fn initWithUUID_major(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithUUID:major:minor:)]
        pub unsafe fn initWithUUID_major_minor(
            this: Allocated<Self>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLCondition`
    #[cfg(feature = "CoreLocation_CLCondition")]
    unsafe impl CLBeaconIdentityCondition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
