//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentContainer;

    unsafe impl ClassType for NSPersistentContainer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSPersistentContainer {}

unsafe impl Sync for NSPersistentContainer {}

unsafe impl NSObjectProtocol for NSPersistentContainer {}

extern_methods!(
    unsafe impl NSPersistentContainer {
        #[method_id(@__retain_semantics Other persistentContainerWithName:)]
        pub unsafe fn persistentContainerWithName(name: &NSString) -> Id<Self>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other persistentContainerWithName:managedObjectModel:)]
        pub unsafe fn persistentContainerWithName_managedObjectModel(
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other defaultDirectoryURL)]
        pub unsafe fn defaultDirectoryURL() -> Id<NSURL>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other viewContext)]
        pub unsafe fn viewContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Id<NSManagedObjectModel>;

        #[cfg(feature = "NSPersistentStoreCoordinator")]
        #[method_id(@__retain_semantics Other persistentStoreCoordinator)]
        pub unsafe fn persistentStoreCoordinator(&self) -> Id<NSPersistentStoreCoordinator>;

        #[cfg(feature = "NSPersistentStoreDescription")]
        #[method_id(@__retain_semantics Other persistentStoreDescriptions)]
        pub unsafe fn persistentStoreDescriptions(
            &self,
        ) -> Id<NSArray<NSPersistentStoreDescription>>;

        #[cfg(feature = "NSPersistentStoreDescription")]
        #[method(setPersistentStoreDescriptions:)]
        pub unsafe fn setPersistentStoreDescriptions(
            &self,
            persistent_store_descriptions: &NSArray<NSPersistentStoreDescription>,
        );

        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Id<Self>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Init initWithName:managedObjectModel:)]
        pub unsafe fn initWithName_managedObjectModel(
            this: Allocated<Self>,
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSPersistentStoreDescription", feature = "block2"))]
        #[method(loadPersistentStoresWithCompletionHandler:)]
        pub unsafe fn loadPersistentStoresWithCompletionHandler(
            &self,
            block: &Block<dyn Fn(NonNull<NSPersistentStoreDescription>, *mut NSError)>,
        );

        #[cfg(feature = "NSManagedObjectContext")]
        #[method_id(@__retain_semantics New newBackgroundContext)]
        pub unsafe fn newBackgroundContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(all(feature = "NSManagedObjectContext", feature = "block2"))]
        #[method(performBackgroundTask:)]
        pub unsafe fn performBackgroundTask(
            &self,
            block: &Block<dyn Fn(NonNull<NSManagedObjectContext>)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentContainer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
