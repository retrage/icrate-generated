//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMediaGrouping(pub NSInteger);
impl MPMediaGrouping {
    #[doc(alias = "MPMediaGroupingTitle")]
    pub const Title: Self = Self(0);
    #[doc(alias = "MPMediaGroupingAlbum")]
    pub const Album: Self = Self(1);
    #[doc(alias = "MPMediaGroupingArtist")]
    pub const Artist: Self = Self(2);
    #[doc(alias = "MPMediaGroupingAlbumArtist")]
    pub const AlbumArtist: Self = Self(3);
    #[doc(alias = "MPMediaGroupingComposer")]
    pub const Composer: Self = Self(4);
    #[doc(alias = "MPMediaGroupingGenre")]
    pub const Genre: Self = Self(5);
    #[doc(alias = "MPMediaGroupingPlaylist")]
    pub const Playlist: Self = Self(6);
    #[doc(alias = "MPMediaGroupingPodcastTitle")]
    pub const PodcastTitle: Self = Self(7);
}

unsafe impl Encode for MPMediaGrouping {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPMediaGrouping {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaQuery;

    unsafe impl ClassType for MPMediaQuery {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MPMediaQuery {}

unsafe impl NSCopying for MPMediaQuery {}

unsafe impl NSObjectProtocol for MPMediaQuery {}

unsafe impl NSSecureCoding for MPMediaQuery {}

extern_methods!(
    unsafe impl MPMediaQuery {
        #[method_id(@__retain_semantics Init initWithFilterPredicates:)]
        pub unsafe fn initWithFilterPredicates(
            this: Allocated<Self>,
            filter_predicates: Option<&NSSet<MPMediaPredicate>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other filterPredicates)]
        pub unsafe fn filterPredicates(&self) -> Option<Id<NSSet<MPMediaPredicate>>>;

        #[method(setFilterPredicates:)]
        pub unsafe fn setFilterPredicates(
            &self,
            filter_predicates: Option<&NSSet<MPMediaPredicate>>,
        );

        #[method(addFilterPredicate:)]
        pub unsafe fn addFilterPredicate(&self, predicate: &MPMediaPredicate);

        #[method(removeFilterPredicate:)]
        pub unsafe fn removeFilterPredicate(&self, predicate: &MPMediaPredicate);

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItem"))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Option<Id<NSArray<MPMediaItem>>>;

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
        #[method_id(@__retain_semantics Other collections)]
        pub unsafe fn collections(&self) -> Option<Id<NSArray<MPMediaItemCollection>>>;

        #[method(groupingType)]
        pub unsafe fn groupingType(&self) -> MPMediaGrouping;

        #[method(setGroupingType:)]
        pub unsafe fn setGroupingType(&self, grouping_type: MPMediaGrouping);

        #[cfg(feature = "MPMediaQuerySection")]
        #[method_id(@__retain_semantics Other itemSections)]
        pub unsafe fn itemSections(&self) -> Option<Id<NSArray<MPMediaQuerySection>>>;

        #[cfg(feature = "MPMediaQuerySection")]
        #[method_id(@__retain_semantics Other collectionSections)]
        pub unsafe fn collectionSections(&self) -> Option<Id<NSArray<MPMediaQuerySection>>>;

        #[method_id(@__retain_semantics Other albumsQuery)]
        pub unsafe fn albumsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other artistsQuery)]
        pub unsafe fn artistsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other songsQuery)]
        pub unsafe fn songsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other playlistsQuery)]
        pub unsafe fn playlistsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other podcastsQuery)]
        pub unsafe fn podcastsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other audiobooksQuery)]
        pub unsafe fn audiobooksQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other compilationsQuery)]
        pub unsafe fn compilationsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other composersQuery)]
        pub unsafe fn composersQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other genresQuery)]
        pub unsafe fn genresQuery() -> Id<MPMediaQuery>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPMediaQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaPredicate;

    unsafe impl ClassType for MPMediaPredicate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MPMediaPredicate {}

unsafe impl NSObjectProtocol for MPMediaPredicate {}

unsafe impl NSSecureCoding for MPMediaPredicate {}

extern_methods!(
    unsafe impl MPMediaPredicate {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPMediaPredicate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMediaPredicateComparison(pub NSInteger);
impl MPMediaPredicateComparison {
    #[doc(alias = "MPMediaPredicateComparisonEqualTo")]
    pub const EqualTo: Self = Self(0);
    #[doc(alias = "MPMediaPredicateComparisonContains")]
    pub const Contains: Self = Self(1);
}

unsafe impl Encode for MPMediaPredicateComparison {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPMediaPredicateComparison {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMediaPropertyPredicate;

    unsafe impl ClassType for MPMediaPropertyPredicate {
        #[inherits(NSObject)]
        type Super = MPMediaPredicate;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MPMediaPropertyPredicate {}

unsafe impl NSObjectProtocol for MPMediaPropertyPredicate {}

unsafe impl NSSecureCoding for MPMediaPropertyPredicate {}

extern_methods!(
    unsafe impl MPMediaPropertyPredicate {
        #[method_id(@__retain_semantics Other predicateWithValue:forProperty:)]
        pub unsafe fn predicateWithValue_forProperty(
            value: Option<&AnyObject>,
            property: &NSString,
        ) -> Id<MPMediaPropertyPredicate>;

        #[method_id(@__retain_semantics Other predicateWithValue:forProperty:comparisonType:)]
        pub unsafe fn predicateWithValue_forProperty_comparisonType(
            value: Option<&AnyObject>,
            property: &NSString,
            comparison_type: MPMediaPredicateComparison,
        ) -> Id<MPMediaPropertyPredicate>;

        #[method_id(@__retain_semantics Other property)]
        pub unsafe fn property(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<AnyObject>>;

        #[method(comparisonType)]
        pub unsafe fn comparisonType(&self) -> MPMediaPredicateComparison;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPMediaPropertyPredicate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// MPMediaQueryAdditions
    #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItem"))]
    unsafe impl MPMediaItem {
        #[method_id(@__retain_semantics Other persistentIDPropertyForGroupingType:)]
        pub unsafe fn persistentIDPropertyForGroupingType(
            grouping_type: MPMediaGrouping,
        ) -> Id<NSString>;

        #[method_id(@__retain_semantics Other titlePropertyForGroupingType:)]
        pub unsafe fn titlePropertyForGroupingType(grouping_type: MPMediaGrouping) -> Id<NSString>;
    }
);
