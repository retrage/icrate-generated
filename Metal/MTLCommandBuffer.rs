//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCommandBufferStatus {
        #[doc(alias = "MTLCommandBufferStatusNotEnqueued")]
        NotEnqueued = 0,
        #[doc(alias = "MTLCommandBufferStatusEnqueued")]
        Enqueued = 1,
        #[doc(alias = "MTLCommandBufferStatusCommitted")]
        Committed = 2,
        #[doc(alias = "MTLCommandBufferStatusScheduled")]
        Scheduled = 3,
        #[doc(alias = "MTLCommandBufferStatusCompleted")]
        Completed = 4,
        #[doc(alias = "MTLCommandBufferStatusError")]
        Error = 5,
    }
);

#[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
extern_static!(MTLCommandBufferErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCommandBufferError {
        #[doc(alias = "MTLCommandBufferErrorNone")]
        None = 0,
        #[doc(alias = "MTLCommandBufferErrorInternal")]
        Internal = 1,
        #[doc(alias = "MTLCommandBufferErrorTimeout")]
        Timeout = 2,
        #[doc(alias = "MTLCommandBufferErrorPageFault")]
        PageFault = 3,
        #[deprecated]
        #[doc(alias = "MTLCommandBufferErrorBlacklisted")]
        Blacklisted = 4,
        #[doc(alias = "MTLCommandBufferErrorAccessRevoked")]
        AccessRevoked = 4,
        #[doc(alias = "MTLCommandBufferErrorNotPermitted")]
        NotPermitted = 7,
        #[doc(alias = "MTLCommandBufferErrorOutOfMemory")]
        OutOfMemory = 8,
        #[doc(alias = "MTLCommandBufferErrorInvalidResource")]
        InvalidResource = 9,
        #[doc(alias = "MTLCommandBufferErrorMemoryless")]
        Memoryless = 10,
        #[doc(alias = "MTLCommandBufferErrorDeviceRemoved")]
        DeviceRemoved = 11,
        #[doc(alias = "MTLCommandBufferErrorStackOverflow")]
        StackOverflow = 12,
    }
);

#[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
extern_static!(MTLCommandBufferEncoderInfoErrorKey: &'static NSErrorUserInfoKey);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLCommandBufferErrorOption {
        #[doc(alias = "MTLCommandBufferErrorOptionNone")]
        None = 0,
        #[doc(alias = "MTLCommandBufferErrorOptionEncoderExecutionStatus")]
        EncoderExecutionStatus = 1 << 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCommandEncoderErrorState {
        #[doc(alias = "MTLCommandEncoderErrorStateUnknown")]
        Unknown = 0,
        #[doc(alias = "MTLCommandEncoderErrorStateCompleted")]
        Completed = 1,
        #[doc(alias = "MTLCommandEncoderErrorStateAffected")]
        Affected = 2,
        #[doc(alias = "MTLCommandEncoderErrorStatePending")]
        Pending = 3,
        #[doc(alias = "MTLCommandEncoderErrorStateFaulted")]
        Faulted = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCommandBufferDescriptor;

    unsafe impl ClassType for MTLCommandBufferDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLCommandBufferDescriptor {}

unsafe impl NSObjectProtocol for MTLCommandBufferDescriptor {}

extern_methods!(
    unsafe impl MTLCommandBufferDescriptor {
        #[method(retainedReferences)]
        pub unsafe fn retainedReferences(&self) -> bool;

        #[method(setRetainedReferences:)]
        pub unsafe fn setRetainedReferences(&self, retained_references: bool);

        #[method(errorOptions)]
        pub unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption;

        #[method(setErrorOptions:)]
        pub unsafe fn setErrorOptions(&self, error_options: MTLCommandBufferErrorOption);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLCommandBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLCommandBufferEncoderInfo: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other debugSignposts)]
        unsafe fn debugSignposts(&self) -> Id<NSArray<NSString>>;

        #[method(errorState)]
        unsafe fn errorState(&self) -> MTLCommandEncoderErrorState;
    }

    unsafe impl ProtocolType for dyn MTLCommandBufferEncoderInfo {}
);

pub type MTLCommandBufferHandler =
    *mut Block<dyn Fn(NonNull<ProtocolObject<dyn MTLCommandBuffer>>)>;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLDispatchType {
        #[doc(alias = "MTLDispatchTypeSerial")]
        Serial = 0,
        #[doc(alias = "MTLDispatchTypeConcurrent")]
        Concurrent = 1,
    }
);

extern_protocol!(
    pub unsafe trait MTLCommandBuffer: NSObjectProtocol {
        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Metal_MTLCommandQueue")]
        #[method_id(@__retain_semantics Other commandQueue)]
        unsafe fn commandQueue(&self) -> Id<ProtocolObject<dyn MTLCommandQueue>>;

        #[method(retainedReferences)]
        unsafe fn retainedReferences(&self) -> bool;

        #[method(errorOptions)]
        unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method(kernelStartTime)]
        unsafe fn kernelStartTime(&self) -> CFTimeInterval;

        #[method(kernelEndTime)]
        unsafe fn kernelEndTime(&self) -> CFTimeInterval;

        #[cfg(all(feature = "Foundation_NSEnumerator", feature = "Metal_MTLFunctionLog"))]
        #[method_id(@__retain_semantics Other logs)]
        unsafe fn logs(&self) -> Id<ProtocolObject<dyn MTLLogContainer>>;

        #[method(GPUStartTime)]
        unsafe fn GPUStartTime(&self) -> CFTimeInterval;

        #[method(GPUEndTime)]
        unsafe fn GPUEndTime(&self) -> CFTimeInterval;

        #[method(enqueue)]
        fn enqueue(&self);

        #[method(commit)]
        fn commit(&self);

        #[method(addScheduledHandler:)]
        unsafe fn addScheduledHandler(&self, block: MTLCommandBufferHandler);

        #[cfg(feature = "Metal_MTLDrawable")]
        #[method(presentDrawable:)]
        fn presentDrawable(&self, drawable: &ProtocolObject<dyn MTLDrawable>);

        #[cfg(feature = "Metal_MTLDrawable")]
        #[method(presentDrawable:atTime:)]
        unsafe fn presentDrawable_atTime(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
            presentation_time: CFTimeInterval,
        );

        #[cfg(feature = "Metal_MTLDrawable")]
        #[method(presentDrawable:afterMinimumDuration:)]
        unsafe fn presentDrawable_afterMinimumDuration(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
            duration: CFTimeInterval,
        );

        #[method(waitUntilScheduled)]
        fn waitUntilScheduled(&self);

        #[method(addCompletedHandler:)]
        unsafe fn addCompletedHandler(&self, block: MTLCommandBufferHandler);

        #[method(waitUntilCompleted)]
        unsafe fn waitUntilCompleted(&self);

        #[method(status)]
        fn status(&self) -> MTLCommandBufferStatus;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        unsafe fn error(&self) -> Option<Id<NSError>>;

        #[cfg(all(
            feature = "Metal_MTLBlitCommandEncoder",
            feature = "Metal_MTLCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other blitCommandEncoder)]
        fn blitCommandEncoder(&self) -> Option<Id<ProtocolObject<dyn MTLBlitCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLCommandEncoder",
            feature = "Metal_MTLRenderCommandEncoder",
            feature = "Metal_MTLRenderPass"
        ))]
        #[method_id(@__retain_semantics Other renderCommandEncoderWithDescriptor:)]
        fn renderCommandEncoderWithDescriptor(
            &self,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLRenderCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLCommandEncoder",
            feature = "Metal_MTLComputeCommandEncoder",
            feature = "Metal_MTLComputePass"
        ))]
        #[method_id(@__retain_semantics Other computeCommandEncoderWithDescriptor:)]
        unsafe fn computeCommandEncoderWithDescriptor(
            &self,
            compute_pass_descriptor: &MTLComputePassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLBlitCommandEncoder",
            feature = "Metal_MTLBlitPass",
            feature = "Metal_MTLCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other blitCommandEncoderWithDescriptor:)]
        unsafe fn blitCommandEncoderWithDescriptor(
            &self,
            blit_pass_descriptor: &MTLBlitPassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLBlitCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLCommandEncoder",
            feature = "Metal_MTLComputeCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other computeCommandEncoder)]
        fn computeCommandEncoder(&self)
            -> Option<Id<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLCommandEncoder",
            feature = "Metal_MTLComputeCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other computeCommandEncoderWithDispatchType:)]
        fn computeCommandEncoderWithDispatchType(
            &self,
            dispatch_type: MTLDispatchType,
        ) -> Option<Id<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[cfg(feature = "Metal_MTLEvent")]
        #[method(encodeWaitForEvent:value:)]
        fn encodeWaitForEvent_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        #[cfg(feature = "Metal_MTLEvent")]
        #[method(encodeSignalEvent:value:)]
        fn encodeSignalEvent_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        #[cfg(all(
            feature = "Metal_MTLCommandEncoder",
            feature = "Metal_MTLParallelRenderCommandEncoder",
            feature = "Metal_MTLRenderPass"
        ))]
        #[method_id(@__retain_semantics Other parallelRenderCommandEncoderWithDescriptor:)]
        fn parallelRenderCommandEncoderWithDescriptor(
            &self,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLParallelRenderCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLCommandEncoder",
            feature = "Metal_MTLResourceStateCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other resourceStateCommandEncoder)]
        unsafe fn resourceStateCommandEncoder(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLResourceStateCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLCommandEncoder",
            feature = "Metal_MTLResourceStateCommandEncoder",
            feature = "Metal_MTLResourceStatePass"
        ))]
        #[method_id(@__retain_semantics Other resourceStateCommandEncoderWithDescriptor:)]
        unsafe fn resourceStateCommandEncoderWithDescriptor(
            &self,
            resource_state_pass_descriptor: &MTLResourceStatePassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLResourceStateCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLAccelerationStructureCommandEncoder",
            feature = "Metal_MTLCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other accelerationStructureCommandEncoder)]
        fn accelerationStructureCommandEncoder(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>>>;

        #[cfg(all(
            feature = "Metal_MTLAccelerationStructureCommandEncoder",
            feature = "Metal_MTLCommandEncoder"
        ))]
        #[method_id(@__retain_semantics Other accelerationStructureCommandEncoderWithDescriptor:)]
        unsafe fn accelerationStructureCommandEncoderWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructurePassDescriptor,
        ) -> Id<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(pushDebugGroup:)]
        fn pushDebugGroup(&self, string: &NSString);

        #[method(popDebugGroup)]
        fn popDebugGroup(&self);
    }

    unsafe impl ProtocolType for dyn MTLCommandBuffer {}
);
