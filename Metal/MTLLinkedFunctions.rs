//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLLinkedFunctions;

    unsafe impl ClassType for MTLLinkedFunctions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLLinkedFunctions {}

unsafe impl NSObjectProtocol for MTLLinkedFunctions {}

extern_methods!(
    unsafe impl MTLLinkedFunctions {
        #[method_id(@__retain_semantics Other linkedFunctions)]
        pub fn linkedFunctions() -> Id<MTLLinkedFunctions>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLLibrary"))]
        #[method_id(@__retain_semantics Other functions)]
        pub fn functions(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLLibrary"))]
        #[method(setFunctions:)]
        pub fn setFunctions(&self, functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLLibrary"))]
        #[method_id(@__retain_semantics Other binaryFunctions)]
        pub fn binaryFunctions(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLLibrary"))]
        #[method(setBinaryFunctions:)]
        pub fn setBinaryFunctions(
            &self,
            binary_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Metal_MTLLibrary"
        ))]
        #[method_id(@__retain_semantics Other groups)]
        pub fn groups(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Metal_MTLLibrary"
        ))]
        #[method(setGroups:)]
        pub fn setGroups(
            &self,
            groups: Option<&NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLLibrary"))]
        #[method_id(@__retain_semantics Other privateFunctions)]
        pub fn privateFunctions(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLLibrary"))]
        #[method(setPrivateFunctions:)]
        pub fn setPrivateFunctions(
            &self,
            private_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLLinkedFunctions {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for MTLLinkedFunctions {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}
