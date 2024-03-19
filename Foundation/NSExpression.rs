//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSExpressionType(pub NSUInteger);
impl NSExpressionType {
    pub const NSConstantValueExpressionType: Self = Self(0);
    pub const NSEvaluatedObjectExpressionType: Self = Self(1);
    pub const NSVariableExpressionType: Self = Self(2);
    pub const NSKeyPathExpressionType: Self = Self(3);
    pub const NSFunctionExpressionType: Self = Self(4);
    pub const NSUnionSetExpressionType: Self = Self(5);
    pub const NSIntersectSetExpressionType: Self = Self(6);
    pub const NSMinusSetExpressionType: Self = Self(7);
    pub const NSSubqueryExpressionType: Self = Self(13);
    pub const NSAggregateExpressionType: Self = Self(14);
    pub const NSAnyKeyExpressionType: Self = Self(15);
    pub const NSBlockExpressionType: Self = Self(19);
    pub const NSConditionalExpressionType: Self = Self(20);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSExpressionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSExpressionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSExpression;

    unsafe impl ClassType for NSExpression {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSExpression {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSExpression {}

unsafe impl NSObjectProtocol for NSExpression {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSExpression {}

extern_methods!(
    unsafe impl NSExpression {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionWithFormat:argumentArray:)]
        pub unsafe fn expressionWithFormat_argumentArray(
            expression_format: &NSString,
            arguments: &NSArray,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForConstantValue:)]
        pub unsafe fn expressionForConstantValue(obj: Option<&AnyObject>) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForEvaluatedObject)]
        pub unsafe fn expressionForEvaluatedObject() -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other expressionForVariable:)]
        pub unsafe fn expressionForVariable(string: &NSString) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other expressionForKeyPath:)]
        pub unsafe fn expressionForKeyPath(key_path: &NSString) -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionForFunction:arguments:)]
        pub unsafe fn expressionForFunction_arguments(
            name: &NSString,
            parameters: &NSArray,
        ) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other expressionForAggregate:)]
        pub unsafe fn expressionForAggregate(
            subexpressions: &NSArray<NSExpression>,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForUnionSet:with:)]
        pub unsafe fn expressionForUnionSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForIntersectSet:with:)]
        pub unsafe fn expressionForIntersectSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForMinusSet:with:)]
        pub unsafe fn expressionForMinusSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSPredicate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionForSubquery:usingIteratorVariable:predicate:)]
        pub unsafe fn expressionForSubquery_usingIteratorVariable_predicate(
            expression: &NSExpression,
            variable: &NSString,
            predicate: &NSPredicate,
        ) -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionForFunction:selectorName:arguments:)]
        pub unsafe fn expressionForFunction_selectorName_arguments(
            target: &NSExpression,
            name: &NSString,
            parameters: Option<&NSArray>,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForAnyKey)]
        pub unsafe fn expressionForAnyKey() -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other expressionForBlock:arguments:)]
        pub unsafe fn expressionForBlock_arguments(
            block: &Block<
                dyn Fn(
                    *mut AnyObject,
                    NonNull<NSArray<NSExpression>>,
                    *mut NSMutableDictionary,
                ) -> NonNull<AnyObject>,
            >,
            arguments: Option<&NSArray<NSExpression>>,
        ) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other expressionForConditional:trueExpression:falseExpression:)]
        pub unsafe fn expressionForConditional_trueExpression_falseExpression(
            predicate: &NSPredicate,
            true_expression: &NSExpression,
            false_expression: &NSExpression,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Init initWithExpressionType:)]
        pub unsafe fn initWithExpressionType(
            this: Allocated<Self>,
            r#type: NSExpressionType,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method(expressionType)]
        pub unsafe fn expressionType(&self) -> NSExpressionType;

        #[method_id(@__retain_semantics Other constantValue)]
        pub unsafe fn constantValue(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other function)]
        pub unsafe fn function(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other variable)]
        pub unsafe fn variable(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other operand)]
        pub unsafe fn operand(&self) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSExpression>>>;

        #[method_id(@__retain_semantics Other collection)]
        pub unsafe fn collection(&self) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Id<NSPredicate>;

        #[method_id(@__retain_semantics Other leftExpression)]
        pub unsafe fn leftExpression(&self) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other rightExpression)]
        pub unsafe fn rightExpression(&self) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other trueExpression)]
        pub unsafe fn trueExpression(&self) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other falseExpression)]
        pub unsafe fn falseExpression(&self) -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method(expressionBlock)]
        pub unsafe fn expressionBlock(
            &self,
        ) -> NonNull<
            Block<
                dyn Fn(
                    *mut AnyObject,
                    NonNull<NSArray<NSExpression>>,
                    *mut NSMutableDictionary,
                ) -> NonNull<AnyObject>,
            >,
        >;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other expressionValueWithObject:context:)]
        pub unsafe fn expressionValueWithObject_context(
            &self,
            object: Option<&AnyObject>,
            context: Option<&NSMutableDictionary>,
        ) -> Option<Id<AnyObject>>;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSExpression {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
