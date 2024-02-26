//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_protocol!(
    pub unsafe trait MEExtension: NSObjectProtocol {
        #[cfg(feature = "MailKit_MEComposeSession")]
        #[optional]
        #[method_id(@__retain_semantics Other handlerForComposeSession:)]
        unsafe fn handlerForComposeSession(
            &self,
            session: &MEComposeSession,
        ) -> Id<ProtocolObject<dyn MEComposeSessionHandler>>;

        #[cfg(feature = "MailKit_MEMessageActionHandler")]
        #[optional]
        #[method_id(@__retain_semantics Other handlerForMessageActions)]
        unsafe fn handlerForMessageActions(&self)
            -> Id<ProtocolObject<dyn MEMessageActionHandler>>;

        #[cfg(feature = "MailKit_MEContentBlocker")]
        #[optional]
        #[method_id(@__retain_semantics Other handlerForContentBlocker)]
        unsafe fn handlerForContentBlocker(&self) -> Id<ProtocolObject<dyn MEContentBlocker>>;

        #[cfg(all(
            feature = "MailKit_MEMessageDecoder",
            feature = "MailKit_MEMessageEncoder",
            feature = "MailKit_MEMessageSecurityHandler"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other handlerForMessageSecurity)]
        unsafe fn handlerForMessageSecurity(
            &self,
        ) -> Id<ProtocolObject<dyn MEMessageSecurityHandler>>;
    }

    unsafe impl ProtocolType for dyn MEExtension {}
);
