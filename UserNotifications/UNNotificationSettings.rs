//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum UNAuthorizationStatus {
        UNAuthorizationStatusNotDetermined = 0,
        UNAuthorizationStatusDenied = 1,
        UNAuthorizationStatusAuthorized = 2,
        UNAuthorizationStatusProvisional = 3,
        UNAuthorizationStatusEphemeral = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum UNShowPreviewsSetting {
        UNShowPreviewsSettingAlways = 0,
        UNShowPreviewsSettingWhenAuthenticated = 1,
        UNShowPreviewsSettingNever = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum UNNotificationSetting {
        UNNotificationSettingNotSupported = 0,
        UNNotificationSettingDisabled = 1,
        UNNotificationSettingEnabled = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum UNAlertStyle {
        UNAlertStyleNone = 0,
        UNAlertStyleBanner = 1,
        UNAlertStyleAlert = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationSettings")]
    pub struct UNNotificationSettings;

    #[cfg(feature = "UserNotifications_UNNotificationSettings")]
    unsafe impl ClassType for UNNotificationSettings {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationSettings")]
unsafe impl NSCoding for UNNotificationSettings {}

#[cfg(feature = "UserNotifications_UNNotificationSettings")]
unsafe impl NSObjectProtocol for UNNotificationSettings {}

#[cfg(feature = "UserNotifications_UNNotificationSettings")]
unsafe impl NSSecureCoding for UNNotificationSettings {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationSettings")]
    unsafe impl UNNotificationSettings {
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus(&self) -> UNAuthorizationStatus;

        #[method(soundSetting)]
        pub unsafe fn soundSetting(&self) -> UNNotificationSetting;

        #[method(badgeSetting)]
        pub unsafe fn badgeSetting(&self) -> UNNotificationSetting;

        #[method(alertSetting)]
        pub unsafe fn alertSetting(&self) -> UNNotificationSetting;

        #[method(notificationCenterSetting)]
        pub unsafe fn notificationCenterSetting(&self) -> UNNotificationSetting;

        #[method(lockScreenSetting)]
        pub unsafe fn lockScreenSetting(&self) -> UNNotificationSetting;

        #[method(carPlaySetting)]
        pub unsafe fn carPlaySetting(&self) -> UNNotificationSetting;

        #[method(alertStyle)]
        pub unsafe fn alertStyle(&self) -> UNAlertStyle;

        #[method(showPreviewsSetting)]
        pub unsafe fn showPreviewsSetting(&self) -> UNShowPreviewsSetting;

        #[method(criticalAlertSetting)]
        pub unsafe fn criticalAlertSetting(&self) -> UNNotificationSetting;

        #[method(providesAppNotificationSettings)]
        pub unsafe fn providesAppNotificationSettings(&self) -> bool;

        #[method(announcementSetting)]
        pub unsafe fn announcementSetting(&self) -> UNNotificationSetting;

        #[method(timeSensitiveSetting)]
        pub unsafe fn timeSensitiveSetting(&self) -> UNNotificationSetting;

        #[method(scheduledDeliverySetting)]
        pub unsafe fn scheduledDeliverySetting(&self) -> UNNotificationSetting;

        #[method(directMessagesSetting)]
        pub unsafe fn directMessagesSetting(&self) -> UNNotificationSetting;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
