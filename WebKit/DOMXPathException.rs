//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static DOMXPathException: Option<&'static NSString>;
}

#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DOMXPathExceptionCode(pub c_uint);
impl DOMXPathExceptionCode {
    #[deprecated]
    pub const DOM_INVALID_EXPRESSION_ERR: Self = Self(51);
    #[deprecated]
    pub const DOM_TYPE_ERR: Self = Self(52);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for DOMXPathExceptionCode {
    const ENCODING: Encoding = c_uint::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for DOMXPathExceptionCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
