//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "ClassKit_CLSActivityItem", feature = "ClassKit_CLSObject"))]
    pub struct CLSQuantityItem;

    #[cfg(all(feature = "ClassKit_CLSActivityItem", feature = "ClassKit_CLSObject"))]
    unsafe impl ClassType for CLSQuantityItem {
        #[inherits(CLSObject, NSObject)]
        type Super = CLSActivityItem;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "ClassKit_CLSActivityItem",
    feature = "ClassKit_CLSObject",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for CLSQuantityItem {}

#[cfg(all(feature = "ClassKit_CLSActivityItem", feature = "ClassKit_CLSObject"))]
unsafe impl NSObjectProtocol for CLSQuantityItem {}

#[cfg(all(
    feature = "ClassKit_CLSActivityItem",
    feature = "ClassKit_CLSObject",
    feature = "Foundation_NSObject"
))]
unsafe impl NSSecureCoding for CLSQuantityItem {}

extern_methods!(
    #[cfg(all(feature = "ClassKit_CLSActivityItem", feature = "ClassKit_CLSObject"))]
    unsafe impl CLSQuantityItem {
        #[method(quantity)]
        pub unsafe fn quantity(&self) -> c_double;

        #[method(setQuantity:)]
        pub unsafe fn setQuantity(&self, quantity: c_double);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:title:)]
        pub unsafe fn initWithIdentifier_title(
            this: Allocated<Self>,
            identifier: &NSString,
            title: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLSActivityItem`
    #[cfg(all(feature = "ClassKit_CLSActivityItem", feature = "ClassKit_CLSObject"))]
    unsafe impl CLSQuantityItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
