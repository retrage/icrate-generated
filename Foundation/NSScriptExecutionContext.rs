//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptExecutionContext;

    unsafe impl ClassType for NSScriptExecutionContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSScriptExecutionContext {}

extern_methods!(
    unsafe impl NSScriptExecutionContext {
        #[method_id(@__retain_semantics Other sharedScriptExecutionContext)]
        pub unsafe fn sharedScriptExecutionContext() -> Id<NSScriptExecutionContext>;

        #[method_id(@__retain_semantics Other topLevelObject)]
        pub unsafe fn topLevelObject(&self) -> Option<Id<AnyObject>>;

        #[method(setTopLevelObject:)]
        pub unsafe fn setTopLevelObject(&self, top_level_object: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other objectBeingTested)]
        pub unsafe fn objectBeingTested(&self) -> Option<Id<AnyObject>>;

        #[method(setObjectBeingTested:)]
        pub unsafe fn setObjectBeingTested(&self, object_being_tested: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other rangeContainerObject)]
        pub unsafe fn rangeContainerObject(&self) -> Option<Id<AnyObject>>;

        #[method(setRangeContainerObject:)]
        pub unsafe fn setRangeContainerObject(&self, range_container_object: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScriptExecutionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
