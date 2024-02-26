//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for ASCredentialProviderViewController {}

#[cfg(all(
    feature = "AppKit_NSKeyValueBinding",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSEditor for ASCredentialProviderViewController {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
unsafe impl NSObjectProtocol for ASCredentialProviderViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSStoryboardSegue",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSSeguePerforming for ASCredentialProviderViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for ASCredentialProviderViewController {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ASCredentialProviderViewController {
        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialProviderExtensionContext",
            feature = "Foundation_NSExtensionContext"
        ))]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Id<ASCredentialProviderExtensionContext>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSArray"
        ))]
        #[method(prepareCredentialListForServiceIdentifiers:)]
        pub unsafe fn prepareCredentialListForServiceIdentifiers(
            &self,
            service_identifiers: &NSArray<ASCredentialServiceIdentifier>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "AuthenticationServices_ASPasskeyCredentialRequestParameters",
            feature = "Foundation_NSArray"
        ))]
        #[method(prepareCredentialListForServiceIdentifiers:requestParameters:)]
        pub unsafe fn prepareCredentialListForServiceIdentifiers_requestParameters(
            &self,
            service_identifiers: &NSArray<ASCredentialServiceIdentifier>,
            request_parameters: &ASPasskeyCredentialRequestParameters,
        );

        #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
        #[deprecated]
        #[method(provideCredentialWithoutUserInteractionForIdentity:)]
        pub unsafe fn provideCredentialWithoutUserInteractionForIdentity(
            &self,
            credential_identity: &ASPasswordCredentialIdentity,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialRequest",
            feature = "Foundation_NSObject"
        ))]
        #[method(provideCredentialWithoutUserInteractionForRequest:)]
        pub unsafe fn provideCredentialWithoutUserInteractionForRequest(
            &self,
            credential_request: &ProtocolObject<dyn ASCredentialRequest>,
        );

        #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
        #[deprecated]
        #[method(prepareInterfaceToProvideCredentialForIdentity:)]
        pub unsafe fn prepareInterfaceToProvideCredentialForIdentity(
            &self,
            credential_identity: &ASPasswordCredentialIdentity,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialRequest",
            feature = "Foundation_NSObject"
        ))]
        #[method(prepareInterfaceToProvideCredentialForRequest:)]
        pub unsafe fn prepareInterfaceToProvideCredentialForRequest(
            &self,
            credential_request: &ProtocolObject<dyn ASCredentialRequest>,
        );

        #[method(prepareInterfaceForExtensionConfiguration)]
        pub unsafe fn prepareInterfaceForExtensionConfiguration(&self);

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialRequest",
            feature = "Foundation_NSObject"
        ))]
        #[method(prepareInterfaceForPasskeyRegistration:)]
        pub unsafe fn prepareInterfaceForPasskeyRegistration(
            &self,
            registration_request: &ProtocolObject<dyn ASCredentialRequest>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ASCredentialProviderViewController {
        #[cfg(all(
            feature = "AppKit_NSNib",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ASCredentialProviderViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ASCredentialProviderViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
