//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub unsafe trait MTLEvent: NSObjectProtocol {
        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Option<Id<ProtocolObject<dyn MTLDevice>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);
    }

    unsafe impl ProtocolType for dyn MTLEvent {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedEventListener;

    unsafe impl ClassType for MTLSharedEventListener {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MTLSharedEventListener {}

extern_methods!(
    unsafe impl MTLSharedEventListener {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLSharedEventListener {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for MTLSharedEventListener {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

pub type MTLSharedEventNotificationBlock =
    *mut Block<dyn Fn(NonNull<ProtocolObject<dyn MTLSharedEvent>>, u64)>;

extern_protocol!(
    pub unsafe trait MTLSharedEvent: MTLEvent {
        #[method(notifyListener:atValue:block:)]
        unsafe fn notifyListener_atValue_block(
            &self,
            listener: &MTLSharedEventListener,
            value: u64,
            block: MTLSharedEventNotificationBlock,
        );

        #[method_id(@__retain_semantics New newSharedEventHandle)]
        unsafe fn newSharedEventHandle(&self) -> Id<MTLSharedEventHandle>;

        #[method(waitUntilSignaledValue:timeoutMS:)]
        unsafe fn waitUntilSignaledValue_timeoutMS(&self, value: u64, milliseconds: u64) -> bool;

        #[method(signaledValue)]
        unsafe fn signaledValue(&self) -> u64;

        #[method(setSignaledValue:)]
        unsafe fn setSignaledValue(&self, signaled_value: u64);
    }

    unsafe impl ProtocolType for dyn MTLSharedEvent {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedEventHandle;

    unsafe impl ClassType for MTLSharedEventHandle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MTLSharedEventHandle {}

unsafe impl NSObjectProtocol for MTLSharedEventHandle {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MTLSharedEventHandle {}

extern_methods!(
    unsafe impl MTLSharedEventHandle {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLSharedEventHandle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
