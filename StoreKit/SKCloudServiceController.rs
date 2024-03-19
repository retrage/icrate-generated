//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKCloudServiceAuthorizationStatus(pub NSInteger);
impl SKCloudServiceAuthorizationStatus {
    #[doc(alias = "SKCloudServiceAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "SKCloudServiceAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "SKCloudServiceAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(2);
    #[doc(alias = "SKCloudServiceAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SKCloudServiceAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SKCloudServiceAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKCloudServiceCapability(pub NSUInteger);
impl SKCloudServiceCapability {
    #[doc(alias = "SKCloudServiceCapabilityNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "SKCloudServiceCapabilityMusicCatalogPlayback")]
    pub const MusicCatalogPlayback: Self = Self(1 << 0);
    #[doc(alias = "SKCloudServiceCapabilityMusicCatalogSubscriptionEligible")]
    pub const MusicCatalogSubscriptionEligible: Self = Self(1 << 1);
    #[doc(alias = "SKCloudServiceCapabilityAddToCloudMusicLibrary")]
    pub const AddToCloudMusicLibrary: Self = Self(1 << 8);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SKCloudServiceCapability {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SKCloudServiceCapability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKCloudServiceController;

    unsafe impl ClassType for SKCloudServiceController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKCloudServiceController {}

extern_methods!(
    unsafe impl SKCloudServiceController {
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> SKCloudServiceAuthorizationStatus;

        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            completion_handler: &Block<dyn Fn(SKCloudServiceAuthorizationStatus)>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestCapabilitiesWithCompletionHandler:)]
        pub unsafe fn requestCapabilitiesWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(SKCloudServiceCapability, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestStorefrontCountryCodeWithCompletionHandler:)]
        pub unsafe fn requestStorefrontCountryCodeWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSString, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestStorefrontIdentifierWithCompletionHandler:)]
        pub unsafe fn requestStorefrontIdentifierWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSString, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestUserTokenForDeveloperToken:completionHandler:)]
        pub unsafe fn requestUserTokenForDeveloperToken_completionHandler(
            &self,
            developer_token: &NSString,
            completion_handler: &Block<dyn Fn(*mut NSString, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(requestPersonalizationTokenForClientToken:withCompletionHandler:)]
        pub unsafe fn requestPersonalizationTokenForClientToken_withCompletionHandler(
            &self,
            client_token: &NSString,
            completion_handler: &Block<dyn Fn(*mut NSString, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKCloudServiceController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static SKCloudServiceCapabilitiesDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static SKStorefrontCountryCodeDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static SKStorefrontIdentifierDidChangeNotification: &'static NSNotificationName;
}
