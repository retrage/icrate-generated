//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionAuthenticationMethod(pub NSInteger);
impl ASAuthorizationProviderExtensionAuthenticationMethod {
    #[doc(alias = "ASAuthorizationProviderExtensionAuthenticationMethodPassword")]
    pub const Password: Self = Self(1);
    #[doc(alias = "ASAuthorizationProviderExtensionAuthenticationMethodUserSecureEnclaveKey")]
    pub const UserSecureEnclaveKey: Self = Self(2);
    #[doc(alias = "ASAuthorizationProviderExtensionAuthenticationMethodSmartCard")]
    pub const SmartCard: Self = Self(3);
}

unsafe impl Encode for ASAuthorizationProviderExtensionAuthenticationMethod {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionAuthenticationMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionRequestOptions(pub NSUInteger);
impl ASAuthorizationProviderExtensionRequestOptions {
    #[doc(alias = "ASAuthorizationProviderExtensionRequestOptionsNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "ASAuthorizationProviderExtensionRequestOptionsUserInteractionEnabled")]
    pub const UserInteractionEnabled: Self = Self(1 << 0);
    #[doc(alias = "ASAuthorizationProviderExtensionRequestOptionsRegistrationRepair")]
    pub const RegistrationRepair: Self = Self(1 << 1);
    #[doc(alias = "ASAuthorizationProviderExtensionRequestOptionsRegistrationSharedDeviceKeys")]
    pub const RegistrationSharedDeviceKeys: Self = Self(1 << 2);
    #[doc(alias = "ASAuthorizationProviderExtensionRequestOptionsRegistrationDeviceKeyMigration")]
    pub const RegistrationDeviceKeyMigration: Self = Self(1 << 3);
    #[doc(alias = "ASAuthorizationProviderExtensionRequestOptionsUserKeyInvalid")]
    pub const UserKeyInvalid: Self = Self(1 << 5);
}

unsafe impl Encode for ASAuthorizationProviderExtensionRequestOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionRequestOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionRegistrationResult(pub NSInteger);
impl ASAuthorizationProviderExtensionRegistrationResult {
    #[doc(alias = "ASAuthorizationProviderExtensionRegistrationResultSuccess")]
    pub const Success: Self = Self(0);
    #[doc(alias = "ASAuthorizationProviderExtensionRegistrationResultFailed")]
    pub const Failed: Self = Self(1);
    #[doc(alias = "ASAuthorizationProviderExtensionRegistrationResultUserInterfaceRequired")]
    pub const UserInterfaceRequired: Self = Self(2);
    #[doc(alias = "ASAuthorizationProviderExtensionRegistrationResultFailedNoRetry")]
    pub const FailedNoRetry: Self = Self(3);
}

unsafe impl Encode for ASAuthorizationProviderExtensionRegistrationResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionRegistrationResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionSupportedGrantTypes(pub NSInteger);
impl ASAuthorizationProviderExtensionSupportedGrantTypes {
    #[doc(alias = "ASAuthorizationProviderExtensionSupportedGrantTypesNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "ASAuthorizationProviderExtensionSupportedGrantTypesPassword")]
    pub const Password: Self = Self(1 << 0);
    #[doc(alias = "ASAuthorizationProviderExtensionSupportedGrantTypesJWTBearer")]
    pub const JWTBearer: Self = Self(1 << 1);
    #[doc(alias = "ASAuthorizationProviderExtensionSupportedGrantTypesSAML1_1")]
    pub const SAML1_1: Self = Self(1 << 2);
    #[doc(alias = "ASAuthorizationProviderExtensionSupportedGrantTypesSAML2_0")]
    pub const SAML2_0: Self = Self(1 << 3);
}

unsafe impl Encode for ASAuthorizationProviderExtensionSupportedGrantTypes {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionSupportedGrantTypes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionPlatformSSOProtocolVersion(pub NSInteger);
impl ASAuthorizationProviderExtensionPlatformSSOProtocolVersion {
    pub const ASAuthorizationProviderExtensionPlatformSSOProtocolVersion1_0: Self = Self(0);
    pub const ASAuthorizationProviderExtensionPlatformSSOProtocolVersion2_0: Self = Self(1);
}

unsafe impl Encode for ASAuthorizationProviderExtensionPlatformSSOProtocolVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionPlatformSSOProtocolVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait ASAuthorizationProviderExtensionRegistrationHandler:
        NSObjectProtocol
    {
        #[cfg(all(
            feature = "ASAuthorizationProviderExtensionLoginManager",
            feature = "block2"
        ))]
        #[method(beginDeviceRegistrationUsingLoginManager:options:completion:)]
        unsafe fn beginDeviceRegistrationUsingLoginManager_options_completion(
            &self,
            login_manager: &ASAuthorizationProviderExtensionLoginManager,
            options: ASAuthorizationProviderExtensionRequestOptions,
            completion: &Block<dyn Fn(ASAuthorizationProviderExtensionRegistrationResult)>,
        );

        #[cfg(all(
            feature = "ASAuthorizationProviderExtensionLoginManager",
            feature = "block2"
        ))]
        #[method(beginUserRegistrationUsingLoginManager:userName:authenticationMethod:options:completion:)]
        unsafe fn beginUserRegistrationUsingLoginManager_userName_authenticationMethod_options_completion(
            &self,
            login_manager: &ASAuthorizationProviderExtensionLoginManager,
            user_name: Option<&NSString>,
            authentication_method: ASAuthorizationProviderExtensionAuthenticationMethod,
            options: ASAuthorizationProviderExtensionRequestOptions,
            completion: &Block<dyn Fn(ASAuthorizationProviderExtensionRegistrationResult)>,
        );

        #[optional]
        #[method(registrationDidComplete)]
        unsafe fn registrationDidComplete(&self);

        #[optional]
        #[method(registrationDidCancel)]
        unsafe fn registrationDidCancel(&self);

        #[optional]
        #[method(supportedGrantTypes)]
        unsafe fn supportedGrantTypes(&self)
            -> ASAuthorizationProviderExtensionSupportedGrantTypes;

        #[optional]
        #[method(protocolVersion)]
        unsafe fn protocolVersion(
            &self,
        ) -> ASAuthorizationProviderExtensionPlatformSSOProtocolVersion;
    }

    unsafe impl ProtocolType for dyn ASAuthorizationProviderExtensionRegistrationHandler {}
);
