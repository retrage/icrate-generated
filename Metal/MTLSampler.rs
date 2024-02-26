//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLSamplerMinMagFilter {
        #[doc(alias = "MTLSamplerMinMagFilterNearest")]
        Nearest = 0,
        #[doc(alias = "MTLSamplerMinMagFilterLinear")]
        Linear = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLSamplerMipFilter {
        #[doc(alias = "MTLSamplerMipFilterNotMipmapped")]
        NotMipmapped = 0,
        #[doc(alias = "MTLSamplerMipFilterNearest")]
        Nearest = 1,
        #[doc(alias = "MTLSamplerMipFilterLinear")]
        Linear = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLSamplerAddressMode {
        #[doc(alias = "MTLSamplerAddressModeClampToEdge")]
        ClampToEdge = 0,
        #[doc(alias = "MTLSamplerAddressModeMirrorClampToEdge")]
        MirrorClampToEdge = 1,
        #[doc(alias = "MTLSamplerAddressModeRepeat")]
        Repeat = 2,
        #[doc(alias = "MTLSamplerAddressModeMirrorRepeat")]
        MirrorRepeat = 3,
        #[doc(alias = "MTLSamplerAddressModeClampToZero")]
        ClampToZero = 4,
        #[doc(alias = "MTLSamplerAddressModeClampToBorderColor")]
        ClampToBorderColor = 5,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLSamplerBorderColor {
        #[doc(alias = "MTLSamplerBorderColorTransparentBlack")]
        TransparentBlack = 0,
        #[doc(alias = "MTLSamplerBorderColorOpaqueBlack")]
        OpaqueBlack = 1,
        #[doc(alias = "MTLSamplerBorderColorOpaqueWhite")]
        OpaqueWhite = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSamplerDescriptor;

    unsafe impl ClassType for MTLSamplerDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLSamplerDescriptor {}

unsafe impl NSObjectProtocol for MTLSamplerDescriptor {}

extern_methods!(
    unsafe impl MTLSamplerDescriptor {
        #[method(minFilter)]
        pub fn minFilter(&self) -> MTLSamplerMinMagFilter;

        #[method(setMinFilter:)]
        pub fn setMinFilter(&self, min_filter: MTLSamplerMinMagFilter);

        #[method(magFilter)]
        pub fn magFilter(&self) -> MTLSamplerMinMagFilter;

        #[method(setMagFilter:)]
        pub fn setMagFilter(&self, mag_filter: MTLSamplerMinMagFilter);

        #[method(mipFilter)]
        pub fn mipFilter(&self) -> MTLSamplerMipFilter;

        #[method(setMipFilter:)]
        pub fn setMipFilter(&self, mip_filter: MTLSamplerMipFilter);

        #[method(maxAnisotropy)]
        pub fn maxAnisotropy(&self) -> NSUInteger;

        #[method(setMaxAnisotropy:)]
        pub fn setMaxAnisotropy(&self, max_anisotropy: NSUInteger);

        #[method(sAddressMode)]
        pub fn sAddressMode(&self) -> MTLSamplerAddressMode;

        #[method(setSAddressMode:)]
        pub fn setSAddressMode(&self, s_address_mode: MTLSamplerAddressMode);

        #[method(tAddressMode)]
        pub fn tAddressMode(&self) -> MTLSamplerAddressMode;

        #[method(setTAddressMode:)]
        pub fn setTAddressMode(&self, t_address_mode: MTLSamplerAddressMode);

        #[method(rAddressMode)]
        pub fn rAddressMode(&self) -> MTLSamplerAddressMode;

        #[method(setRAddressMode:)]
        pub fn setRAddressMode(&self, r_address_mode: MTLSamplerAddressMode);

        #[method(borderColor)]
        pub fn borderColor(&self) -> MTLSamplerBorderColor;

        #[method(setBorderColor:)]
        pub fn setBorderColor(&self, border_color: MTLSamplerBorderColor);

        #[method(normalizedCoordinates)]
        pub fn normalizedCoordinates(&self) -> bool;

        #[method(setNormalizedCoordinates:)]
        pub fn setNormalizedCoordinates(&self, normalized_coordinates: bool);

        #[method(lodMinClamp)]
        pub fn lodMinClamp(&self) -> c_float;

        #[method(setLodMinClamp:)]
        pub fn setLodMinClamp(&self, lod_min_clamp: c_float);

        #[method(lodMaxClamp)]
        pub fn lodMaxClamp(&self) -> c_float;

        #[method(setLodMaxClamp:)]
        pub fn setLodMaxClamp(&self, lod_max_clamp: c_float);

        #[method(lodAverage)]
        pub fn lodAverage(&self) -> bool;

        #[method(setLodAverage:)]
        pub fn setLodAverage(&self, lod_average: bool);

        #[cfg(feature = "Metal_MTLDepthStencil")]
        #[method(compareFunction)]
        pub fn compareFunction(&self) -> MTLCompareFunction;

        #[cfg(feature = "Metal_MTLDepthStencil")]
        #[method(setCompareFunction:)]
        pub fn setCompareFunction(&self, compare_function: MTLCompareFunction);

        #[method(supportArgumentBuffers)]
        pub fn supportArgumentBuffers(&self) -> bool;

        #[method(setSupportArgumentBuffers:)]
        pub fn setSupportArgumentBuffers(&self, support_argument_buffers: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLSamplerDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for MTLSamplerDescriptor {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_protocol!(
    pub unsafe trait MTLSamplerState: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;
    }

    unsafe impl ProtocolType for dyn MTLSamplerState {}
);
