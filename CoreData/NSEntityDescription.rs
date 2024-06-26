//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEntityDescription;

    unsafe impl ClassType for NSEntityDescription {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSEntityDescription {}

unsafe impl NSCopying for NSEntityDescription {}

unsafe impl NSFastEnumeration for NSEntityDescription {}

unsafe impl NSObjectProtocol for NSEntityDescription {}

extern_methods!(
    unsafe impl NSEntityDescription {
        #[cfg(feature = "NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other entityForName:inManagedObjectContext:)]
        pub unsafe fn entityForName_inManagedObjectContext(
            entity_name: &NSString,
            context: &NSManagedObjectContext,
        ) -> Option<Id<NSEntityDescription>>;

        #[cfg(all(feature = "NSManagedObject", feature = "NSManagedObjectContext"))]
        #[method_id(@__retain_semantics Other insertNewObjectForEntityForName:inManagedObjectContext:)]
        pub unsafe fn insertNewObjectForEntityForName_inManagedObjectContext(
            entity_name: &NSString,
            context: &NSManagedObjectContext,
        ) -> Id<NSManagedObject>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Id<NSManagedObjectModel>;

        #[method_id(@__retain_semantics Other managedObjectClassName)]
        pub unsafe fn managedObjectClassName(&self) -> Id<NSString>;

        #[method(setManagedObjectClassName:)]
        pub unsafe fn setManagedObjectClassName(
            &self,
            managed_object_class_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(isAbstract)]
        pub unsafe fn isAbstract(&self) -> bool;

        #[method(setAbstract:)]
        pub unsafe fn setAbstract(&self, r#abstract: bool);

        #[method_id(@__retain_semantics Other subentitiesByName)]
        pub unsafe fn subentitiesByName(&self) -> Id<NSDictionary<NSString, NSEntityDescription>>;

        #[method_id(@__retain_semantics Other subentities)]
        pub unsafe fn subentities(&self) -> Id<NSArray<NSEntityDescription>>;

        #[method(setSubentities:)]
        pub unsafe fn setSubentities(&self, subentities: &NSArray<NSEntityDescription>);

        #[method_id(@__retain_semantics Other superentity)]
        pub unsafe fn superentity(&self) -> Option<Id<NSEntityDescription>>;

        #[cfg(feature = "NSPropertyDescription")]
        #[method_id(@__retain_semantics Other propertiesByName)]
        pub unsafe fn propertiesByName(&self) -> Id<NSDictionary<NSString, NSPropertyDescription>>;

        #[cfg(feature = "NSPropertyDescription")]
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Id<NSArray<NSPropertyDescription>>;

        #[cfg(feature = "NSPropertyDescription")]
        #[method(setProperties:)]
        pub unsafe fn setProperties(&self, properties: &NSArray<NSPropertyDescription>);

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

        #[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
        #[method_id(@__retain_semantics Other attributesByName)]
        pub unsafe fn attributesByName(&self)
            -> Id<NSDictionary<NSString, NSAttributeDescription>>;

        #[cfg(all(
            feature = "NSPropertyDescription",
            feature = "NSRelationshipDescription"
        ))]
        #[method_id(@__retain_semantics Other relationshipsByName)]
        pub unsafe fn relationshipsByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSRelationshipDescription>>;

        #[cfg(all(
            feature = "NSPropertyDescription",
            feature = "NSRelationshipDescription"
        ))]
        #[method_id(@__retain_semantics Other relationshipsWithDestinationEntity:)]
        pub unsafe fn relationshipsWithDestinationEntity(
            &self,
            entity: &NSEntityDescription,
        ) -> Id<NSArray<NSRelationshipDescription>>;

        #[method(isKindOfEntity:)]
        pub unsafe fn isKindOfEntity(&self, entity: &NSEntityDescription) -> bool;

        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics Other versionHashModifier)]
        pub unsafe fn versionHashModifier(&self) -> Option<Id<NSString>>;

        #[method(setVersionHashModifier:)]
        pub unsafe fn setVersionHashModifier(&self, version_hash_modifier: Option<&NSString>);

        #[method_id(@__retain_semantics Other renamingIdentifier)]
        pub unsafe fn renamingIdentifier(&self) -> Option<Id<NSString>>;

        #[method(setRenamingIdentifier:)]
        pub unsafe fn setRenamingIdentifier(&self, renaming_identifier: Option<&NSString>);

        #[cfg(feature = "NSFetchIndexDescription")]
        #[method_id(@__retain_semantics Other indexes)]
        pub unsafe fn indexes(&self) -> Id<NSArray<NSFetchIndexDescription>>;

        #[cfg(feature = "NSFetchIndexDescription")]
        #[method(setIndexes:)]
        pub unsafe fn setIndexes(&self, indexes: &NSArray<NSFetchIndexDescription>);

        #[method_id(@__retain_semantics Other uniquenessConstraints)]
        pub unsafe fn uniquenessConstraints(&self) -> Id<NSArray<NSArray<AnyObject>>>;

        #[method(setUniquenessConstraints:)]
        pub unsafe fn setUniquenessConstraints(
            &self,
            uniqueness_constraints: &NSArray<NSArray<AnyObject>>,
        );

        #[deprecated = "Use NSEntityDescription.indexes instead"]
        #[method_id(@__retain_semantics Other compoundIndexes)]
        pub unsafe fn compoundIndexes(&self) -> Id<NSArray<NSArray<AnyObject>>>;

        #[deprecated = "Use NSEntityDescription.indexes instead"]
        #[method(setCompoundIndexes:)]
        pub unsafe fn setCompoundIndexes(&self, compound_indexes: &NSArray<NSArray<AnyObject>>);

        #[method_id(@__retain_semantics Other coreSpotlightDisplayNameExpression)]
        pub unsafe fn coreSpotlightDisplayNameExpression(&self) -> Id<NSExpression>;

        #[method(setCoreSpotlightDisplayNameExpression:)]
        pub unsafe fn setCoreSpotlightDisplayNameExpression(
            &self,
            core_spotlight_display_name_expression: &NSExpression,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSEntityDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
