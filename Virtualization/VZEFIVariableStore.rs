//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VZEFIVariableStoreInitializationOptions(pub NSUInteger);
impl VZEFIVariableStoreInitializationOptions {
    pub const VZEFIVariableStoreInitializationOptionAllowOverwrite: Self = Self(1 << 0);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for VZEFIVariableStoreInitializationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for VZEFIVariableStoreInitializationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZEFIVariableStore;

    unsafe impl ClassType for VZEFIVariableStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for VZEFIVariableStore {}

extern_methods!(
    unsafe impl VZEFIVariableStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initCreatingVariableStoreAtURL:options:error:_)]
        pub unsafe fn initCreatingVariableStoreAtURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            options: VZEFIVariableStoreInitializationOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL>;
    }
);
