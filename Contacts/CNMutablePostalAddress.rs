//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNPostalAddress")]
    pub struct CNMutablePostalAddress;

    #[cfg(feature = "CNPostalAddress")]
    unsafe impl ClassType for CNMutablePostalAddress {
        #[inherits(NSObject)]
        type Super = CNPostalAddress;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSCoding for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSCopying for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSMutableCopying for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSObjectProtocol for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSSecureCoding for CNMutablePostalAddress {}

extern_methods!(
    #[cfg(feature = "CNPostalAddress")]
    unsafe impl CNMutablePostalAddress {
        #[method_id(@__retain_semantics Other street)]
        pub unsafe fn street(&self) -> Id<NSString>;

        #[method(setStreet:)]
        pub unsafe fn setStreet(&self, street: &NSString);

        #[method_id(@__retain_semantics Other subLocality)]
        pub unsafe fn subLocality(&self) -> Id<NSString>;

        #[method(setSubLocality:)]
        pub unsafe fn setSubLocality(&self, sub_locality: &NSString);

        #[method_id(@__retain_semantics Other city)]
        pub unsafe fn city(&self) -> Id<NSString>;

        #[method(setCity:)]
        pub unsafe fn setCity(&self, city: &NSString);

        #[method_id(@__retain_semantics Other subAdministrativeArea)]
        pub unsafe fn subAdministrativeArea(&self) -> Id<NSString>;

        #[method(setSubAdministrativeArea:)]
        pub unsafe fn setSubAdministrativeArea(&self, sub_administrative_area: &NSString);

        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Id<NSString>;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: &NSString);

        #[method_id(@__retain_semantics Other postalCode)]
        pub unsafe fn postalCode(&self) -> Id<NSString>;

        #[method(setPostalCode:)]
        pub unsafe fn setPostalCode(&self, postal_code: &NSString);

        #[method_id(@__retain_semantics Other country)]
        pub unsafe fn country(&self) -> Id<NSString>;

        #[method(setCountry:)]
        pub unsafe fn setCountry(&self, country: &NSString);

        #[method_id(@__retain_semantics Other ISOCountryCode)]
        pub unsafe fn ISOCountryCode(&self) -> Id<NSString>;

        #[method(setISOCountryCode:)]
        pub unsafe fn setISOCountryCode(&self, iso_country_code: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CNPostalAddress")]
    unsafe impl CNMutablePostalAddress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
