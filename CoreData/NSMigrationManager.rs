//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMigrationManager;

    unsafe impl ClassType for NSMigrationManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSMigrationManager {}

extern_methods!(
    unsafe impl NSMigrationManager {
        #[cfg(feature = "CoreData_NSManagedObjectModel")]
        #[method_id(@__retain_semantics Init initWithSourceModel:destinationModel:)]
        pub unsafe fn initWithSourceModel_destinationModel(
            this: Allocated<Self>,
            source_model: &NSManagedObjectModel,
            destination_model: &NSManagedObjectModel,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreData_NSMappingModel",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(migrateStoreFromURL:type:options:withMappingModel:toDestinationURL:destinationType:destinationOptions:error:_)]
        pub unsafe fn migrateStoreFromURL_type_options_withMappingModel_toDestinationURL_destinationType_destinationOptions_error(
            &self,
            source_url: &NSURL,
            s_store_type: &NSString,
            s_options: Option<&NSDictionary>,
            mappings: Option<&NSMappingModel>,
            d_url: &NSURL,
            d_store_type: &NSString,
            d_options: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError>>;

        #[method(usesStoreSpecificMigrationManager)]
        pub unsafe fn usesStoreSpecificMigrationManager(&self) -> bool;

        #[method(setUsesStoreSpecificMigrationManager:)]
        pub unsafe fn setUsesStoreSpecificMigrationManager(
            &self,
            uses_store_specific_migration_manager: bool,
        );

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[cfg(feature = "CoreData_NSMappingModel")]
        #[method_id(@__retain_semantics Other mappingModel)]
        pub unsafe fn mappingModel(&self) -> Id<NSMappingModel>;

        #[cfg(feature = "CoreData_NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other sourceModel)]
        pub unsafe fn sourceModel(&self) -> Id<NSManagedObjectModel>;

        #[cfg(feature = "CoreData_NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other destinationModel)]
        pub unsafe fn destinationModel(&self) -> Id<NSManagedObjectModel>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other sourceContext)]
        pub unsafe fn sourceContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other destinationContext)]
        pub unsafe fn destinationContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSEntityMapping"
        ))]
        #[method_id(@__retain_semantics Other sourceEntityForEntityMapping:)]
        pub unsafe fn sourceEntityForEntityMapping(
            &self,
            m_entity: &NSEntityMapping,
        ) -> Option<Id<NSEntityDescription>>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSEntityMapping"
        ))]
        #[method_id(@__retain_semantics Other destinationEntityForEntityMapping:)]
        pub unsafe fn destinationEntityForEntityMapping(
            &self,
            m_entity: &NSEntityMapping,
        ) -> Option<Id<NSEntityDescription>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSManagedObject"
        ))]
        #[method(associateSourceInstance:withDestinationInstance:forEntityMapping:)]
        pub unsafe fn associateSourceInstance_withDestinationInstance_forEntityMapping(
            &self,
            source_instance: &NSManagedObject,
            destination_instance: &NSManagedObject,
            entity_mapping: &NSEntityMapping,
        );

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other destinationInstancesForEntityMappingNamed:sourceInstances:)]
        pub unsafe fn destinationInstancesForEntityMappingNamed_sourceInstances(
            &self,
            mapping_name: &NSString,
            source_instances: Option<&NSArray<NSManagedObject>>,
        ) -> Id<NSArray<NSManagedObject>>;

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other sourceInstancesForEntityMappingNamed:destinationInstances:)]
        pub unsafe fn sourceInstancesForEntityMappingNamed_destinationInstances(
            &self,
            mapping_name: &NSString,
            destination_instances: Option<&NSArray<NSManagedObject>>,
        ) -> Id<NSArray<NSManagedObject>>;

        #[cfg(feature = "CoreData_NSEntityMapping")]
        #[method_id(@__retain_semantics Other currentEntityMapping)]
        pub unsafe fn currentEntityMapping(&self) -> Id<NSEntityMapping>;

        #[method(migrationProgress)]
        pub unsafe fn migrationProgress(&self) -> c_float;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelMigrationWithError:)]
        pub unsafe fn cancelMigrationWithError(&self, error: &NSError);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMigrationManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
