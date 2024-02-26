//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNFetchRequest")]
    pub struct CNContactFetchRequest;

    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl ClassType for CNContactFetchRequest {
        #[inherits(NSObject)]
        type Super = CNFetchRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Contacts_CNFetchRequest", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for CNContactFetchRequest {}

#[cfg(feature = "Contacts_CNFetchRequest")]
unsafe impl NSObjectProtocol for CNContactFetchRequest {}

#[cfg(all(feature = "Contacts_CNFetchRequest", feature = "Foundation_NSObject"))]
unsafe impl NSSecureCoding for CNContactFetchRequest {}

extern_methods!(
    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl CNContactFetchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObject"
        ))]
        #[method_id(@__retain_semantics Init initWithKeysToFetch:)]
        pub unsafe fn initWithKeysToFetch(
            this: Allocated<Self>,
            keys_to_fetch: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObject"
        ))]
        #[method_id(@__retain_semantics Other keysToFetch)]
        pub unsafe fn keysToFetch(&self) -> Id<NSArray<ProtocolObject<dyn CNKeyDescriptor>>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObject"
        ))]
        #[method(setKeysToFetch:)]
        pub unsafe fn setKeysToFetch(
            &self,
            keys_to_fetch: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        );

        #[method(mutableObjects)]
        pub unsafe fn mutableObjects(&self) -> bool;

        #[method(setMutableObjects:)]
        pub unsafe fn setMutableObjects(&self, mutable_objects: bool);

        #[method(unifyResults)]
        pub unsafe fn unifyResults(&self) -> bool;

        #[method(setUnifyResults:)]
        pub unsafe fn setUnifyResults(&self, unify_results: bool);

        #[cfg(feature = "Contacts_CNContact")]
        #[method(sortOrder)]
        pub unsafe fn sortOrder(&self) -> CNContactSortOrder;

        #[cfg(feature = "Contacts_CNContact")]
        #[method(setSortOrder:)]
        pub unsafe fn setSortOrder(&self, sort_order: CNContactSortOrder);
    }
);
