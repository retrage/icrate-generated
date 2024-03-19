//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AdServices::*;
use crate::Foundation::*;

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static AAAttributionErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AAAttributionErrorCode(pub NSInteger);
impl AAAttributionErrorCode {
    #[doc(alias = "AAAttributionErrorCodeNetworkError")]
    pub const NetworkError: Self = Self(1);
    #[doc(alias = "AAAttributionErrorCodeInternalError")]
    pub const InternalError: Self = Self(2);
    #[doc(alias = "AAAttributionErrorCodePlatformNotSupported")]
    pub const PlatformNotSupported: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for AAAttributionErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AAAttributionErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AAAttribution;

    unsafe impl ClassType for AAAttribution {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AAAttribution {}

extern_methods!(
    unsafe impl AAAttribution {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other attributionTokenWithError:_)]
        pub unsafe fn attributionTokenWithError() -> Result<Id<NSString>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AAAttribution {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
