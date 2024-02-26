//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNFetchRequest")]
    pub struct CNChangeHistoryFetchRequest;

    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl ClassType for CNChangeHistoryFetchRequest {
        #[inherits(NSObject)]
        type Super = CNFetchRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Contacts_CNFetchRequest", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for CNChangeHistoryFetchRequest {}

#[cfg(feature = "Contacts_CNFetchRequest")]
unsafe impl NSObjectProtocol for CNChangeHistoryFetchRequest {}

#[cfg(all(feature = "Contacts_CNFetchRequest", feature = "Foundation_NSObject"))]
unsafe impl NSSecureCoding for CNChangeHistoryFetchRequest {}

extern_methods!(
    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl CNChangeHistoryFetchRequest {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other startingToken)]
        pub unsafe fn startingToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setStartingToken:)]
        pub unsafe fn setStartingToken(&self, starting_token: Option<&NSData>);

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObject"
        ))]
        #[method_id(@__retain_semantics Other additionalContactKeyDescriptors)]
        pub unsafe fn additionalContactKeyDescriptors(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn CNKeyDescriptor>>>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObject"
        ))]
        #[method(setAdditionalContactKeyDescriptors:)]
        pub unsafe fn setAdditionalContactKeyDescriptors(
            &self,
            additional_contact_key_descriptors: Option<
                &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
            >,
        );

        #[method(shouldUnifyResults)]
        pub unsafe fn shouldUnifyResults(&self) -> bool;

        #[method(setShouldUnifyResults:)]
        pub unsafe fn setShouldUnifyResults(&self, should_unify_results: bool);

        #[method(mutableObjects)]
        pub unsafe fn mutableObjects(&self) -> bool;

        #[method(setMutableObjects:)]
        pub unsafe fn setMutableObjects(&self, mutable_objects: bool);

        #[method(includeGroupChanges)]
        pub unsafe fn includeGroupChanges(&self) -> bool;

        #[method(setIncludeGroupChanges:)]
        pub unsafe fn setIncludeGroupChanges(&self, include_group_changes: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other excludedTransactionAuthors)]
        pub unsafe fn excludedTransactionAuthors(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setExcludedTransactionAuthors:)]
        pub unsafe fn setExcludedTransactionAuthors(
            &self,
            excluded_transaction_authors: Option<&NSArray<NSString>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNFetchRequest")]
    unsafe impl CNChangeHistoryFetchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
