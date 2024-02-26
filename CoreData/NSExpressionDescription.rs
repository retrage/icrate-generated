//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPropertyDescription")]
    pub struct NSExpressionDescription;

    #[cfg(feature = "CoreData_NSPropertyDescription")]
    unsafe impl ClassType for NSExpressionDescription {
        #[inherits(NSObject)]
        type Super = NSPropertyDescription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "CoreData_NSPropertyDescription",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSExpressionDescription {}

#[cfg(all(
    feature = "CoreData_NSPropertyDescription",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCopying for NSExpressionDescription {}

#[cfg(feature = "CoreData_NSPropertyDescription")]
unsafe impl NSObjectProtocol for NSExpressionDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPropertyDescription")]
    unsafe impl NSExpressionDescription {
        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other expression)]
        pub unsafe fn expression(&self) -> Option<Id<NSExpression>>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method(setExpression:)]
        pub unsafe fn setExpression(&self, expression: Option<&NSExpression>);

        #[cfg(feature = "CoreData_NSAttributeDescription")]
        #[method(expressionResultType)]
        pub unsafe fn expressionResultType(&self) -> NSAttributeType;

        #[cfg(feature = "CoreData_NSAttributeDescription")]
        #[method(setExpressionResultType:)]
        pub unsafe fn setExpressionResultType(&self, expression_result_type: NSAttributeType);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPropertyDescription")]
    unsafe impl NSExpressionDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
