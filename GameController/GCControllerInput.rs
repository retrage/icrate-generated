//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCControllerInputState;

    unsafe impl ClassType for GCControllerInputState {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCControllerInputState {}

unsafe impl NSObjectProtocol for GCControllerInputState {}

extern_methods!(
    unsafe impl GCControllerInputState {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCControllerInputState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCControllerLiveInput;

    unsafe impl ClassType for GCControllerLiveInput {
        #[inherits(NSObject)]
        type Super = GCControllerInputState;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "GameController_GCDevicePhysicalInput",
    feature = "GameController_GCDevicePhysicalInputState"
))]
unsafe impl GCDevicePhysicalInput for GCControllerLiveInput {}

#[cfg(feature = "GameController_GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCControllerLiveInput {}

unsafe impl NSObjectProtocol for GCControllerLiveInput {}

extern_methods!(
    unsafe impl GCControllerLiveInput {
        #[method_id(@__retain_semantics Other unmappedInput)]
        pub unsafe fn unmappedInput(&self) -> Option<Id<GCControllerLiveInput>>;

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCControllerInputState>;

        #[cfg(feature = "GameController_GCDevicePhysicalInputStateDiff")]
        #[method_id(@__retain_semantics Other nextInputState)]
        pub unsafe fn nextInputState(&self) -> Option<Id<GCControllerInputState>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCControllerLiveInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
