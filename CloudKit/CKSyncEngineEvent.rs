//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKSyncEngineEventType {
        #[doc(alias = "CKSyncEngineEventTypeStateUpdate")]
        StateUpdate = 0,
        #[doc(alias = "CKSyncEngineEventTypeAccountChange")]
        AccountChange = 1,
        #[doc(alias = "CKSyncEngineEventTypeFetchedDatabaseChanges")]
        FetchedDatabaseChanges = 2,
        #[doc(alias = "CKSyncEngineEventTypeFetchedRecordZoneChanges")]
        FetchedRecordZoneChanges = 3,
        #[doc(alias = "CKSyncEngineEventTypeSentDatabaseChanges")]
        SentDatabaseChanges = 4,
        #[doc(alias = "CKSyncEngineEventTypeSentRecordZoneChanges")]
        SentRecordZoneChanges = 5,
        #[doc(alias = "CKSyncEngineEventTypeWillFetchChanges")]
        WillFetchChanges = 6,
        #[doc(alias = "CKSyncEngineEventTypeWillFetchRecordZoneChanges")]
        WillFetchRecordZoneChanges = 7,
        #[doc(alias = "CKSyncEngineEventTypeDidFetchRecordZoneChanges")]
        DidFetchRecordZoneChanges = 8,
        #[doc(alias = "CKSyncEngineEventTypeDidFetchChanges")]
        DidFetchChanges = 9,
        #[doc(alias = "CKSyncEngineEventTypeWillSendChanges")]
        WillSendChanges = 10,
        #[doc(alias = "CKSyncEngineEventTypeDidSendChanges")]
        DidSendChanges = 11,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineEvent;

    unsafe impl ClassType for CKSyncEngineEvent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineEvent {}

unsafe impl Sync for CKSyncEngineEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineEvent {}

extern_methods!(
    unsafe impl CKSyncEngineEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> CKSyncEngineEventType;

        #[method_id(@__retain_semantics Other stateUpdateEvent)]
        pub unsafe fn stateUpdateEvent(&self) -> Id<CKSyncEngineStateUpdateEvent>;

        #[method_id(@__retain_semantics Other accountChangeEvent)]
        pub unsafe fn accountChangeEvent(&self) -> Id<CKSyncEngineAccountChangeEvent>;

        #[method_id(@__retain_semantics Other fetchedDatabaseChangesEvent)]
        pub unsafe fn fetchedDatabaseChangesEvent(
            &self,
        ) -> Id<CKSyncEngineFetchedDatabaseChangesEvent>;

        #[method_id(@__retain_semantics Other fetchedRecordZoneChangesEvent)]
        pub unsafe fn fetchedRecordZoneChangesEvent(
            &self,
        ) -> Id<CKSyncEngineFetchedRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other sentDatabaseChangesEvent)]
        pub unsafe fn sentDatabaseChangesEvent(&self) -> Id<CKSyncEngineSentDatabaseChangesEvent>;

        #[method_id(@__retain_semantics Other sentRecordZoneChangesEvent)]
        pub unsafe fn sentRecordZoneChangesEvent(
            &self,
        ) -> Id<CKSyncEngineSentRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other willFetchChangesEvent)]
        pub unsafe fn willFetchChangesEvent(&self) -> Id<CKSyncEngineWillFetchChangesEvent>;

        #[method_id(@__retain_semantics Other willFetchRecordZoneChangesEvent)]
        pub unsafe fn willFetchRecordZoneChangesEvent(
            &self,
        ) -> Id<CKSyncEngineWillFetchRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other didFetchRecordZoneChangesEvent)]
        pub unsafe fn didFetchRecordZoneChangesEvent(
            &self,
        ) -> Id<CKSyncEngineDidFetchRecordZoneChangesEvent>;

        #[method_id(@__retain_semantics Other didFetchChangesEvent)]
        pub unsafe fn didFetchChangesEvent(&self) -> Id<CKSyncEngineDidFetchChangesEvent>;

        #[method_id(@__retain_semantics Other willSendChangesEvent)]
        pub unsafe fn willSendChangesEvent(&self) -> Id<CKSyncEngineWillSendChangesEvent>;

        #[method_id(@__retain_semantics Other didSendChangesEvent)]
        pub unsafe fn didSendChangesEvent(&self) -> Id<CKSyncEngineDidSendChangesEvent>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineStateUpdateEvent;

    unsafe impl ClassType for CKSyncEngineStateUpdateEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineStateUpdateEvent {}

unsafe impl Sync for CKSyncEngineStateUpdateEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineStateUpdateEvent {}

extern_methods!(
    unsafe impl CKSyncEngineStateUpdateEvent {
        #[cfg(feature = "CloudKit_CKSyncEngineState")]
        #[method_id(@__retain_semantics Other stateSerialization)]
        pub unsafe fn stateSerialization(&self) -> Id<CKSyncEngineStateSerialization>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineStateUpdateEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKSyncEngineAccountChangeType {
        #[doc(alias = "CKSyncEngineAccountChangeTypeSignIn")]
        SignIn = 0,
        #[doc(alias = "CKSyncEngineAccountChangeTypeSignOut")]
        SignOut = 1,
        #[doc(alias = "CKSyncEngineAccountChangeTypeSwitchAccounts")]
        SwitchAccounts = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineAccountChangeEvent;

    unsafe impl ClassType for CKSyncEngineAccountChangeEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineAccountChangeEvent {}

unsafe impl Sync for CKSyncEngineAccountChangeEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineAccountChangeEvent {}

extern_methods!(
    unsafe impl CKSyncEngineAccountChangeEvent {
        #[method(changeType)]
        pub unsafe fn changeType(&self) -> CKSyncEngineAccountChangeType;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other previousUser)]
        pub unsafe fn previousUser(&self) -> Option<Id<CKRecordID>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other currentUser)]
        pub unsafe fn currentUser(&self) -> Option<Id<CKRecordID>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineAccountChangeEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedDatabaseChangesEvent;

    unsafe impl ClassType for CKSyncEngineFetchedDatabaseChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineFetchedDatabaseChangesEvent {}

unsafe impl Sync for CKSyncEngineFetchedDatabaseChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedDatabaseChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedDatabaseChangesEvent {
        #[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other modifications)]
        pub unsafe fn modifications(&self) -> Id<NSArray<CKRecordZone>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other deletions)]
        pub unsafe fn deletions(&self) -> Id<NSArray<CKSyncEngineFetchedZoneDeletion>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineFetchedDatabaseChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedRecordZoneChangesEvent;

    unsafe impl ClassType for CKSyncEngineFetchedRecordZoneChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineFetchedRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineFetchedRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedRecordZoneChangesEvent {
        #[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other modifications)]
        pub unsafe fn modifications(&self) -> Id<NSArray<CKRecord>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other deletions)]
        pub unsafe fn deletions(&self) -> Id<NSArray<CKSyncEngineFetchedRecordDeletion>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineFetchedRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineSentDatabaseChangesEvent;

    unsafe impl ClassType for CKSyncEngineSentDatabaseChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineSentDatabaseChangesEvent {}

unsafe impl Sync for CKSyncEngineSentDatabaseChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineSentDatabaseChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineSentDatabaseChangesEvent {
        #[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other savedZones)]
        pub unsafe fn savedZones(&self) -> Id<NSArray<CKRecordZone>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other failedZoneSaves)]
        pub unsafe fn failedZoneSaves(&self) -> Id<NSArray<CKSyncEngineFailedZoneSave>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other deletedZoneIDs)]
        pub unsafe fn deletedZoneIDs(&self) -> Id<NSArray<CKRecordZoneID>>;

        #[cfg(all(
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other failedZoneDeletes)]
        pub unsafe fn failedZoneDeletes(&self) -> Id<NSDictionary<CKRecordZoneID, NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineSentDatabaseChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineSentRecordZoneChangesEvent;

    unsafe impl ClassType for CKSyncEngineSentRecordZoneChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineSentRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineSentRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineSentRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineSentRecordZoneChangesEvent {
        #[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other savedRecords)]
        pub unsafe fn savedRecords(&self) -> Id<NSArray<CKRecord>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other failedRecordSaves)]
        pub unsafe fn failedRecordSaves(&self) -> Id<NSArray<CKSyncEngineFailedRecordSave>>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other deletedRecordIDs)]
        pub unsafe fn deletedRecordIDs(&self) -> Id<NSArray<CKRecordID>>;

        #[cfg(all(
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other failedRecordDeletes)]
        pub unsafe fn failedRecordDeletes(&self) -> Id<NSDictionary<CKRecordID, NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineSentRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineWillFetchChangesEvent;

    unsafe impl ClassType for CKSyncEngineWillFetchChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineWillFetchChangesEvent {}

unsafe impl Sync for CKSyncEngineWillFetchChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineWillFetchChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineWillFetchChangesEvent {
        #[cfg(feature = "CloudKit_CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<CKSyncEngineFetchChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineWillFetchChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineWillFetchRecordZoneChangesEvent;

    unsafe impl ClassType for CKSyncEngineWillFetchRecordZoneChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineWillFetchRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineWillFetchRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineWillFetchRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineWillFetchRecordZoneChangesEvent {
        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Id<CKRecordZoneID>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineWillFetchRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineDidFetchRecordZoneChangesEvent;

    unsafe impl ClassType for CKSyncEngineDidFetchRecordZoneChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineDidFetchRecordZoneChangesEvent {}

unsafe impl Sync for CKSyncEngineDidFetchRecordZoneChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineDidFetchRecordZoneChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineDidFetchRecordZoneChangesEvent {
        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Id<CKRecordZoneID>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineDidFetchRecordZoneChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineDidFetchChangesEvent;

    unsafe impl ClassType for CKSyncEngineDidFetchChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineDidFetchChangesEvent {}

unsafe impl Sync for CKSyncEngineDidFetchChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineDidFetchChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineDidFetchChangesEvent {
        #[cfg(feature = "CloudKit_CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<CKSyncEngineFetchChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineDidFetchChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineWillSendChangesEvent;

    unsafe impl ClassType for CKSyncEngineWillSendChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineWillSendChangesEvent {}

unsafe impl Sync for CKSyncEngineWillSendChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineWillSendChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineWillSendChangesEvent {
        #[cfg(feature = "CloudKit_CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<CKSyncEngineSendChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineWillSendChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineDidSendChangesEvent;

    unsafe impl ClassType for CKSyncEngineDidSendChangesEvent {
        #[inherits(NSObject)]
        type Super = CKSyncEngineEvent;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineDidSendChangesEvent {}

unsafe impl Sync for CKSyncEngineDidSendChangesEvent {}

unsafe impl NSObjectProtocol for CKSyncEngineDidSendChangesEvent {}

extern_methods!(
    unsafe impl CKSyncEngineDidSendChangesEvent {
        #[cfg(feature = "CloudKit_CKSyncEngine")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<CKSyncEngineSendChangesContext>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKSyncEngineEvent`
    unsafe impl CKSyncEngineDidSendChangesEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedRecordDeletion;

    unsafe impl ClassType for CKSyncEngineFetchedRecordDeletion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineFetchedRecordDeletion {}

unsafe impl Sync for CKSyncEngineFetchedRecordDeletion {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedRecordDeletion {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedRecordDeletion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Id<CKRecordID>;

        #[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Id<CKRecordType>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKSyncEngineZoneDeletionReason {
        #[doc(alias = "CKSyncEngineZoneDeletionReasonDeleted")]
        Deleted = 0,
        #[doc(alias = "CKSyncEngineZoneDeletionReasonPurged")]
        Purged = 1,
        #[doc(alias = "CKSyncEngineZoneDeletionReasonEncryptedDataReset")]
        EncryptedDataReset = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchedZoneDeletion;

    unsafe impl ClassType for CKSyncEngineFetchedZoneDeletion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineFetchedZoneDeletion {}

unsafe impl Sync for CKSyncEngineFetchedZoneDeletion {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchedZoneDeletion {}

extern_methods!(
    unsafe impl CKSyncEngineFetchedZoneDeletion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Id<CKRecordZoneID>;

        #[method(reason)]
        pub unsafe fn reason(&self) -> CKSyncEngineZoneDeletionReason;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFailedRecordSave;

    unsafe impl ClassType for CKSyncEngineFailedRecordSave {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineFailedRecordSave {}

unsafe impl Sync for CKSyncEngineFailedRecordSave {}

unsafe impl NSObjectProtocol for CKSyncEngineFailedRecordSave {}

extern_methods!(
    unsafe impl CKSyncEngineFailedRecordSave {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecord")]
        #[method_id(@__retain_semantics Other record)]
        pub unsafe fn record(&self) -> Id<CKRecord>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Id<NSError>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFailedZoneSave;

    unsafe impl ClassType for CKSyncEngineFailedZoneSave {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKSyncEngineFailedZoneSave {}

unsafe impl Sync for CKSyncEngineFailedZoneSave {}

unsafe impl NSObjectProtocol for CKSyncEngineFailedZoneSave {}

extern_methods!(
    unsafe impl CKSyncEngineFailedZoneSave {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZone")]
        #[method_id(@__retain_semantics Other recordZone)]
        pub unsafe fn recordZone(&self) -> Id<CKRecordZone>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Id<NSError>;
    }
);
