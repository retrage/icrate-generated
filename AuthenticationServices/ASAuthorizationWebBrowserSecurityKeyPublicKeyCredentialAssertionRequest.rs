//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialAssertionRequest {
        #[cfg(feature = "AuthenticationServices_ASPublicKeyCredentialClientData")]
        #[method_id(@__retain_semantics Other clientData)]
        unsafe fn clientData(&self) -> Option<Id<ASPublicKeyCredentialClientData>>;
    }

    unsafe impl ProtocolType
        for dyn ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialAssertionRequest
    {
    }
);
