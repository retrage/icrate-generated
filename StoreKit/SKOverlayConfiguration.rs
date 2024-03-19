//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKOverlayPosition(pub NSInteger);
impl SKOverlayPosition {
    #[doc(alias = "SKOverlayPositionBottom")]
    pub const Bottom: Self = Self(0);
    #[doc(alias = "SKOverlayPositionBottomRaised")]
    pub const BottomRaised: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SKOverlayPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SKOverlayPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKOverlayConfiguration;

    unsafe impl ClassType for SKOverlayConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKOverlayConfiguration {}

extern_methods!(
    unsafe impl SKOverlayConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKOverlayAppConfiguration;

    unsafe impl ClassType for SKOverlayAppConfiguration {
        #[inherits(NSObject)]
        type Super = SKOverlayConfiguration;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKOverlayAppConfiguration {}

extern_methods!(
    unsafe impl SKOverlayAppConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAppIdentifier:position:)]
        pub unsafe fn initWithAppIdentifier_position(
            this: Allocated<Self>,
            app_identifier: &NSString,
            position: SKOverlayPosition,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other appIdentifier)]
        pub unsafe fn appIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAppIdentifier:)]
        pub unsafe fn setAppIdentifier(&self, app_identifier: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other campaignToken)]
        pub unsafe fn campaignToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCampaignToken:)]
        pub unsafe fn setCampaignToken(&self, campaign_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other providerToken)]
        pub unsafe fn providerToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setProviderToken:)]
        pub unsafe fn setProviderToken(&self, provider_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customProductPageIdentifier)]
        pub unsafe fn customProductPageIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomProductPageIdentifier:)]
        pub unsafe fn setCustomProductPageIdentifier(
            &self,
            custom_product_page_identifier: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other latestReleaseID)]
        pub unsafe fn latestReleaseID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLatestReleaseID:)]
        pub unsafe fn setLatestReleaseID(&self, latest_release_id: Option<&NSString>);

        #[method(position)]
        pub unsafe fn position(&self) -> SKOverlayPosition;

        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: SKOverlayPosition);

        #[method(userDismissible)]
        pub unsafe fn userDismissible(&self) -> bool;

        #[method(setUserDismissible:)]
        pub unsafe fn setUserDismissible(&self, user_dismissible: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdditionalValue:forKey:)]
        pub unsafe fn setAdditionalValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other additionalValueForKey:)]
        pub unsafe fn additionalValueForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "StoreKit_SKAdImpression")]
        #[method(setAdImpression:)]
        pub unsafe fn setAdImpression(&self, impression: &SKAdImpression);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKOverlayAppClipConfiguration;

    unsafe impl ClassType for SKOverlayAppClipConfiguration {
        #[inherits(NSObject)]
        type Super = SKOverlayConfiguration;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKOverlayAppClipConfiguration {}

extern_methods!(
    unsafe impl SKOverlayAppClipConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithPosition:)]
        pub unsafe fn initWithPosition(
            this: Allocated<Self>,
            position: SKOverlayPosition,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other campaignToken)]
        pub unsafe fn campaignToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCampaignToken:)]
        pub unsafe fn setCampaignToken(&self, campaign_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other providerToken)]
        pub unsafe fn providerToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setProviderToken:)]
        pub unsafe fn setProviderToken(&self, provider_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customProductPageIdentifier)]
        pub unsafe fn customProductPageIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomProductPageIdentifier:)]
        pub unsafe fn setCustomProductPageIdentifier(
            &self,
            custom_product_page_identifier: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other latestReleaseID)]
        pub unsafe fn latestReleaseID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLatestReleaseID:)]
        pub unsafe fn setLatestReleaseID(&self, latest_release_id: Option<&NSString>);

        #[method(position)]
        pub unsafe fn position(&self) -> SKOverlayPosition;

        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: SKOverlayPosition);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdditionalValue:forKey:)]
        pub unsafe fn setAdditionalValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other additionalValueForKey:)]
        pub unsafe fn additionalValueForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;
    }
);
