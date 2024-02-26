//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVisibleFunctionTableDescriptor;

    unsafe impl ClassType for MTLVisibleFunctionTableDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLVisibleFunctionTableDescriptor {}

unsafe impl NSObjectProtocol for MTLVisibleFunctionTableDescriptor {}

extern_methods!(
    unsafe impl MTLVisibleFunctionTableDescriptor {
        #[method_id(@__retain_semantics Other visibleFunctionTableDescriptor)]
        pub unsafe fn visibleFunctionTableDescriptor() -> Id<MTLVisibleFunctionTableDescriptor>;

        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        #[method(setFunctionCount:)]
        pub unsafe fn setFunctionCount(&self, function_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVisibleFunctionTableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    #[cfg(feature = "Metal_MTLResource")]
    pub unsafe trait MTLVisibleFunctionTable: MTLResource {
        #[cfg(feature = "Metal_MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "Metal_MTLFunctionHandle")]
        #[method(setFunction:atIndex:)]
        unsafe fn setFunction_atIndex(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: NSUInteger,
        );

        #[cfg(all(feature = "Foundation_NSRange", feature = "Metal_MTLFunctionHandle"))]
        #[method(setFunctions:withRange:)]
        unsafe fn setFunctions_withRange(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
        );
    }

    #[cfg(feature = "Metal_MTLResource")]
    unsafe impl ProtocolType for dyn MTLVisibleFunctionTable {}
);
