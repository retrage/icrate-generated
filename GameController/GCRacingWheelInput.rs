//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCRacingWheelInputState;

    unsafe impl ClassType for GCRacingWheelInputState {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCRacingWheelInputState {}

unsafe impl NSObjectProtocol for GCRacingWheelInputState {}

extern_methods!(
    unsafe impl GCRacingWheelInputState {
        #[cfg(feature = "GameController_GCSteeringWheelElement")]
        #[method_id(@__retain_semantics Other wheel)]
        pub unsafe fn wheel(&self) -> Id<GCSteeringWheelElement>;

        #[cfg(all(
            feature = "GameController_GCButtonElement",
            feature = "GameController_GCPhysicalInputElement"
        ))]
        #[method_id(@__retain_semantics Other acceleratorPedal)]
        pub unsafe fn acceleratorPedal(&self) -> Option<Id<ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(all(
            feature = "GameController_GCButtonElement",
            feature = "GameController_GCPhysicalInputElement"
        ))]
        #[method_id(@__retain_semantics Other brakePedal)]
        pub unsafe fn brakePedal(&self) -> Option<Id<ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(all(
            feature = "GameController_GCButtonElement",
            feature = "GameController_GCPhysicalInputElement"
        ))]
        #[method_id(@__retain_semantics Other clutchPedal)]
        pub unsafe fn clutchPedal(&self) -> Option<Id<ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(feature = "GameController_GCGearShifterElement")]
        #[method_id(@__retain_semantics Other shifter)]
        pub unsafe fn shifter(&self) -> Option<Id<GCGearShifterElement>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCRacingWheelInputState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCRacingWheelInput;

    unsafe impl ClassType for GCRacingWheelInput {
        #[inherits(NSObject)]
        type Super = GCRacingWheelInputState;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "GameController_GCDevicePhysicalInput",
    feature = "GameController_GCDevicePhysicalInputState"
))]
unsafe impl GCDevicePhysicalInput for GCRacingWheelInput {}

#[cfg(feature = "GameController_GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCRacingWheelInput {}

unsafe impl NSObjectProtocol for GCRacingWheelInput {}

extern_methods!(
    unsafe impl GCRacingWheelInput {
        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCRacingWheelInputState>;

        #[cfg(feature = "GameController_GCDevicePhysicalInputStateDiff")]
        #[method_id(@__retain_semantics Other nextInputState)]
        pub unsafe fn nextInputState(&self) -> Option<Id<GCRacingWheelInputState>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCRacingWheelInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
