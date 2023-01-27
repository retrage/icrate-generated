//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum UNAuthorizationOptions {
        UNAuthorizationOptionBadge = 1 << 0,
        UNAuthorizationOptionSound = 1 << 1,
        UNAuthorizationOptionAlert = 1 << 2,
        UNAuthorizationOptionCarPlay = 1 << 3,
        UNAuthorizationOptionCriticalAlert = 1 << 4,
        UNAuthorizationOptionProvidesAppNotificationSettings = 1 << 5,
        UNAuthorizationOptionProvisional = 1 << 6,
        #[deprecated = "Announcement authorization is always included"]
        UNAuthorizationOptionAnnouncement = 1 << 7,
        #[deprecated = "Use time-sensitive entitlement"]
        UNAuthorizationOptionTimeSensitive = 1 << 8,
    }
);

extern_static!(UNAuthorizationOptionNone: UNAuthorizationOptions = 0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNUserNotificationCenter")]
    pub struct UNUserNotificationCenter;

    #[cfg(feature = "UserNotifications_UNUserNotificationCenter")]
    unsafe impl ClassType for UNUserNotificationCenter {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNUserNotificationCenter")]
unsafe impl NSObjectProtocol for UNUserNotificationCenter {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNUserNotificationCenter")]
    unsafe impl UNUserNotificationCenter {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UNUserNotificationCenterDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UNUserNotificationCenterDelegate>>,
        );

        #[method(supportsContentExtensions)]
        pub unsafe fn supportsContentExtensions(&self) -> bool;

        #[method_id(@__retain_semantics Other currentNotificationCenter)]
        pub unsafe fn currentNotificationCenter() -> Id<UNUserNotificationCenter, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestAuthorizationWithOptions:completionHandler:)]
        pub unsafe fn requestAuthorizationWithOptions_completionHandler(
            &self,
            options: UNAuthorizationOptions,
            completion_handler: &Block<(Bool, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSSet",
            feature = "UserNotifications_UNNotificationCategory"
        ))]
        #[method(setNotificationCategories:)]
        pub unsafe fn setNotificationCategories(&self, categories: &NSSet<UNNotificationCategory>);

        #[cfg(all(
            feature = "Foundation_NSSet",
            feature = "UserNotifications_UNNotificationCategory"
        ))]
        #[method(getNotificationCategoriesWithCompletionHandler:)]
        pub unsafe fn getNotificationCategoriesWithCompletionHandler(
            &self,
            completion_handler: &Block<(NonNull<NSSet<UNNotificationCategory>>,), ()>,
        );

        #[cfg(feature = "UserNotifications_UNNotificationSettings")]
        #[method(getNotificationSettingsWithCompletionHandler:)]
        pub unsafe fn getNotificationSettingsWithCompletionHandler(
            &self,
            completion_handler: &Block<(NonNull<UNNotificationSettings>,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "UserNotifications_UNNotificationRequest"
        ))]
        #[method(addNotificationRequest:withCompletionHandler:)]
        pub unsafe fn addNotificationRequest_withCompletionHandler(
            &self,
            request: &UNNotificationRequest,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UserNotifications_UNNotificationRequest"
        ))]
        #[method(getPendingNotificationRequestsWithCompletionHandler:)]
        pub unsafe fn getPendingNotificationRequestsWithCompletionHandler(
            &self,
            completion_handler: &Block<(NonNull<NSArray<UNNotificationRequest>>,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(removePendingNotificationRequestsWithIdentifiers:)]
        pub unsafe fn removePendingNotificationRequestsWithIdentifiers(
            &self,
            identifiers: &NSArray<NSString>,
        );

        #[method(removeAllPendingNotificationRequests)]
        pub unsafe fn removeAllPendingNotificationRequests(&self);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UserNotifications_UNNotification"
        ))]
        #[method(getDeliveredNotificationsWithCompletionHandler:)]
        pub unsafe fn getDeliveredNotificationsWithCompletionHandler(
            &self,
            completion_handler: &Block<(NonNull<NSArray<UNNotification>>,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(removeDeliveredNotificationsWithIdentifiers:)]
        pub unsafe fn removeDeliveredNotificationsWithIdentifiers(
            &self,
            identifiers: &NSArray<NSString>,
        );

        #[method(removeAllDeliveredNotifications)]
        pub unsafe fn removeAllDeliveredNotifications(&self);

        #[cfg(feature = "Foundation_NSError")]
        #[method(setBadgeCount:withCompletionHandler:)]
        pub unsafe fn setBadgeCount_withCompletionHandler(
            &self,
            new_badge_count: NSInteger,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum UNNotificationPresentationOptions {
        UNNotificationPresentationOptionBadge = 1 << 0,
        UNNotificationPresentationOptionSound = 1 << 1,
        #[deprecated]
        UNNotificationPresentationOptionAlert = 1 << 2,
        UNNotificationPresentationOptionList = 1 << 3,
        UNNotificationPresentationOptionBanner = 1 << 4,
    }
);

extern_static!(UNNotificationPresentationOptionNone: UNNotificationPresentationOptions = 0);

extern_protocol!(
    pub unsafe trait UNUserNotificationCenterDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "UserNotifications_UNNotification",
            feature = "UserNotifications_UNUserNotificationCenter"
        ))]
        #[optional]
        #[method(userNotificationCenter:willPresentNotification:withCompletionHandler:)]
        unsafe fn userNotificationCenter_willPresentNotification_withCompletionHandler(
            &self,
            center: &UNUserNotificationCenter,
            notification: &UNNotification,
            completion_handler: &Block<(UNNotificationPresentationOptions,), ()>,
        );

        #[cfg(all(
            feature = "UserNotifications_UNNotificationResponse",
            feature = "UserNotifications_UNUserNotificationCenter"
        ))]
        #[optional]
        #[method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:)]
        unsafe fn userNotificationCenter_didReceiveNotificationResponse_withCompletionHandler(
            &self,
            center: &UNUserNotificationCenter,
            response: &UNNotificationResponse,
            completion_handler: &Block<(), ()>,
        );

        #[cfg(all(
            feature = "UserNotifications_UNNotification",
            feature = "UserNotifications_UNUserNotificationCenter"
        ))]
        #[optional]
        #[method(userNotificationCenter:openSettingsForNotification:)]
        unsafe fn userNotificationCenter_openSettingsForNotification(
            &self,
            center: &UNUserNotificationCenter,
            notification: Option<&UNNotification>,
        );
    }

    unsafe impl ProtocolType for dyn UNUserNotificationCenterDelegate {}
);
