//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIntersectionFunctionSignature(pub NSUInteger);
impl MTLIntersectionFunctionSignature {
    #[doc(alias = "MTLIntersectionFunctionSignatureNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MTLIntersectionFunctionSignatureInstancing")]
    pub const Instancing: Self = Self(1 << 0);
    #[doc(alias = "MTLIntersectionFunctionSignatureTriangleData")]
    pub const TriangleData: Self = Self(1 << 1);
    #[doc(alias = "MTLIntersectionFunctionSignatureWorldSpaceData")]
    pub const WorldSpaceData: Self = Self(1 << 2);
    #[doc(alias = "MTLIntersectionFunctionSignatureInstanceMotion")]
    pub const InstanceMotion: Self = Self(1 << 3);
    #[doc(alias = "MTLIntersectionFunctionSignaturePrimitiveMotion")]
    pub const PrimitiveMotion: Self = Self(1 << 4);
    #[doc(alias = "MTLIntersectionFunctionSignatureExtendedLimits")]
    pub const ExtendedLimits: Self = Self(1 << 5);
    #[doc(alias = "MTLIntersectionFunctionSignatureMaxLevels")]
    pub const MaxLevels: Self = Self(1 << 6);
    #[doc(alias = "MTLIntersectionFunctionSignatureCurveData")]
    pub const CurveData: Self = Self(1 << 7);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MTLIntersectionFunctionSignature {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MTLIntersectionFunctionSignature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIntersectionFunctionTableDescriptor;

    unsafe impl ClassType for MTLIntersectionFunctionTableDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLIntersectionFunctionTableDescriptor {}

unsafe impl NSObjectProtocol for MTLIntersectionFunctionTableDescriptor {}

extern_methods!(
    unsafe impl MTLIntersectionFunctionTableDescriptor {
        #[method_id(@__retain_semantics Other intersectionFunctionTableDescriptor)]
        pub unsafe fn intersectionFunctionTableDescriptor(
        ) -> Id<MTLIntersectionFunctionTableDescriptor>;

        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        #[method(setFunctionCount:)]
        pub fn setFunctionCount(&self, function_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLIntersectionFunctionTableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for MTLIntersectionFunctionTableDescriptor {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_protocol!(
    #[cfg(feature = "Metal_MTLResource")]
    pub unsafe trait MTLIntersectionFunctionTable: MTLResource {
        #[cfg(feature = "Metal_MTLBuffer")]
        #[method(setBuffer:offset:atIndex:)]
        unsafe fn setBuffer_offset_atIndex(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "Foundation_NSRange", feature = "Metal_MTLBuffer"))]
        #[method(setBuffers:offsets:withRange:)]
        unsafe fn setBuffers_offsets_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "Metal_MTLFunctionHandle")]
        #[method(setFunction:atIndex:)]
        fn setFunction_atIndex(
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

        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_atIndex(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_withRange(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[method(setOpaqueCurveIntersectionFunctionWithSignature:atIndex:)]
        unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_atIndex(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setOpaqueCurveIntersectionFunctionWithSignature:withRange:)]
        unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_withRange(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[cfg(feature = "Metal_MTLVisibleFunctionTable")]
        #[method(setVisibleFunctionTable:atBufferIndex:)]
        unsafe fn setVisibleFunctionTable_atBufferIndex(
            &self,
            function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[cfg(all(
            feature = "Foundation_NSRange",
            feature = "Metal_MTLVisibleFunctionTable"
        ))]
        #[method(setVisibleFunctionTables:withBufferRange:)]
        unsafe fn setVisibleFunctionTables_withBufferRange(
            &self,
            function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_range: NSRange,
        );
    }

    #[cfg(feature = "Metal_MTLResource")]
    unsafe impl ProtocolType for dyn MTLIntersectionFunctionTable {}
);
