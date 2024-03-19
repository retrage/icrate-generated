//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASUserDetectionStatus(pub NSInteger);
impl ASUserDetectionStatus {
    #[doc(alias = "ASUserDetectionStatusUnsupported")]
    pub const Unsupported: Self = Self(0);
    #[doc(alias = "ASUserDetectionStatusUnknown")]
    pub const Unknown: Self = Self(1);
    #[doc(alias = "ASUserDetectionStatusLikelyReal")]
    pub const LikelyReal: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ASUserDetectionStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ASUserDetectionStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASUserAgeRange(pub NSInteger);
impl ASUserAgeRange {
    #[doc(alias = "ASUserAgeRangeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "ASUserAgeRangeChild")]
    pub const Child: Self = Self(1);
    #[doc(alias = "ASUserAgeRangeNotChild")]
    pub const NotChild: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ASUserAgeRange {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ASUserAgeRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationAppleIDCredential;

    unsafe impl ClassType for ASAuthorizationAppleIDCredential {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationCredential",
    feature = "Foundation_NSObject"
))]
unsafe impl ASAuthorizationCredential for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for ASAuthorizationAppleIDCredential {}

unsafe impl NSObjectProtocol for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for ASAuthorizationAppleIDCredential {}

extern_methods!(
    unsafe impl ASAuthorizationAppleIDCredential {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorization",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other authorizedScopes)]
        pub unsafe fn authorizedScopes(&self) -> Id<NSArray<ASAuthorizationScope>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other authorizationCode)]
        pub unsafe fn authorizationCode(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other identityToken)]
        pub unsafe fn identityToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other email)]
        pub unsafe fn email(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[method_id(@__retain_semantics Other fullName)]
        pub unsafe fn fullName(&self) -> Option<Id<NSPersonNameComponents>>;

        #[method(realUserStatus)]
        pub unsafe fn realUserStatus(&self) -> ASUserDetectionStatus;

        #[method(userAgeRange)]
        pub unsafe fn userAgeRange(&self) -> ASUserAgeRange;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
