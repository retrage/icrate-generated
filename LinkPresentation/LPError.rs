//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::LinkPresentation::*;

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static LPErrorDomain: Option<&'static NSErrorDomain>;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LPErrorCode(pub NSInteger);
impl LPErrorCode {
    pub const LPErrorUnknown: Self = Self(1);
    pub const LPErrorMetadataFetchFailed: Self = Self(2);
    pub const LPErrorMetadataFetchCancelled: Self = Self(3);
    pub const LPErrorMetadataFetchTimedOut: Self = Self(4);
    pub const LPErrorMetadataFetchNotAllowed: Self = Self(5);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for LPErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for LPErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
