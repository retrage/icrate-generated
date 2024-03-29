//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Virtualization_VZDirectoryShare")]
    pub struct VZMultipleDirectoryShare;

    #[cfg(feature = "Virtualization_VZDirectoryShare")]
    unsafe impl ClassType for VZMultipleDirectoryShare {
        #[inherits(NSObject)]
        type Super = VZDirectoryShare;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Virtualization_VZDirectoryShare")]
unsafe impl NSObjectProtocol for VZMultipleDirectoryShare {}

extern_methods!(
    #[cfg(feature = "Virtualization_VZDirectoryShare")]
    unsafe impl VZMultipleDirectoryShare {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Virtualization_VZSharedDirectory"
        ))]
        #[method_id(@__retain_semantics Init initWithDirectories:)]
        pub unsafe fn initWithDirectories(
            this: Allocated<Self>,
            directories: &NSDictionary<NSString, VZSharedDirectory>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Virtualization_VZSharedDirectory"
        ))]
        #[method_id(@__retain_semantics Other directories)]
        pub unsafe fn directories(&self) -> Id<NSDictionary<NSString, VZSharedDirectory>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(validateName:error:_)]
        pub unsafe fn validateName_error(name: &NSString) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other canonicalizedNameFromName:)]
        pub unsafe fn canonicalizedNameFromName(name: &NSString) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZDirectoryShare`
    #[cfg(feature = "Virtualization_VZDirectoryShare")]
    unsafe impl VZMultipleDirectoryShare {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
