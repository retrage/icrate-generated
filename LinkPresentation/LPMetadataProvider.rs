//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::LinkPresentation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LPMetadataProvider;

    unsafe impl ClassType for LPMetadataProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LPMetadataProvider {}

extern_methods!(
    unsafe impl LPMetadataProvider {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL",
            feature = "LinkPresentation_LPLinkMetadata"
        ))]
        #[method(startFetchingMetadataForURL:completionHandler:)]
        pub unsafe fn startFetchingMetadataForURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut LPLinkMetadata, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSURLRequest",
            feature = "LinkPresentation_LPLinkMetadata"
        ))]
        #[method(startFetchingMetadataForRequest:completionHandler:)]
        pub unsafe fn startFetchingMetadataForRequest_completionHandler(
            &self,
            request: &NSURLRequest,
            completion_handler: &Block<dyn Fn(*mut LPLinkMetadata, *mut NSError)>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(shouldFetchSubresources)]
        pub unsafe fn shouldFetchSubresources(&self) -> bool;

        #[method(setShouldFetchSubresources:)]
        pub unsafe fn setShouldFetchSubresources(&self, should_fetch_subresources: bool);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(timeout)]
        pub unsafe fn timeout(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setTimeout:)]
        pub unsafe fn setTimeout(&self, timeout: NSTimeInterval);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LPMetadataProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
