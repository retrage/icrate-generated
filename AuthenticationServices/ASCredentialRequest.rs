//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASCredentialRequestType(pub NSInteger);
impl ASCredentialRequestType {
    #[doc(alias = "ASCredentialRequestTypePassword")]
    pub const Password: Self = Self(0);
    #[doc(alias = "ASCredentialRequestTypePasskeyAssertion")]
    pub const PasskeyAssertion: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ASCredentialRequestType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ASCredentialRequestType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    #[cfg(feature = "Foundation_NSObject")]
    pub unsafe trait ASCredentialRequest:
        NSCopying + NSObjectProtocol + NSSecureCoding
    {
        #[method(type)]
        unsafe fn r#type(&self) -> ASCredentialRequestType;

        #[cfg(feature = "AuthenticationServices_ASCredentialIdentity")]
        #[method_id(@__retain_semantics Other credentialIdentity)]
        unsafe fn credentialIdentity(&self) -> Id<ProtocolObject<dyn ASCredentialIdentity>>;
    }

    #[cfg(feature = "Foundation_NSObject")]
    unsafe impl ProtocolType for dyn ASCredentialRequest {}
);
