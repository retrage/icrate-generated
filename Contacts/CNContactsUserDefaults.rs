//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactsUserDefaults;

    unsafe impl ClassType for CNContactsUserDefaults {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CNContactsUserDefaults {}

extern_methods!(
    unsafe impl CNContactsUserDefaults {
        #[method_id(@__retain_semantics Other sharedDefaults)]
        pub unsafe fn sharedDefaults() -> Id<Self>;

        #[cfg(feature = "Contacts_CNContact")]
        #[method(sortOrder)]
        pub unsafe fn sortOrder(&self) -> CNContactSortOrder;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactsUserDefaults {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
