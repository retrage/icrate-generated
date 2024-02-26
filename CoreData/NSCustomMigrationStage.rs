//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSMigrationStage")]
    pub struct NSCustomMigrationStage;

    #[cfg(feature = "CoreData_NSMigrationStage")]
    unsafe impl ClassType for NSCustomMigrationStage {
        #[inherits(NSObject)]
        type Super = NSMigrationStage;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSMigrationStage")]
unsafe impl NSObjectProtocol for NSCustomMigrationStage {}

extern_methods!(
    #[cfg(feature = "CoreData_NSMigrationStage")]
    unsafe impl NSCustomMigrationStage {
        #[cfg(feature = "CoreData_NSManagedObjectModelReference")]
        #[method_id(@__retain_semantics Other currentModel)]
        pub unsafe fn currentModel(&self) -> Id<NSManagedObjectModelReference>;

        #[cfg(feature = "CoreData_NSManagedObjectModelReference")]
        #[method_id(@__retain_semantics Other nextModel)]
        pub unsafe fn nextModel(&self) -> Id<NSManagedObjectModelReference>;

        #[cfg(all(
            feature = "CoreData_NSStagedMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(willMigrateHandler)]
        pub unsafe fn willMigrateHandler(
            &self,
        ) -> *mut Block<
            dyn Fn(
                NonNull<NSStagedMigrationManager>,
                NonNull<NSCustomMigrationStage>,
                *mut *mut NSError,
            ) -> Bool,
        >;

        #[cfg(all(
            feature = "CoreData_NSStagedMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(setWillMigrateHandler:)]
        pub unsafe fn setWillMigrateHandler(
            &self,
            will_migrate_handler: Option<
                &Block<
                    dyn Fn(
                        NonNull<NSStagedMigrationManager>,
                        NonNull<NSCustomMigrationStage>,
                        *mut *mut NSError,
                    ) -> Bool,
                >,
            >,
        );

        #[cfg(all(
            feature = "CoreData_NSStagedMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(didMigrateHandler)]
        pub unsafe fn didMigrateHandler(
            &self,
        ) -> *mut Block<
            dyn Fn(
                NonNull<NSStagedMigrationManager>,
                NonNull<NSCustomMigrationStage>,
                *mut *mut NSError,
            ) -> Bool,
        >;

        #[cfg(all(
            feature = "CoreData_NSStagedMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(setDidMigrateHandler:)]
        pub unsafe fn setDidMigrateHandler(
            &self,
            did_migrate_handler: Option<
                &Block<
                    dyn Fn(
                        NonNull<NSStagedMigrationManager>,
                        NonNull<NSCustomMigrationStage>,
                        *mut *mut NSError,
                    ) -> Bool,
                >,
            >,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CoreData_NSManagedObjectModelReference")]
        #[method_id(@__retain_semantics Init initWithCurrentModelReference:nextModelReference:)]
        pub unsafe fn initWithCurrentModelReference_nextModelReference(
            this: Allocated<Self>,
            current_model: &NSManagedObjectModelReference,
            next_model: &NSManagedObjectModelReference,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSMigrationStage")]
    unsafe impl NSCustomMigrationStage {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
