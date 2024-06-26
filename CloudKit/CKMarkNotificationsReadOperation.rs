//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
    pub struct CKMarkNotificationsReadOperation;

    #[cfg(feature = "CKOperation")]
    unsafe impl ClassType for CKMarkNotificationsReadOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKMarkNotificationsReadOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKMarkNotificationsReadOperation {
        #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CKNotification")]
        #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
        #[method_id(@__retain_semantics Init initWithNotificationIDsToMarkRead:)]
        pub unsafe fn initWithNotificationIDsToMarkRead(
            this: Allocated<Self>,
            notification_i_ds: &NSArray<CKNotificationID>,
        ) -> Id<Self>;

        #[cfg(feature = "CKNotification")]
        #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
        #[method_id(@__retain_semantics Other notificationIDs)]
        pub unsafe fn notificationIDs(&self) -> Option<Id<NSArray<CKNotificationID>>>;

        #[cfg(feature = "CKNotification")]
        #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
        #[method(setNotificationIDs:)]
        pub unsafe fn setNotificationIDs(
            &self,
            notification_i_ds: Option<&NSArray<CKNotificationID>>,
        );

        #[cfg(all(feature = "CKNotification", feature = "block2"))]
        #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
        #[method(markNotificationsReadCompletionBlock)]
        pub unsafe fn markNotificationsReadCompletionBlock(
            &self,
        ) -> *mut Block<dyn Fn(*mut NSArray<CKNotificationID>, *mut NSError)>;

        #[cfg(all(feature = "CKNotification", feature = "block2"))]
        #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
        #[method(setMarkNotificationsReadCompletionBlock:)]
        pub unsafe fn setMarkNotificationsReadCompletionBlock(
            &self,
            mark_notifications_read_completion_block: Option<
                &Block<dyn Fn(*mut NSArray<CKNotificationID>, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKMarkNotificationsReadOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
