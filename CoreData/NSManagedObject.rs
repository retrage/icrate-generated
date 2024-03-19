//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSnapshotEventType(pub NSUInteger);
impl NSSnapshotEventType {
    pub const NSSnapshotEventUndoInsertion: Self = Self(1 << 1);
    pub const NSSnapshotEventUndoDeletion: Self = Self(1 << 2);
    pub const NSSnapshotEventUndoUpdate: Self = Self(1 << 3);
    pub const NSSnapshotEventRollback: Self = Self(1 << 4);
    pub const NSSnapshotEventRefresh: Self = Self(1 << 5);
    pub const NSSnapshotEventMergePolicy: Self = Self(1 << 6);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSSnapshotEventType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSSnapshotEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSManagedObject;

    unsafe impl ClassType for NSManagedObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSManagedObject {}

extern_methods!(
    unsafe impl NSManagedObject {
        #[method(contextShouldIgnoreUnmodeledPropertyChanges)]
        pub unsafe fn contextShouldIgnoreUnmodeledPropertyChanges() -> bool;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity_class() -> Id<NSEntityDescription>;

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "CoreData_NSPersistentStoreRequest"
        ))]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest() -> Id<NSFetchRequest>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObjectContext"
        ))]
        #[method_id(@__retain_semantics Init initWithEntity:insertIntoManagedObjectContext:)]
        pub unsafe fn initWithEntity_insertIntoManagedObjectContext(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
            context: Option<&NSManagedObjectContext>,
        ) -> Id<NSManagedObject>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Init initWithContext:)]
        pub unsafe fn initWithContext(
            this: Allocated<Self>,
            moc: &NSManagedObjectContext,
        ) -> Id<Self>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Id<NSEntityDescription>;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other objectID)]
        pub unsafe fn objectID(&self) -> Id<NSManagedObjectID>;

        #[method(isInserted)]
        pub unsafe fn isInserted(&self) -> bool;

        #[method(isUpdated)]
        pub unsafe fn isUpdated(&self) -> bool;

        #[method(isDeleted)]
        pub unsafe fn isDeleted(&self) -> bool;

        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[method(hasPersistentChangedValues)]
        pub unsafe fn hasPersistentChangedValues(&self) -> bool;

        #[method(isFault)]
        pub unsafe fn isFault(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasFaultForRelationshipNamed:)]
        pub unsafe fn hasFaultForRelationshipNamed(&self, key: &NSString) -> bool;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectIDsForRelationshipNamed:)]
        pub unsafe fn objectIDsForRelationshipNamed(
            &self,
            key: &NSString,
        ) -> Id<NSArray<NSManagedObjectID>>;

        #[method(faultingState)]
        pub unsafe fn faultingState(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(willAccessValueForKey:)]
        pub unsafe fn willAccessValueForKey(&self, key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(didAccessValueForKey:)]
        pub unsafe fn didAccessValueForKey(&self, key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(willChangeValueForKey:)]
        pub unsafe fn willChangeValueForKey(&self, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(didChangeValueForKey:)]
        pub unsafe fn didChangeValueForKey(&self, key: &NSString);

        #[cfg(all(
            feature = "Foundation_NSKeyValueObserving",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString"
        ))]
        #[method(willChangeValueForKey:withSetMutation:usingObjects:)]
        pub unsafe fn willChangeValueForKey_withSetMutation_usingObjects(
            &self,
            in_key: &NSString,
            in_mutation_kind: NSKeyValueSetMutationKind,
            in_objects: &NSSet,
        );

        #[cfg(all(
            feature = "Foundation_NSKeyValueObserving",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString"
        ))]
        #[method(didChangeValueForKey:withSetMutation:usingObjects:)]
        pub unsafe fn didChangeValueForKey_withSetMutation_usingObjects(
            &self,
            in_key: &NSString,
            in_mutation_kind: NSKeyValueSetMutationKind,
            in_objects: &NSSet,
        );

        #[method(awakeFromFetch)]
        pub unsafe fn awakeFromFetch(&self);

        #[method(awakeFromInsert)]
        pub unsafe fn awakeFromInsert(&self);

        #[method(awakeFromSnapshotEvents:)]
        pub unsafe fn awakeFromSnapshotEvents(&self, flags: NSSnapshotEventType);

        #[method(prepareForDeletion)]
        pub unsafe fn prepareForDeletion(&self);

        #[method(willSave)]
        pub unsafe fn willSave(&self);

        #[method(didSave)]
        pub unsafe fn didSave(&self);

        #[method(willTurnIntoFault)]
        pub unsafe fn willTurnIntoFault(&self);

        #[method(didTurnIntoFault)]
        pub unsafe fn didTurnIntoFault(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other primitiveValueForKey:)]
        pub unsafe fn primitiveValueForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPrimitiveValue:forKey:)]
        pub unsafe fn setPrimitiveValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other committedValuesForKeys:)]
        pub unsafe fn committedValuesForKeys(
            &self,
            keys: Option<&NSArray<NSString>>,
        ) -> Id<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other changedValues)]
        pub unsafe fn changedValues(&self) -> Id<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other changedValuesForCurrentEvent)]
        pub unsafe fn changedValuesForCurrentEvent(&self) -> Id<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(validateValue:forKey:error:_)]
        pub unsafe fn validateValue_forKey_error(
            &self,
            value: &mut Option<Id<AnyObject>>,
            key: &NSString,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateForDelete:_)]
        pub unsafe fn validateForDelete(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateForInsert:_)]
        pub unsafe fn validateForInsert(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateForUpdate:_)]
        pub unsafe fn validateForUpdate(&self) -> Result<(), Id<NSError>>;

        #[method(setObservationInfo:)]
        pub unsafe fn setObservationInfo(&self, in_observation_info: *mut c_void);

        #[method(observationInfo)]
        pub unsafe fn observationInfo(&self) -> *mut c_void;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSManagedObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
