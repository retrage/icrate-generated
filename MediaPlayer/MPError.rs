//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MPErrorDomain: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPErrorCode(pub NSInteger);
impl MPErrorCode {
    pub const MPErrorUnknown: Self = Self(0);
    pub const MPErrorPermissionDenied: Self = Self(1);
    pub const MPErrorCloudServiceCapabilityMissing: Self = Self(2);
    pub const MPErrorNetworkConnectionFailed: Self = Self(3);
    pub const MPErrorNotFound: Self = Self(4);
    pub const MPErrorNotSupported: Self = Self(5);
    pub const MPErrorCancelled: Self = Self(6);
    pub const MPErrorRequestTimedOut: Self = Self(7);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MPErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MPErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
