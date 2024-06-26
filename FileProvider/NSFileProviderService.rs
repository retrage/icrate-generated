//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSFileProviderServiceSource {
        #[method_id(@__retain_semantics Other serviceName)]
        unsafe fn serviceName(&self) -> Id<NSFileProviderServiceName>;

        #[method_id(@__retain_semantics Other makeListenerEndpointAndReturnError:_)]
        unsafe fn makeListenerEndpointAndReturnError(
            &self,
        ) -> Result<Id<NSXPCListenerEndpoint>, Id<NSError>>;

        #[optional]
        #[method(isRestricted)]
        unsafe fn isRestricted(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn NSFileProviderServiceSource {}
);

extern_methods!(
    /// NSFileProviderService
    #[cfg(feature = "NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other supportedServiceSourcesForItemIdentifier:error:_)]
        pub unsafe fn supportedServiceSourcesForItemIdentifier_error(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
        ) -> Result<Id<NSArray<ProtocolObject<dyn NSFileProviderServiceSource>>>, Id<NSError>>;
    }
);

extern_methods!(
    /// NSFileProviderService
    #[cfg(feature = "NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(getServiceWithName:itemIdentifier:completionHandler:)]
        pub unsafe fn getServiceWithName_itemIdentifier_completionHandler(
            &self,
            service_name: &NSFileProviderServiceName,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<dyn Fn(*mut NSFileProviderService, *mut NSError)>,
        );
    }
);
