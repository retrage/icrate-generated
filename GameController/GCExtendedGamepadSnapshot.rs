//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub struct GCExtendedGamepadSnapshot;

    #[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
    unsafe impl ClassType for GCExtendedGamepadSnapshot {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCExtendedGamepad;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
unsafe impl NSObjectProtocol for GCExtendedGamepadSnapshot {}

extern_methods!(
    #[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
    unsafe impl GCExtendedGamepadSnapshot {
        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        #[method_id(@__retain_semantics Other snapshotData)]
        pub unsafe fn snapshotData(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        #[method(setSnapshotData:)]
        pub unsafe fn setSnapshotData(&self, snapshot_data: &NSData);

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        #[method_id(@__retain_semantics Init initWithSnapshotData:)]
        pub unsafe fn initWithSnapshotData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "GameController_GCController"))]
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
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
    #[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
    unsafe impl GCExtendedGamepadSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub enum GCExtendedGamepadSnapshotDataVersion {
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        GCExtendedGamepadSnapshotDataVersion1 = 0x0100,
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        GCExtendedGamepadSnapshotDataVersion2 = 0x0101,
    }
);

extern_static!(GCCurrentExtendedGamepadSnapshotDataVersion: GCExtendedGamepadSnapshotDataVersion);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub unsafe fn GCExtendedGamepadSnapshotDataFromNSData(
        snapshot_data: *mut GCExtendedGamepadSnapshotData,
        data: Option<&NSData>,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub unsafe fn NSDataFromGCExtendedGamepadSnapshotData(
        snapshot_data: *mut GCExtendedGamepadSnapshotData,
    ) -> *mut NSData;
);

extern_struct!(
    #[encoding_name("?")]
    pub struct GCExtendedGamepadSnapShotDataV100 {
        pub version: u16,
        pub size: u16,
        pub dpadX: c_float,
        pub dpadY: c_float,
        pub buttonA: c_float,
        pub buttonB: c_float,
        pub buttonX: c_float,
        pub buttonY: c_float,
        pub leftShoulder: c_float,
        pub rightShoulder: c_float,
        pub leftThumbstickX: c_float,
        pub leftThumbstickY: c_float,
        pub rightThumbstickX: c_float,
        pub rightThumbstickY: c_float,
        pub leftTrigger: c_float,
        pub rightTrigger: c_float,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub unsafe fn GCExtendedGamepadSnapShotDataV100FromNSData(
        snapshot_data: *mut GCExtendedGamepadSnapShotDataV100,
        data: Option<&NSData>,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub unsafe fn NSDataFromGCExtendedGamepadSnapShotDataV100(
        snapshot_data: *mut GCExtendedGamepadSnapShotDataV100,
    ) -> *mut NSData;
);
