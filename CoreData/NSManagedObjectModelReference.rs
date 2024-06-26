//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSManagedObjectModelReference;

    unsafe impl ClassType for NSManagedObjectModelReference {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSManagedObjectModelReference {}

extern_methods!(
    unsafe impl NSManagedObjectModelReference {
        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other resolvedModel)]
        pub unsafe fn resolvedModel(&self) -> Id<NSManagedObjectModel>;

        #[method_id(@__retain_semantics Other versionChecksum)]
        pub unsafe fn versionChecksum(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Init initWithModel:versionChecksum:)]
        pub unsafe fn initWithModel_versionChecksum(
            this: Allocated<Self>,
            model: &NSManagedObjectModel,
            version_checksum: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithFileURL:versionChecksum:)]
        pub unsafe fn initWithFileURL_versionChecksum(
            this: Allocated<Self>,
            file_url: &NSURL,
            version_checksum: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithEntityVersionHashes:inBundle:versionChecksum:)]
        pub unsafe fn initWithEntityVersionHashes_inBundle_versionChecksum(
            this: Allocated<Self>,
            version_hash: &NSDictionary,
            bundle: Option<&NSBundle>,
            version_checksum: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithName:inBundle:versionChecksum:)]
        pub unsafe fn initWithName_inBundle_versionChecksum(
            this: Allocated<Self>,
            model_name: &NSString,
            bundle: Option<&NSBundle>,
            version_checksum: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSManagedObjectModelReference {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
