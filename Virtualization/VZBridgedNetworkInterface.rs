//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Virtualization::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZBridgedNetworkInterface;

    unsafe impl ClassType for VZBridgedNetworkInterface {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for VZBridgedNetworkInterface {}

extern_methods!(
    unsafe impl VZBridgedNetworkInterface {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other networkInterfaces)]
        pub unsafe fn networkInterfaces() -> Id<NSArray<VZBridgedNetworkInterface>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedDisplayName)]
        pub unsafe fn localizedDisplayName(&self) -> Option<Id<NSString>>;
    }
);
