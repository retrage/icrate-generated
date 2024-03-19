//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKCookiePolicy(pub NSInteger);
impl WKCookiePolicy {
    #[doc(alias = "WKCookiePolicyAllow")]
    pub const Allow: Self = Self(0);
    #[doc(alias = "WKCookiePolicyDisallow")]
    pub const Disallow: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for WKCookiePolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for WKCookiePolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait WKHTTPCookieStoreObserver: NSObjectProtocol {
        #[optional]
        #[method(cookiesDidChangeInCookieStore:)]
        unsafe fn cookiesDidChangeInCookieStore(&self, cookie_store: &WKHTTPCookieStore);
    }

    unsafe impl ProtocolType for dyn WKHTTPCookieStoreObserver {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKHTTPCookieStore;

    unsafe impl ClassType for WKHTTPCookieStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKHTTPCookieStore {}

extern_methods!(
    unsafe impl WKHTTPCookieStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSHTTPCookie"))]
        #[method(getAllCookies:)]
        pub unsafe fn getAllCookies(
            &self,
            completion_handler: &Block<dyn Fn(NonNull<NSArray<NSHTTPCookie>>)>,
        );

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(setCookie:completionHandler:)]
        pub unsafe fn setCookie_completionHandler(
            &self,
            cookie: &NSHTTPCookie,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(deleteCookie:completionHandler:)]
        pub unsafe fn deleteCookie_completionHandler(
            &self,
            cookie: &NSHTTPCookie,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[method(addObserver:)]
        pub unsafe fn addObserver(&self, observer: &ProtocolObject<dyn WKHTTPCookieStoreObserver>);

        #[method(removeObserver:)]
        pub unsafe fn removeObserver(
            &self,
            observer: &ProtocolObject<dyn WKHTTPCookieStoreObserver>,
        );

        #[method(setCookiePolicy:completionHandler:)]
        pub unsafe fn setCookiePolicy_completionHandler(
            &self,
            policy: WKCookiePolicy,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[method(getCookiePolicy:)]
        pub unsafe fn getCookiePolicy(&self, completion_handler: &Block<dyn Fn(WKCookiePolicy)>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKHTTPCookieStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
