//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[cfg_attr(feature = "apple", link(name = "AdServices", kind = "framework"))]
extern "C" {}
#[path = "AAAttribution.rs"]
mod __AAAttribution;

#[cfg(feature = "AdServices_AAAttribution")]
pub use self::__AAAttribution::AAAttribution;
pub use self::__AAAttribution::AAAttributionErrorDomain;
pub use self::__AAAttribution::{
    AAAttributionErrorCode, AAAttributionErrorCodeInternalError,
    AAAttributionErrorCodeNetworkError, AAAttributionErrorCodePlatformNotSupported,
};
