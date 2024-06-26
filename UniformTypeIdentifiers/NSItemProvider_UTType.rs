//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "UTType" on [`NSItemProvider`].
    #[doc(alias = "UTType")]
    pub unsafe trait NSItemProviderUTType {
        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:contentType:openInPlace:coordinated:visibility:)]
        unsafe fn initWithContentsOfURL_contentType_openInPlace_coordinated_visibility(
            this: Allocated<Self>,
            file_url: &NSURL,
            content_type: Option<&UTType>,
            open_in_place: bool,
            coordinated: bool,
            visibility: NSItemProviderRepresentationVisibility,
        ) -> Id<Self>;

        #[cfg(all(feature = "UTType", feature = "block2"))]
        #[method(registerDataRepresentationForContentType:visibility:loadHandler:)]
        unsafe fn registerDataRepresentationForContentType_visibility_loadHandler(
            &self,
            content_type: &UTType,
            visibility: NSItemProviderRepresentationVisibility,
            load_handler: &Block<
                dyn Fn(NonNull<Block<dyn Fn(*mut NSData, *mut NSError)>>) -> *mut NSProgress,
            >,
        );

        #[cfg(all(feature = "UTType", feature = "block2"))]
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

        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Other registeredContentTypes)]
        unsafe fn registeredContentTypes(&self) -> Id<NSArray<UTType>>;

        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Other registeredContentTypesForOpenInPlace)]
        unsafe fn registeredContentTypesForOpenInPlace(&self) -> Id<NSArray<UTType>>;

        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Other registeredContentTypesConformingToContentType:)]
        unsafe fn registeredContentTypesConformingToContentType(
            &self,
            content_type: &UTType,
        ) -> Id<NSArray<UTType>>;

        #[cfg(all(feature = "UTType", feature = "block2"))]
        #[method_id(@__retain_semantics Other loadDataRepresentationForContentType:completionHandler:)]
        unsafe fn loadDataRepresentationForContentType_completionHandler(
            &self,
            content_type: &UTType,
            completion_handler: &Block<dyn Fn(*mut NSData, *mut NSError)>,
        ) -> Id<NSProgress>;

        #[cfg(all(feature = "UTType", feature = "block2"))]
        #[method_id(@__retain_semantics Other loadFileRepresentationForContentType:openInPlace:completionHandler:)]
        unsafe fn loadFileRepresentationForContentType_openInPlace_completionHandler(
            &self,
            content_type: &UTType,
            open_in_place: bool,
            completion_handler: &Block<dyn Fn(*mut NSURL, Bool, *mut NSError)>,
        ) -> Id<NSProgress>;
    }

    unsafe impl NSItemProviderUTType for NSItemProvider {}
);
