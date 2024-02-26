//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXCallAction"))]
    pub struct CXEndCallAction;

    #[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXCallAction"))]
    unsafe impl ClassType for CXEndCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for CXEndCallAction {}

#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCopying for CXEndCallAction {}

#[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXCallAction"))]
unsafe impl NSObjectProtocol for CXEndCallAction {}

#[cfg(all(
    feature = "CallKit_CXAction",
    feature = "CallKit_CXCallAction",
    feature = "Foundation_NSObject"
))]
unsafe impl NSSecureCoding for CXEndCallAction {}

extern_methods!(
    #[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXCallAction"))]
    unsafe impl CXEndCallAction {
        #[cfg(feature = "Foundation_NSDate")]
        #[method(fulfillWithDateEnded:)]
        pub unsafe fn fulfillWithDateEnded(&self, date_ended: &NSDate);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXCallAction"))]
    unsafe impl CXEndCallAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder)
            -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CallKit_CXAction", feature = "CallKit_CXCallAction"))]
    unsafe impl CXEndCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
