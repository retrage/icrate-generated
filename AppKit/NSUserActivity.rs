//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSUserActivityRestoring: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "Foundation_NSUserActivity")]
        #[method(restoreUserActivityState:)]
        unsafe fn restoreUserActivityState(&self, user_activity: &NSUserActivity);
    }

    unsafe impl ProtocolType for dyn NSUserActivityRestoring {}
);

extern_methods!(
    /// NSUserActivity
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[cfg(feature = "Foundation_NSUserActivity")]
        #[method_id(@__retain_semantics Other userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Id<NSUserActivity>>;

        #[cfg(feature = "Foundation_NSUserActivity")]
        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, user_activity: Option<&NSUserActivity>);

        #[cfg(feature = "Foundation_NSUserActivity")]
        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, user_activity: &NSUserActivity);
    }
);

#[cfg(feature = "AppKit_NSResponder")]
unsafe impl NSUserActivityRestoring for NSResponder {}

extern_methods!(
    /// NSUserActivity
    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl NSDocument {
        #[cfg(feature = "Foundation_NSUserActivity")]
        #[method_id(@__retain_semantics Other userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Id<NSUserActivity>>;

        #[cfg(feature = "Foundation_NSUserActivity")]
        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, user_activity: Option<&NSUserActivity>);

        #[cfg(feature = "Foundation_NSUserActivity")]
        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, activity: &NSUserActivity);
    }
);

#[cfg(feature = "AppKit_NSDocument")]
unsafe impl NSUserActivityRestoring for NSDocument {}

extern_static!(NSUserActivityDocumentURLKey: &'static NSString);
