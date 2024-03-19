//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIndirectCommandType(pub NSUInteger);
impl MTLIndirectCommandType {
    #[doc(alias = "MTLIndirectCommandTypeDraw")]
    pub const Draw: Self = Self(1 << 0);
    #[doc(alias = "MTLIndirectCommandTypeDrawIndexed")]
    pub const DrawIndexed: Self = Self(1 << 1);
    #[doc(alias = "MTLIndirectCommandTypeDrawPatches")]
    pub const DrawPatches: Self = Self(1 << 2);
    #[doc(alias = "MTLIndirectCommandTypeDrawIndexedPatches")]
    pub const DrawIndexedPatches: Self = Self(1 << 3);
    #[doc(alias = "MTLIndirectCommandTypeConcurrentDispatch")]
    pub const ConcurrentDispatch: Self = Self(1 << 5);
    #[doc(alias = "MTLIndirectCommandTypeConcurrentDispatchThreads")]
    pub const ConcurrentDispatchThreads: Self = Self(1 << 6);
    #[doc(alias = "MTLIndirectCommandTypeDrawMeshThreadgroups")]
    pub const DrawMeshThreadgroups: Self = Self(1 << 7);
    #[doc(alias = "MTLIndirectCommandTypeDrawMeshThreads")]
    pub const DrawMeshThreads: Self = Self(1 << 8);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MTLIndirectCommandType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MTLIndirectCommandType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLIndirectCommandBufferExecutionRange {
        pub location: u32,
        pub length: u32,
    }
);

// TODO: pub fn MTLIndirectCommandBufferExecutionRangeMake(location: u32,length: u32,) -> MTLIndirectCommandBufferExecutionRange;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIndirectCommandBufferDescriptor;

    unsafe impl ClassType for MTLIndirectCommandBufferDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLIndirectCommandBufferDescriptor {}

unsafe impl NSObjectProtocol for MTLIndirectCommandBufferDescriptor {}

extern_methods!(
    unsafe impl MTLIndirectCommandBufferDescriptor {
        #[method(commandTypes)]
        pub fn commandTypes(&self) -> MTLIndirectCommandType;

        #[method(setCommandTypes:)]
        pub fn setCommandTypes(&self, command_types: MTLIndirectCommandType);

        #[method(inheritPipelineState)]
        pub fn inheritPipelineState(&self) -> bool;

        #[method(setInheritPipelineState:)]
        pub fn setInheritPipelineState(&self, inherit_pipeline_state: bool);

        #[method(inheritBuffers)]
        pub fn inheritBuffers(&self) -> bool;

        #[method(setInheritBuffers:)]
        pub fn setInheritBuffers(&self, inherit_buffers: bool);

        #[method(maxVertexBufferBindCount)]
        pub fn maxVertexBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxVertexBufferBindCount:)]
        pub fn setMaxVertexBufferBindCount(&self, max_vertex_buffer_bind_count: NSUInteger);

        #[method(maxFragmentBufferBindCount)]
        pub fn maxFragmentBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxFragmentBufferBindCount:)]
        pub fn setMaxFragmentBufferBindCount(&self, max_fragment_buffer_bind_count: NSUInteger);

        #[method(maxKernelBufferBindCount)]
        pub fn maxKernelBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxKernelBufferBindCount:)]
        pub fn setMaxKernelBufferBindCount(&self, max_kernel_buffer_bind_count: NSUInteger);

        #[method(maxKernelThreadgroupMemoryBindCount)]
        pub unsafe fn maxKernelThreadgroupMemoryBindCount(&self) -> NSUInteger;

        #[method(setMaxKernelThreadgroupMemoryBindCount:)]
        pub unsafe fn setMaxKernelThreadgroupMemoryBindCount(
            &self,
            max_kernel_threadgroup_memory_bind_count: NSUInteger,
        );

        #[method(maxObjectBufferBindCount)]
        pub unsafe fn maxObjectBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxObjectBufferBindCount:)]
        pub unsafe fn setMaxObjectBufferBindCount(&self, max_object_buffer_bind_count: NSUInteger);

        #[method(maxMeshBufferBindCount)]
        pub unsafe fn maxMeshBufferBindCount(&self) -> NSUInteger;

        #[method(setMaxMeshBufferBindCount:)]
        pub unsafe fn setMaxMeshBufferBindCount(&self, max_mesh_buffer_bind_count: NSUInteger);

        #[method(maxObjectThreadgroupMemoryBindCount)]
        pub unsafe fn maxObjectThreadgroupMemoryBindCount(&self) -> NSUInteger;

        #[method(setMaxObjectThreadgroupMemoryBindCount:)]
        pub unsafe fn setMaxObjectThreadgroupMemoryBindCount(
            &self,
            max_object_threadgroup_memory_bind_count: NSUInteger,
        );

        #[method(supportRayTracing)]
        pub unsafe fn supportRayTracing(&self) -> bool;

        #[method(setSupportRayTracing:)]
        pub unsafe fn setSupportRayTracing(&self, support_ray_tracing: bool);

        #[method(supportDynamicAttributeStride)]
        pub unsafe fn supportDynamicAttributeStride(&self) -> bool;

        #[method(setSupportDynamicAttributeStride:)]
        pub unsafe fn setSupportDynamicAttributeStride(
            &self,
            support_dynamic_attribute_stride: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLIndirectCommandBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    #[cfg(feature = "Metal_MTLResource")]
    pub unsafe trait MTLIndirectCommandBuffer: MTLResource {
        #[method(size)]
        fn size(&self) -> NSUInteger;

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(resetWithRange:)]
        unsafe fn resetWithRange(&self, range: NSRange);

        #[cfg(feature = "Metal_MTLIndirectCommandEncoder")]
        #[method_id(@__retain_semantics Other indirectRenderCommandAtIndex:)]
        unsafe fn indirectRenderCommandAtIndex(
            &self,
            command_index: NSUInteger,
        ) -> Id<ProtocolObject<dyn MTLIndirectRenderCommand>>;

        #[cfg(feature = "Metal_MTLIndirectCommandEncoder")]
        #[method_id(@__retain_semantics Other indirectComputeCommandAtIndex:)]
        unsafe fn indirectComputeCommandAtIndex(
            &self,
            command_index: NSUInteger,
        ) -> Id<ProtocolObject<dyn MTLIndirectComputeCommand>>;
    }

    #[cfg(feature = "Metal_MTLResource")]
    unsafe impl ProtocolType for dyn MTLIndirectCommandBuffer {}
);
