//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFileProviderSyncAnchor = NSData;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFileProviderPage = NSData;

extern "C" {
    pub static NSFileProviderInitialPageSortedByDate: &'static NSFileProviderPage;
}

extern "C" {
    pub static NSFileProviderInitialPageSortedByName: &'static NSFileProviderPage;
}

extern_protocol!(
    pub unsafe trait NSFileProviderEnumerationObserver: NSObjectProtocol {
        #[cfg(feature = "NSFileProviderItem")]
        #[method(didEnumerateItems:)]
        unsafe fn didEnumerateItems(
            &self,
            updated_items: &NSArray<ProtocolObject<dyn NSFileProviderItemProtocol>>,
        );

        #[method(finishEnumeratingUpToPage:)]
        unsafe fn finishEnumeratingUpToPage(&self, next_page: Option<&NSFileProviderPage>);

        #[method(finishEnumeratingWithError:)]
        unsafe fn finishEnumeratingWithError(&self, error: &NSError);

        #[optional]
        #[method(suggestedPageSize)]
        unsafe fn suggestedPageSize(&self) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn NSFileProviderEnumerationObserver {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderChangeObserver: NSObjectProtocol {
        #[cfg(feature = "NSFileProviderItem")]
        #[method(didUpdateItems:)]
        unsafe fn didUpdateItems(
            &self,
            updated_items: &NSArray<ProtocolObject<dyn NSFileProviderItemProtocol>>,
        );

        #[cfg(feature = "NSFileProviderItem")]
        #[method(didDeleteItemsWithIdentifiers:)]
        unsafe fn didDeleteItemsWithIdentifiers(
            &self,
            deleted_item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
        );

        #[method(finishEnumeratingChangesUpToSyncAnchor:moreComing:)]
        unsafe fn finishEnumeratingChangesUpToSyncAnchor_moreComing(
            &self,
            anchor: &NSFileProviderSyncAnchor,
            more_coming: bool,
        );

        #[method(finishEnumeratingWithError:)]
        unsafe fn finishEnumeratingWithError(&self, error: &NSError);

        #[optional]
        #[method(suggestedBatchSize)]
        unsafe fn suggestedBatchSize(&self) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn NSFileProviderChangeObserver {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderEnumerator: NSObjectProtocol {
        #[method(invalidate)]
        unsafe fn invalidate(&self);

        #[method(enumerateItemsForObserver:startingAtPage:)]
        unsafe fn enumerateItemsForObserver_startingAtPage(
            &self,
            observer: &ProtocolObject<dyn NSFileProviderEnumerationObserver>,
            page: &NSFileProviderPage,
        );

        #[optional]
        #[method(enumerateChangesForObserver:fromSyncAnchor:)]
        unsafe fn enumerateChangesForObserver_fromSyncAnchor(
            &self,
            observer: &ProtocolObject<dyn NSFileProviderChangeObserver>,
            sync_anchor: &NSFileProviderSyncAnchor,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(currentSyncAnchorWithCompletionHandler:)]
        unsafe fn currentSyncAnchorWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSFileProviderSyncAnchor)>,
        );
    }

    unsafe impl ProtocolType for dyn NSFileProviderEnumerator {}
);

extern_methods!(
    /// NSFileProviderEnumeration
    #[cfg(feature = "NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other enumeratorForContainerItemIdentifier:error:_)]
        pub unsafe fn enumeratorForContainerItemIdentifier_error(
            &self,
            container_item_identifier: &NSFileProviderItemIdentifier,
        ) -> Result<Id<ProtocolObject<dyn NSFileProviderEnumerator>>, Id<NSError>>;
    }
);
