//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static ILMessageFilterErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ILMessageFilterError(pub NSInteger);
impl ILMessageFilterError {
    #[doc(alias = "ILMessageFilterErrorSystem")]
    pub const System: Self = Self(1);
    #[doc(alias = "ILMessageFilterErrorInvalidNetworkURL")]
    pub const InvalidNetworkURL: Self = Self(2);
    #[doc(alias = "ILMessageFilterErrorNetworkURLUnauthorized")]
    pub const NetworkURLUnauthorized: Self = Self(3);
    #[doc(alias = "ILMessageFilterErrorNetworkRequestFailed")]
    pub const NetworkRequestFailed: Self = Self(4);
    #[doc(alias = "ILMessageFilterErrorRedundantNetworkDeferral")]
    pub const RedundantNetworkDeferral: Self = Self(5);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ILMessageFilterError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ILMessageFilterError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
