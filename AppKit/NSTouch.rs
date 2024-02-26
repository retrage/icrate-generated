//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTouchPhase {
        #[doc(alias = "NSTouchPhaseBegan")]
        Began = 1 << 0,
        #[doc(alias = "NSTouchPhaseMoved")]
        Moved = 1 << 1,
        #[doc(alias = "NSTouchPhaseStationary")]
        Stationary = 1 << 2,
        #[doc(alias = "NSTouchPhaseEnded")]
        Ended = 1 << 3,
        #[doc(alias = "NSTouchPhaseCancelled")]
        Cancelled = 1 << 4,
        #[doc(alias = "NSTouchPhaseTouching")]
        Touching = NSTouchPhase::Began.0 | NSTouchPhase::Moved.0 | NSTouchPhase::Stationary.0,
        #[doc(alias = "NSTouchPhaseAny")]
        Any = NSUIntegerMax as _,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTouchType {
        #[doc(alias = "NSTouchTypeDirect")]
        Direct = 0,
        #[doc(alias = "NSTouchTypeIndirect")]
        Indirect = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTouchTypeMask {
        #[doc(alias = "NSTouchTypeMaskDirect")]
        Direct = 1 << NSTouchType::Direct.0,
        #[doc(alias = "NSTouchTypeMaskIndirect")]
        Indirect = 1 << NSTouchType::Indirect.0,
    }
);

inline_fn!(
    pub unsafe fn NSTouchTypeMaskFromType(r#type: NSTouchType) -> NSTouchTypeMask {
        todo!()
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouch;

    unsafe impl ClassType for NSTouch {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

unsafe impl Send for NSTouch {}

unsafe impl Sync for NSTouch {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSTouch {}

unsafe impl NSObjectProtocol for NSTouch {}

extern_methods!(
    unsafe impl NSTouch {
        #[cfg(feature = "Foundation_NSObject")]
        #[method_id(@__retain_semantics Other identity)]
        pub unsafe fn identity(&self) -> Id<TodoProtocols>;

        #[method(phase)]
        pub unsafe fn phase(&self) -> NSTouchPhase;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(normalizedPosition)]
        pub unsafe fn normalizedPosition(&self) -> NSPoint;

        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(deviceSize)]
        pub unsafe fn deviceSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTouch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSTouchBar
    unsafe impl NSTouch {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSTouchType;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method(previousLocationInView:)]
        pub unsafe fn previousLocationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);
