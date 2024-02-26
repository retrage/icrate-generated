//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
typed_extensible_enum!(
    pub type NSValueTransformerName = NSString;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSNegateBooleanTransformerName: &'static NSValueTransformerName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSIsNilTransformerName: &'static NSValueTransformerName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSIsNotNilTransformerName: &'static NSValueTransformerName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSUnarchiveFromDataTransformerName: &'static NSValueTransformerName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSKeyedUnarchiveFromDataTransformerName: &'static NSValueTransformerName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSSecureUnarchiveFromDataTransformerName: &'static NSValueTransformerName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSValueTransformer;

    unsafe impl ClassType for NSValueTransformer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSValueTransformer {}

extern_methods!(
    unsafe impl NSValueTransformer {
        #[cfg(feature = "Foundation_NSString")]
        #[method(setValueTransformer:forName:)]
        pub unsafe fn setValueTransformer_forName(
            transformer: Option<&NSValueTransformer>,
            name: &NSValueTransformerName,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueTransformerForName:)]
        pub unsafe fn valueTransformerForName(
            name: &NSValueTransformerName,
        ) -> Option<Id<NSValueTransformer>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other valueTransformerNames)]
        pub unsafe fn valueTransformerNames() -> Id<NSArray<NSValueTransformerName>>;

        #[method(transformedValueClass)]
        pub unsafe fn transformedValueClass() -> &'static AnyClass;

        #[method(allowsReverseTransformation)]
        pub unsafe fn allowsReverseTransformation() -> bool;

        #[method_id(@__retain_semantics Other transformedValue:)]
        pub unsafe fn transformedValue(&self, value: Option<&AnyObject>) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other reverseTransformedValue:)]
        pub unsafe fn reverseTransformedValue(
            &self,
            value: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSValueTransformer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSecureUnarchiveFromDataTransformer;

    unsafe impl ClassType for NSSecureUnarchiveFromDataTransformer {
        #[inherits(NSObject)]
        type Super = NSValueTransformer;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSSecureUnarchiveFromDataTransformer {}

extern_methods!(
    unsafe impl NSSecureUnarchiveFromDataTransformer {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allowedTopLevelClasses)]
        pub unsafe fn allowedTopLevelClasses() -> Id<NSArray<TodoClass>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSecureUnarchiveFromDataTransformer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
