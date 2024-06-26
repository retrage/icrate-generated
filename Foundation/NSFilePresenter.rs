//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSFilePresenter: NSObjectProtocol {
        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other presentedItemURL)]
        unsafe fn presentedItemURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "NSOperation")]
        #[method_id(@__retain_semantics Other presentedItemOperationQueue)]
        unsafe fn presentedItemOperationQueue(&self) -> Id<NSOperationQueue>;

        #[cfg(feature = "NSURL")]
        #[optional]
        #[method_id(@__retain_semantics Other primaryPresentedItemURL)]
        unsafe fn primaryPresentedItemURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "block2")]
        #[optional]
        #[method(relinquishPresentedItemToReader:)]
        unsafe fn relinquishPresentedItemToReader(
            &self,
            reader: &Block<dyn Fn(*mut Block<dyn Fn()>)>,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(relinquishPresentedItemToWriter:)]
        unsafe fn relinquishPresentedItemToWriter(
            &self,
            writer: &Block<dyn Fn(*mut Block<dyn Fn()>)>,
        );

        #[cfg(all(feature = "NSError", feature = "block2"))]
        #[optional]
        #[method(savePresentedItemChangesWithCompletionHandler:)]
        unsafe fn savePresentedItemChangesWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSError", feature = "block2"))]
        #[optional]
        #[method(accommodatePresentedItemDeletionWithCompletionHandler:)]
        unsafe fn accommodatePresentedItemDeletionWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSError", feature = "block2"))]
        #[optional]
        #[method(accommodatePresentedItemEvictionWithCompletionHandler:)]
        unsafe fn accommodatePresentedItemEvictionWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "NSURL")]
        #[optional]
        #[method(presentedItemDidMoveToURL:)]
        unsafe fn presentedItemDidMoveToURL(&self, new_url: &NSURL);

        #[optional]
        #[method(presentedItemDidChange)]
        unsafe fn presentedItemDidChange(&self);

        #[cfg(all(feature = "NSSet", feature = "NSString", feature = "NSURL"))]
        #[optional]
        #[method(presentedItemDidChangeUbiquityAttributes:)]
        unsafe fn presentedItemDidChangeUbiquityAttributes(
            &self,
            attributes: &NSSet<NSURLResourceKey>,
        );

        #[cfg(all(feature = "NSSet", feature = "NSString", feature = "NSURL"))]
        #[optional]
        #[method_id(@__retain_semantics Other observedPresentedItemUbiquityAttributes)]
        unsafe fn observedPresentedItemUbiquityAttributes(&self) -> Id<NSSet<NSURLResourceKey>>;

        #[cfg(feature = "NSFileVersion")]
        #[optional]
        #[method(presentedItemDidGainVersion:)]
        unsafe fn presentedItemDidGainVersion(&self, version: &NSFileVersion);

        #[cfg(feature = "NSFileVersion")]
        #[optional]
        #[method(presentedItemDidLoseVersion:)]
        unsafe fn presentedItemDidLoseVersion(&self, version: &NSFileVersion);

        #[cfg(feature = "NSFileVersion")]
        #[optional]
        #[method(presentedItemDidResolveConflictVersion:)]
        unsafe fn presentedItemDidResolveConflictVersion(&self, version: &NSFileVersion);

        #[cfg(all(feature = "NSError", feature = "NSURL", feature = "block2"))]
        #[optional]
        #[method(accommodatePresentedSubitemDeletionAtURL:completionHandler:)]
        unsafe fn accommodatePresentedSubitemDeletionAtURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "NSURL")]
        #[optional]
        #[method(presentedSubitemDidAppearAtURL:)]
        unsafe fn presentedSubitemDidAppearAtURL(&self, url: &NSURL);

        #[cfg(feature = "NSURL")]
        #[optional]
        #[method(presentedSubitemAtURL:didMoveToURL:)]
        unsafe fn presentedSubitemAtURL_didMoveToURL(&self, old_url: &NSURL, new_url: &NSURL);

        #[cfg(feature = "NSURL")]
        #[optional]
        #[method(presentedSubitemDidChangeAtURL:)]
        unsafe fn presentedSubitemDidChangeAtURL(&self, url: &NSURL);

        #[cfg(all(feature = "NSFileVersion", feature = "NSURL"))]
        #[optional]
        #[method(presentedSubitemAtURL:didGainVersion:)]
        unsafe fn presentedSubitemAtURL_didGainVersion(&self, url: &NSURL, version: &NSFileVersion);

        #[cfg(all(feature = "NSFileVersion", feature = "NSURL"))]
        #[optional]
        #[method(presentedSubitemAtURL:didLoseVersion:)]
        unsafe fn presentedSubitemAtURL_didLoseVersion(&self, url: &NSURL, version: &NSFileVersion);

        #[cfg(all(feature = "NSFileVersion", feature = "NSURL"))]
        #[optional]
        #[method(presentedSubitemAtURL:didResolveConflictVersion:)]
        unsafe fn presentedSubitemAtURL_didResolveConflictVersion(
            &self,
            url: &NSURL,
            version: &NSFileVersion,
        );
    }

    unsafe impl ProtocolType for dyn NSFilePresenter {}
);
