//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASCredentialServiceIdentifierType(pub NSInteger);
impl ASCredentialServiceIdentifierType {
    #[doc(alias = "ASCredentialServiceIdentifierTypeDomain")]
    pub const Domain: Self = Self(0);
    #[doc(alias = "ASCredentialServiceIdentifierTypeURL")]
    pub const URL: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ASCredentialServiceIdentifierType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ASCredentialServiceIdentifierType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASCredentialServiceIdentifier;

    unsafe impl ClassType for ASCredentialServiceIdentifier {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for ASCredentialServiceIdentifier {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for ASCredentialServiceIdentifier {}

unsafe impl NSObjectProtocol for ASCredentialServiceIdentifier {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for ASCredentialServiceIdentifier {}

extern_methods!(
    unsafe impl ASCredentialServiceIdentifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:type:)]
        pub unsafe fn initWithIdentifier_type(
            this: Allocated<Self>,
            identifier: &NSString,
            r#type: ASCredentialServiceIdentifierType,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> ASCredentialServiceIdentifierType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASCredentialServiceIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
