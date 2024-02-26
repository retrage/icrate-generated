//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKUserIdentityLookupInfo;

    unsafe impl ClassType for CKUserIdentityLookupInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CKUserIdentityLookupInfo {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for CKUserIdentityLookupInfo {}

unsafe impl NSObjectProtocol for CKUserIdentityLookupInfo {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CKUserIdentityLookupInfo {}

extern_methods!(
    unsafe impl CKUserIdentityLookupInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithEmailAddress:)]
        pub unsafe fn initWithEmailAddress(
            this: Allocated<Self>,
            email_address: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithPhoneNumber:)]
        pub unsafe fn initWithPhoneNumber(
            this: Allocated<Self>,
            phone_number: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Init initWithUserRecordID:)]
        pub unsafe fn initWithUserRecordID(
            this: Allocated<Self>,
            user_record_id: &CKRecordID,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other lookupInfosWithEmails:)]
        pub unsafe fn lookupInfosWithEmails(
            emails: &NSArray<NSString>,
        ) -> Id<NSArray<CKUserIdentityLookupInfo>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other lookupInfosWithPhoneNumbers:)]
        pub unsafe fn lookupInfosWithPhoneNumbers(
            phone_numbers: &NSArray<NSString>,
        ) -> Id<NSArray<CKUserIdentityLookupInfo>>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other lookupInfosWithRecordIDs:)]
        pub unsafe fn lookupInfosWithRecordIDs(
            record_i_ds: &NSArray<CKRecordID>,
        ) -> Id<NSArray<CKUserIdentityLookupInfo>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other emailAddress)]
        pub unsafe fn emailAddress(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other userRecordID)]
        pub unsafe fn userRecordID(&self) -> Option<Id<CKRecordID>>;
    }
);
