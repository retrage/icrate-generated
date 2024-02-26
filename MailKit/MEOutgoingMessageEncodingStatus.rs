//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEOutgoingMessageEncodingStatus;

    unsafe impl ClassType for MEOutgoingMessageEncodingStatus {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MEOutgoingMessageEncodingStatus {}

unsafe impl NSObjectProtocol for MEOutgoingMessageEncodingStatus {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MEOutgoingMessageEncodingStatus {}

extern_methods!(
    unsafe impl MEOutgoingMessageEncodingStatus {
        #[method(canSign)]
        pub unsafe fn canSign(&self) -> bool;

        #[method(canEncrypt)]
        pub unsafe fn canEncrypt(&self) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other securityError)]
        pub unsafe fn securityError(&self) -> Option<Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEEmailAddress"))]
        #[method_id(@__retain_semantics Other addressesFailingEncryption)]
        pub unsafe fn addressesFailingEncryption(&self) -> Id<NSArray<MEEmailAddress>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "MailKit_MEEmailAddress"
        ))]
        #[method_id(@__retain_semantics Init initWithCanSign:canEncrypt:securityError:addressesFailingEncryption:)]
        pub unsafe fn initWithCanSign_canEncrypt_securityError_addressesFailingEncryption(
            this: Allocated<Self>,
            can_sign: bool,
            can_encrypt: bool,
            security_error: Option<&NSError>,
            addresses_failing_encryption: &NSArray<MEEmailAddress>,
        ) -> Id<Self>;
    }
);
