//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLMutability {
        #[doc(alias = "MTLMutabilityDefault")]
        Default = 0,
        #[doc(alias = "MTLMutabilityMutable")]
        Mutable = 1,
        #[doc(alias = "MTLMutabilityImmutable")]
        Immutable = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPipelineBufferDescriptor;

    unsafe impl ClassType for MTLPipelineBufferDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLPipelineBufferDescriptor {}

unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptor {}

extern_methods!(
    unsafe impl MTLPipelineBufferDescriptor {
        #[method(mutability)]
        pub fn mutability(&self) -> MTLMutability;

        #[method(setMutability:)]
        pub fn setMutability(&self, mutability: MTLMutability);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLPipelineBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPipelineBufferDescriptorArray;

    unsafe impl ClassType for MTLPipelineBufferDescriptorArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptorArray {}

extern_methods!(
    unsafe impl MTLPipelineBufferDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            buffer_index: NSUInteger,
        ) -> Id<MTLPipelineBufferDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            buffer: Option<&MTLPipelineBufferDescriptor>,
            buffer_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLPipelineBufferDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
