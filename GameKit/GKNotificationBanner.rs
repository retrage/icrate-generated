//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use UNNotificationRequest or provide custom UI instead. This method will become a no-op in a future version of GameKit."]
    pub struct GKNotificationBanner;

    unsafe impl ClassType for GKNotificationBanner {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKNotificationBanner {}

extern_methods!(
    unsafe impl GKNotificationBanner {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use UNNotificationRequest or provide custom UI instead. This method will become a no-op in a future version of GameKit."]
        #[method(showBannerWithTitle:message:completionHandler:)]
        pub unsafe fn showBannerWithTitle_message_completionHandler(
            title: Option<&NSString>,
            message: Option<&NSString>,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[deprecated = "Use UNNotificationRequest or provide custom UI instead. This method will become a no-op in a future version of GameKit."]
        #[method(showBannerWithTitle:message:duration:completionHandler:)]
        pub unsafe fn showBannerWithTitle_message_duration_completionHandler(
            title: Option<&NSString>,
            message: Option<&NSString>,
            duration: NSTimeInterval,
            completion_handler: Option<&Block<dyn Fn()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKNotificationBanner {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
