//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AMWorkflow;

    unsafe impl ClassType for AMWorkflow {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for AMWorkflow {}

unsafe impl NSObjectProtocol for AMWorkflow {}

extern_methods!(
    unsafe impl AMWorkflow {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other runWorkflowAtURL:withInput:error:_)]
        pub unsafe fn runWorkflowAtURL_withInput_error(
            file_url: &NSURL,
            input: Option<&AnyObject>,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Allocated<Self>,
            file_url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writeToURL:error:_)]
        pub unsafe fn writeToURL_error(&self, file_url: &NSURL) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forVariableWithName:)]
        pub unsafe fn setValue_forVariableWithName(
            &self,
            value: Option<&AnyObject>,
            variable_name: &NSString,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForVariableWithName:)]
        pub unsafe fn valueForVariableWithName(
            &self,
            variable_name: &NSString,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Automator_AMAction")]
        #[method(addAction:)]
        pub unsafe fn addAction(&self, action: &AMAction);

        #[cfg(feature = "Automator_AMAction")]
        #[method(removeAction:)]
        pub unsafe fn removeAction(&self, action: &AMAction);

        #[cfg(feature = "Automator_AMAction")]
        #[method(insertAction:atIndex:)]
        pub unsafe fn insertAction_atIndex(&self, action: &AMAction, index: NSUInteger);

        #[method(moveActionAtIndex:toIndex:)]
        pub unsafe fn moveActionAtIndex_toIndex(
            &self,
            start_index: NSUInteger,
            end_index: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<NSURL>>;

        #[cfg(all(feature = "Automator_AMAction", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other actions)]
        pub unsafe fn actions(&self) -> Id<NSArray<AMAction>>;

        #[method_id(@__retain_semantics Other input)]
        pub unsafe fn input(&self) -> Option<Id<AnyObject>>;

        #[method(setInput:)]
        pub unsafe fn setInput(&self, input: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other output)]
        pub unsafe fn output(&self) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AMWorkflow {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
