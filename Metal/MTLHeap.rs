//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLHeapType(pub NSInteger);
impl MTLHeapType {
    #[doc(alias = "MTLHeapTypeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "MTLHeapTypePlacement")]
    pub const Placement: Self = Self(1);
    #[doc(alias = "MTLHeapTypeSparse")]
    pub const Sparse: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for MTLHeapType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for MTLHeapType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLHeapDescriptor;

    unsafe impl ClassType for MTLHeapDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLHeapDescriptor {}

unsafe impl NSObjectProtocol for MTLHeapDescriptor {}

extern_methods!(
    unsafe impl MTLHeapDescriptor {
        #[method(size)]
        pub fn size(&self) -> NSUInteger;

        #[method(setSize:)]
        pub fn setSize(&self, size: NSUInteger);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setStorageMode:)]
        pub fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setCpuCacheMode:)]
        pub fn setCpuCacheMode(&self, cpu_cache_mode: MTLCPUCacheMode);

        #[cfg(feature = "Metal_MTLDevice")]
        #[method(sparsePageSize)]
        pub unsafe fn sparsePageSize(&self) -> MTLSparsePageSize;

        #[cfg(feature = "Metal_MTLDevice")]
        #[method(setSparsePageSize:)]
        pub unsafe fn setSparsePageSize(&self, sparse_page_size: MTLSparsePageSize);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setHazardTrackingMode:)]
        pub fn setHazardTrackingMode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setResourceOptions:)]
        pub fn setResourceOptions(&self, resource_options: MTLResourceOptions);

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLHeapType;

        #[method(setType:)]
        pub fn setType(&self, r#type: MTLHeapType);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLHeapDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLHeap: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(storageMode)]
        fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(cpuCacheMode)]
        fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(hazardTrackingMode)]
        fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(resourceOptions)]
        fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(size)]
        fn size(&self) -> NSUInteger;

        #[method(usedSize)]
        fn usedSize(&self) -> NSUInteger;

        #[method(currentAllocatedSize)]
        fn currentAllocatedSize(&self) -> NSUInteger;

        #[method(maxAvailableSizeWithAlignment:)]
        fn maxAvailableSizeWithAlignment(&self, alignment: NSUInteger) -> NSUInteger;

        #[cfg(all(feature = "Metal_MTLBuffer", feature = "Metal_MTLResource"))]
        #[method_id(@__retain_semantics New newBufferWithLength:options:)]
        fn newBufferWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(all(feature = "Metal_MTLResource", feature = "Metal_MTLTexture"))]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:)]
        fn newTextureWithDescriptor(
            &self,
            descriptor: &MTLTextureDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setPurgeableState:)]
        fn setPurgeableState(&self, state: MTLPurgeableState) -> MTLPurgeableState;

        #[method(type)]
        unsafe fn r#type(&self) -> MTLHeapType;

        #[cfg(all(feature = "Metal_MTLBuffer", feature = "Metal_MTLResource"))]
        #[method_id(@__retain_semantics New newBufferWithLength:options:offset:)]
        unsafe fn newBufferWithLength_options_offset(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(all(feature = "Metal_MTLResource", feature = "Metal_MTLTexture"))]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:offset:)]
        unsafe fn newTextureWithDescriptor_offset(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(all(
            feature = "Metal_MTLAccelerationStructure",
            feature = "Metal_MTLResource"
        ))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:)]
        unsafe fn newAccelerationStructureWithSize(
            &self,
            size: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(all(
            feature = "Metal_MTLAccelerationStructure",
            feature = "Metal_MTLResource"
        ))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:)]
        unsafe fn newAccelerationStructureWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(all(
            feature = "Metal_MTLAccelerationStructure",
            feature = "Metal_MTLResource"
        ))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:offset:)]
        unsafe fn newAccelerationStructureWithSize_offset(
            &self,
            size: NSUInteger,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(all(
            feature = "Metal_MTLAccelerationStructure",
            feature = "Metal_MTLResource"
        ))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:offset:)]
        unsafe fn newAccelerationStructureWithDescriptor_offset(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;
    }

    unsafe impl ProtocolType for dyn MTLHeap {}
);
