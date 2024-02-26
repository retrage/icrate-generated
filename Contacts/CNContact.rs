//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNContactType {
        #[doc(alias = "CNContactTypePerson")]
        Person = 0,
        #[doc(alias = "CNContactTypeOrganization")]
        Organization = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNContactSortOrder {
        #[doc(alias = "CNContactSortOrderNone")]
        None = 0,
        #[doc(alias = "CNContactSortOrderUserDefault")]
        UserDefault = 1,
        #[doc(alias = "CNContactSortOrderGivenName")]
        GivenName = 2,
        #[doc(alias = "CNContactSortOrderFamilyName")]
        FamilyName = 3,
    }
);

extern_protocol!(
    #[cfg(feature = "Foundation_NSObject")]
    pub unsafe trait CNKeyDescriptor: NSCopying + NSObjectProtocol + NSSecureCoding {}

    #[cfg(feature = "Foundation_NSObject")]
    unsafe impl ProtocolType for dyn CNKeyDescriptor {}
);

#[cfg(all(feature = "Foundation_NSObject", feature = "Foundation_NSString"))]
unsafe impl CNKeyDescriptor for NSString {}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContact;

    unsafe impl ClassType for CNContact {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CNContact {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CNContact {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSMutableCopying for CNContact {}

unsafe impl NSObjectProtocol for CNContact {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CNContact {}

extern_methods!(
    unsafe impl CNContact {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(contactType)]
        pub unsafe fn contactType(&self) -> CNContactType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other namePrefix)]
        pub unsafe fn namePrefix(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other givenName)]
        pub unsafe fn givenName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other middleName)]
        pub unsafe fn middleName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other familyName)]
        pub unsafe fn familyName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other previousFamilyName)]
        pub unsafe fn previousFamilyName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nameSuffix)]
        pub unsafe fn nameSuffix(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nickname)]
        pub unsafe fn nickname(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other organizationName)]
        pub unsafe fn organizationName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other departmentName)]
        pub unsafe fn departmentName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other jobTitle)]
        pub unsafe fn jobTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticGivenName)]
        pub unsafe fn phoneticGivenName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticMiddleName)]
        pub unsafe fn phoneticMiddleName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticFamilyName)]
        pub unsafe fn phoneticFamilyName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticOrganizationName)]
        pub unsafe fn phoneticOrganizationName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other note)]
        pub unsafe fn note(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageData)]
        pub unsafe fn imageData(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other thumbnailImageData)]
        pub unsafe fn thumbnailImageData(&self) -> Option<Id<NSData>>;

        #[method(imageDataAvailable)]
        pub unsafe fn imageDataAvailable(&self) -> bool;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Contacts_CNPhoneNumber",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other phoneNumbers)]
        pub unsafe fn phoneNumbers(&self) -> Id<NSArray<CNLabeledValue<CNPhoneNumber>>>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other emailAddresses)]
        pub unsafe fn emailAddresses(&self) -> Id<NSArray<CNLabeledValue<NSString>>>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Contacts_CNPostalAddress",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other postalAddresses)]
        pub unsafe fn postalAddresses(&self) -> Id<NSArray<CNLabeledValue<CNPostalAddress>>>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other urlAddresses)]
        pub unsafe fn urlAddresses(&self) -> Id<NSArray<CNLabeledValue<NSString>>>;

        #[cfg(all(
            feature = "Contacts_CNContactRelation",
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other contactRelations)]
        pub unsafe fn contactRelations(&self) -> Id<NSArray<CNLabeledValue<CNContactRelation>>>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Contacts_CNSocialProfile",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other socialProfiles)]
        pub unsafe fn socialProfiles(&self) -> Id<NSArray<CNLabeledValue<CNSocialProfile>>>;

        #[cfg(all(
            feature = "Contacts_CNInstantMessageAddress",
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other instantMessageAddresses)]
        pub unsafe fn instantMessageAddresses(
            &self,
        ) -> Id<NSArray<CNLabeledValue<CNInstantMessageAddress>>>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other birthday)]
        pub unsafe fn birthday(&self) -> Option<Id<NSDateComponents>>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other nonGregorianBirthday)]
        pub unsafe fn nonGregorianBirthday(&self) -> Option<Id<NSDateComponents>>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSCalendar"
        ))]
        #[method_id(@__retain_semantics Other dates)]
        pub unsafe fn dates(&self) -> Id<NSArray<CNLabeledValue<NSDateComponents>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isKeyAvailable:)]
        pub unsafe fn isKeyAvailable(&self, key: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSObject"))]
        #[method(areKeysAvailable:)]
        pub unsafe fn areKeysAvailable(
            &self,
            key_descriptors: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:)]
        pub unsafe fn localizedStringForKey(key: &NSString) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSObjCRuntime")]
        #[method(comparatorForNameSortOrder:)]
        pub unsafe fn comparatorForNameSortOrder(sort_order: CNContactSortOrder) -> NSComparator;

        #[cfg(feature = "Foundation_NSObject")]
        #[method_id(@__retain_semantics Other descriptorForAllComparatorKeys)]
        pub unsafe fn descriptorForAllComparatorKeys() -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isUnifiedWithContactWithIdentifier:)]
        pub unsafe fn isUnifiedWithContactWithIdentifier(
            &self,
            contact_identifier: &NSString,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContact {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPropertyNotFetchedExceptionName: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactIdentifierKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactNamePrefixKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactGivenNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactMiddleNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactFamilyNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPreviousFamilyNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactNameSuffixKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactNicknameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactOrganizationNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactDepartmentNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactJobTitleKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPhoneticGivenNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPhoneticMiddleNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPhoneticFamilyNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPhoneticOrganizationNameKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactBirthdayKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactNonGregorianBirthdayKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactNoteKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactImageDataKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactThumbnailImageDataKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactImageDataAvailableKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactTypeKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPhoneNumbersKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactEmailAddressesKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPostalAddressesKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactDatesKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactUrlAddressesKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactRelationsKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactSocialProfilesKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactInstantMessageAddressesKey: &'static NSString);
