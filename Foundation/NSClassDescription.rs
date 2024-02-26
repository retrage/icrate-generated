//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSClassDescription;

    unsafe impl ClassType for NSClassDescription {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSClassDescription {}

extern_methods!(
    unsafe impl NSClassDescription {
        #[method(registerClassDescription:forClass:)]
        pub unsafe fn registerClassDescription_forClass(
            description: &NSClassDescription,
            a_class: &AnyClass,
        );

        #[method(invalidateClassDescriptionCache)]
        pub unsafe fn invalidateClassDescriptionCache();

        #[method_id(@__retain_semantics Other classDescriptionForClass:)]
        pub unsafe fn classDescriptionForClass(
            a_class: &AnyClass,
        ) -> Option<Id<NSClassDescription>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other attributeKeys)]
        pub unsafe fn attributeKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other toOneRelationshipKeys)]
        pub unsafe fn toOneRelationshipKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other toManyRelationshipKeys)]
        pub unsafe fn toManyRelationshipKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other inverseForRelationshipKey:)]
        pub unsafe fn inverseForRelationshipKey(
            &self,
            relationship_key: &NSString,
        ) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSClassDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_category!(
    /// Category "NSClassDescriptionPrimitives" on [`NSObject`].
    #[doc(alias = "NSClassDescriptionPrimitives")]
    pub unsafe trait NSObjectNSClassDescriptionPrimitives {
        #[cfg(feature = "Foundation_NSClassDescription")]
        #[method_id(@__retain_semantics Other classDescription)]
        unsafe fn classDescription(&self) -> Id<NSClassDescription>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other attributeKeys)]
        unsafe fn attributeKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other toOneRelationshipKeys)]
        unsafe fn toOneRelationshipKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other toManyRelationshipKeys)]
        unsafe fn toManyRelationshipKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other inverseForRelationshipKey:)]
        unsafe fn inverseForRelationshipKey(
            &self,
            relationship_key: &NSString,
        ) -> Option<Id<NSString>>;
    }

    unsafe impl NSObjectNSClassDescriptionPrimitives for NSObject {}
);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSClassDescriptionNeededForClassNotification: &'static NSNotificationName);
