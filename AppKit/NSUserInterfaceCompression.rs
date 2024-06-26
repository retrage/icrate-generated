//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserInterfaceCompressionOptions;

    unsafe impl ClassType for NSUserInterfaceCompressionOptions {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

unsafe impl NSCoding for NSUserInterfaceCompressionOptions {}

unsafe impl NSCopying for NSUserInterfaceCompressionOptions {}

unsafe impl NSObjectProtocol for NSUserInterfaceCompressionOptions {}

extern_methods!(
    unsafe impl NSUserInterfaceCompressionOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(this: Allocated<Self>, identifier: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCompressionOptions:)]
        pub unsafe fn initWithCompressionOptions(
            this: Allocated<Self>,
            options: &NSSet<NSUserInterfaceCompressionOptions>,
        ) -> Id<Self>;

        #[method(containsOptions:)]
        pub unsafe fn containsOptions(&self, options: &NSUserInterfaceCompressionOptions) -> bool;

        #[method(intersectsOptions:)]
        pub unsafe fn intersectsOptions(&self, options: &NSUserInterfaceCompressionOptions)
            -> bool;

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method_id(@__retain_semantics Other optionsByAddingOptions:)]
        pub unsafe fn optionsByAddingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Id<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other optionsByRemovingOptions:)]
        pub unsafe fn optionsByRemovingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Id<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other hideImagesOption)]
        pub unsafe fn hideImagesOption() -> Id<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other hideTextOption)]
        pub unsafe fn hideTextOption() -> Id<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other reduceMetricsOption)]
        pub unsafe fn reduceMetricsOption() -> Id<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other breakEqualWidthsOption)]
        pub unsafe fn breakEqualWidthsOption() -> Id<NSUserInterfaceCompressionOptions>;

        #[method_id(@__retain_semantics Other standardOptions)]
        pub unsafe fn standardOptions() -> Id<NSUserInterfaceCompressionOptions>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserInterfaceCompressionOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSUserInterfaceCompression {
        #[method(compressWithPrioritizedCompressionOptions:)]
        unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );

        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;

        #[method_id(@__retain_semantics Other activeCompressionOptions)]
        unsafe fn activeCompressionOptions(&self) -> Id<NSUserInterfaceCompressionOptions>;
    }

    unsafe impl ProtocolType for dyn NSUserInterfaceCompression {}
);
