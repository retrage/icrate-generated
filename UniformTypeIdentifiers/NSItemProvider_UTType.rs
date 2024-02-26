//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_category!(
    /// Category "UTType" on [`NSItemProvider`].
    #[doc(alias = "UTType")]
    pub unsafe trait NSItemProviderUTType {
        #[cfg(all(
            feature = "Foundation_NSItemProvider",
            feature = "Foundation_NSURL",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:contentType:openInPlace:coordinated:visibility:)]
        unsafe fn initWithContentsOfURL_contentType_openInPlace_coordinated_visibility(
            this: Allocated<Self>,
            file_url: &NSURL,
            content_type: Option<&UTType>,
            open_in_place: bool,
            coordinated: bool,
            visibility: NSItemProviderRepresentationVisibility,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSItemProvider",
            feature = "Foundation_NSProgress",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method(registerDataRepresentationForContentType:visibility:loadHandler:)]
        unsafe fn registerDataRepresentationForContentType_visibility_loadHandler(
            &self,
            content_type: &UTType,
            visibility: NSItemProviderRepresentationVisibility,
            load_handler: &Block<
                dyn Fn(NonNull<Block<dyn Fn(*mut NSData, *mut NSError)>>) -> *mut NSProgress,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSItemProvider",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSURL",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method(registerFileRepresentationForContentType:visibility:openInPlace:loadHandler:)]
        unsafe fn registerFileRepresentationForContentType_visibility_openInPlace_loadHandler(
            &self,
            content_type: &UTType,
            visibility: NSItemProviderRepresentationVisibility,
            open_in_place: bool,
            load_handler: &Block<
                dyn Fn(NonNull<Block<dyn Fn(*mut NSURL, Bool, *mut NSError)>>) -> *mut NSProgress,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method_id(@__retain_semantics Other registeredContentTypes)]
        unsafe fn registeredContentTypes(&self) -> Id<NSArray<UTType>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method_id(@__retain_semantics Other registeredContentTypesForOpenInPlace)]
        unsafe fn registeredContentTypesForOpenInPlace(&self) -> Id<NSArray<UTType>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method_id(@__retain_semantics Other registeredContentTypesConformingToContentType:)]
        unsafe fn registeredContentTypesConformingToContentType(
            &self,
            content_type: &UTType,
        ) -> Id<NSArray<UTType>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method_id(@__retain_semantics Other loadDataRepresentationForContentType:completionHandler:)]
        unsafe fn loadDataRepresentationForContentType_completionHandler(
            &self,
            content_type: &UTType,
            completion_handler: &Block<dyn Fn(*mut NSData, *mut NSError)>,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSURL",
            feature = "UniformTypeIdentifiers_UTType"
        ))]
        #[method_id(@__retain_semantics Other loadFileRepresentationForContentType:openInPlace:completionHandler:)]
        unsafe fn loadFileRepresentationForContentType_openInPlace_completionHandler(
            &self,
            content_type: &UTType,
            open_in_place: bool,
            completion_handler: &Block<dyn Fn(*mut NSURL, Bool, *mut NSError)>,
        ) -> Id<NSProgress>;
    }

    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProviderUTType for NSItemProvider {}
);
