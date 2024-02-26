//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    pub struct ILMessageClassificationRequest;

    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    unsafe impl ClassType for ILMessageClassificationRequest {
        #[inherits(NSObject)]
        type Super = ILClassificationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "IdentityLookup_ILClassificationRequest"
))]
unsafe impl NSCoding for ILMessageClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
unsafe impl NSObjectProtocol for ILMessageClassificationRequest {}

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "IdentityLookup_ILClassificationRequest"
))]
unsafe impl NSSecureCoding for ILMessageClassificationRequest {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    unsafe impl ILMessageClassificationRequest {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "IdentityLookup_ILCommunication",
            feature = "IdentityLookup_ILMessageCommunication"
        ))]
        #[method_id(@__retain_semantics Other messageCommunications)]
        pub unsafe fn messageCommunications(&self) -> Id<NSArray<ILMessageCommunication>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    unsafe impl ILMessageClassificationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
