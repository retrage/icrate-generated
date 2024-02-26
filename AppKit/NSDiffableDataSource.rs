//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceSnapshot<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl<SectionIdentifierType: ?Sized + IsIdCloneable, ItemIdentifierType: ?Sized + IsIdCloneable>
    NSCopying for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
{
}

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(numberOfSections)]
        pub unsafe fn numberOfSections(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sectionIdentifiers)]
        pub unsafe fn sectionIdentifiers(&self) -> Id<NSArray<SectionIdentifierType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Id<NSArray<ItemIdentifierType>>;

        #[method(numberOfItemsInSection:)]
        pub unsafe fn numberOfItemsInSection(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other itemIdentifiersInSectionWithIdentifier:)]
        pub unsafe fn itemIdentifiersInSectionWithIdentifier(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> Id<NSArray<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other sectionIdentifierForSectionContainingItemIdentifier:)]
        pub unsafe fn sectionIdentifierForSectionContainingItemIdentifier(
            &self,
            item_identifier: &ItemIdentifierType,
        ) -> Option<Id<SectionIdentifierType>>;

        #[method(indexOfItemIdentifier:)]
        pub unsafe fn indexOfItemIdentifier(
            &self,
            item_identifier: &ItemIdentifierType,
        ) -> NSInteger;

        #[method(indexOfSectionIdentifier:)]
        pub unsafe fn indexOfSectionIdentifier(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(appendItemsWithIdentifiers:)]
        pub unsafe fn appendItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(appendItemsWithIdentifiers:intoSectionWithIdentifier:)]
        pub unsafe fn appendItemsWithIdentifiers_intoSectionWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            section_identifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertItemsWithIdentifiers:beforeItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_beforeItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            item_identifier: &ItemIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertItemsWithIdentifiers:afterItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_afterItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            item_identifier: &ItemIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deleteItemsWithIdentifiers:)]
        pub unsafe fn deleteItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(deleteAllItems)]
        pub unsafe fn deleteAllItems(&self);

        #[method(moveItemWithIdentifier:beforeItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_beforeItemWithIdentifier(
            &self,
            from_identifier: &ItemIdentifierType,
            to_identifier: &ItemIdentifierType,
        );

        #[method(moveItemWithIdentifier:afterItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_afterItemWithIdentifier(
            &self,
            from_identifier: &ItemIdentifierType,
            to_identifier: &ItemIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(reloadItemsWithIdentifiers:)]
        pub unsafe fn reloadItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(appendSectionsWithIdentifiers:)]
        pub unsafe fn appendSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertSectionsWithIdentifiers:beforeSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_beforeSectionWithIdentifier(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
            to_section_identifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertSectionsWithIdentifiers:afterSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_afterSectionWithIdentifier(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
            to_section_identifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deleteSectionsWithIdentifiers:)]
        pub unsafe fn deleteSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );

        #[method(moveSectionWithIdentifier:beforeSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_beforeSectionWithIdentifier(
            &self,
            from_section_identifier: &SectionIdentifierType,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(moveSectionWithIdentifier:afterSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_afterSectionWithIdentifier(
            &self,
            from_section_identifier: &SectionIdentifierType,
            to_section_identifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(reloadSectionsWithIdentifiers:)]
        pub unsafe fn reloadSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(all(
    feature = "AppKit_NSCollectionView",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSIndexPath",
    feature = "Foundation_NSString"
))]
pub type NSCollectionViewDiffableDataSourceSupplementaryViewProvider = *mut Block<
    dyn Fn(NonNull<NSCollectionView>, NonNull<NSString>, NonNull<NSIndexPath>) -> *mut NSView,
>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewDiffableDataSource<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType
        for NSCollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSCollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSCollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Other snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Id<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other itemIdentifierForIndexPath:)]
        pub unsafe fn itemIdentifierForIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Id<ItemIdentifierType>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other indexPathForItemIdentifier:)]
        pub unsafe fn indexPathForItemIdentifier(
            &self,
            identifier: &ItemIdentifierType,
        ) -> Option<Id<NSIndexPath>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionView",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSIndexPath",
            feature = "Foundation_NSString"
        ))]
        #[method(supplementaryViewProvider)]
        pub unsafe fn supplementaryViewProvider(
            &self,
            mtm: MainThreadMarker,
        ) -> NSCollectionViewDiffableDataSourceSupplementaryViewProvider;

        #[cfg(all(
            feature = "AppKit_NSCollectionView",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSIndexPath",
            feature = "Foundation_NSString"
        ))]
        #[method(setSupplementaryViewProvider:)]
        pub unsafe fn setSupplementaryViewProvider(
            &self,
            supplementary_view_provider: NSCollectionViewDiffableDataSourceSupplementaryViewProvider,
        );
    }
);
