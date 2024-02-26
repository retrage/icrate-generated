//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "GameController_GCMicroGamepad",
        feature = "GameController_GCPhysicalInputProfile"
    ))]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub struct GCMicroGamepadSnapshot;

    #[cfg(all(
        feature = "GameController_GCMicroGamepad",
        feature = "GameController_GCPhysicalInputProfile"
    ))]
    unsafe impl ClassType for GCMicroGamepadSnapshot {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCMicroGamepad;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "GameController_GCMicroGamepad",
    feature = "GameController_GCPhysicalInputProfile"
))]
unsafe impl NSObjectProtocol for GCMicroGamepadSnapshot {}

extern_methods!(
    #[cfg(all(
        feature = "GameController_GCMicroGamepad",
        feature = "GameController_GCPhysicalInputProfile"
    ))]
    unsafe impl GCMicroGamepadSnapshot {
        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method_id(@__retain_semantics Other snapshotData)]
        pub unsafe fn snapshotData(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method(setSnapshotData:)]
        pub unsafe fn setSnapshotData(&self, snapshot_data: &NSData);

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method_id(@__retain_semantics Init initWithSnapshotData:)]
        pub unsafe fn initWithSnapshotData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "GameController_GCController"))]
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method_id(@__retain_semantics Init initWithController:snapshotData:)]
        pub unsafe fn initWithController_snapshotData(
            this: Allocated<Self>,
            controller: &GCController,
            data: &NSData,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "GameController_GCMicroGamepad",
        feature = "GameController_GCPhysicalInputProfile"
    ))]
    unsafe impl GCMicroGamepadSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub enum GCMicroGamepadSnapshotDataVersion {
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        GCMicroGamepadSnapshotDataVersion1 = 0x0100,
    }
);

extern_static!(GCCurrentMicroGamepadSnapshotDataVersion: GCMicroGamepadSnapshotDataVersion);

extern_struct!(
    #[encoding_name("?")]
    pub struct GCMicroGamepadSnapshotData {
        pub version: u16,
        pub size: u16,
        pub dpadX: c_float,
        pub dpadY: c_float,
        pub buttonA: c_float,
        pub buttonX: c_float,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn GCMicroGamepadSnapshotDataFromNSData(
        snapshot_data: *mut GCMicroGamepadSnapshotData,
        data: Option<&NSData>,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn NSDataFromGCMicroGamepadSnapshotData(
        snapshot_data: *mut GCMicroGamepadSnapshotData,
    ) -> *mut NSData;
);

extern_struct!(
    #[encoding_name("?")]
    pub struct GCMicroGamepadSnapShotDataV100 {
        pub version: u16,
        pub size: u16,
        pub dpadX: c_float,
        pub dpadY: c_float,
        pub buttonA: c_float,
        pub buttonX: c_float,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn GCMicroGamepadSnapShotDataV100FromNSData(
        snapshot_data: *mut GCMicroGamepadSnapShotDataV100,
        data: Option<&NSData>,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn NSDataFromGCMicroGamepadSnapShotDataV100(
        snapshot_data: *mut GCMicroGamepadSnapShotDataV100,
    ) -> *mut NSData;
);
