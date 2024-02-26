//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSExtensionContext")]
    pub struct ASCredentialProviderExtensionContext;

    #[cfg(feature = "Foundation_NSExtensionContext")]
    unsafe impl ClassType for ASCredentialProviderExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSExtensionContext")]
unsafe impl NSObjectProtocol for ASCredentialProviderExtensionContext {}

extern_methods!(
    #[cfg(feature = "Foundation_NSExtensionContext")]
    unsafe impl ASCredentialProviderExtensionContext {
        #[cfg(feature = "AuthenticationServices_ASPasswordCredential")]
        #[method(completeRequestWithSelectedCredential:completionHandler:)]
        pub unsafe fn completeRequestWithSelectedCredential_completionHandler(
            &self,
            credential: &ASPasswordCredential,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
        #[method(completeAssertionRequestWithSelectedPasskeyCredential:completionHandler:)]
        pub unsafe fn completeAssertionRequestWithSelectedPasskeyCredential_completionHandler(
            &self,
            credential: &ASPasskeyAssertionCredential,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "AuthenticationServices_ASPasskeyRegistrationCredential")]
        #[method(completeRegistrationRequestWithSelectedPasskeyCredential:completionHandler:)]
        pub unsafe fn completeRegistrationRequestWithSelectedPasskeyCredential_completionHandler(
            &self,
            credential: &ASPasskeyRegistrationCredential,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[method(completeExtensionConfigurationRequest)]
        pub unsafe fn completeExtensionConfigurationRequest(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completion_handler: Option<&Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSExtensionContext")]
    unsafe impl ASCredentialProviderExtensionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
