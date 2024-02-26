//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPointerArray;

    unsafe impl ClassType for NSPointerArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSPointerArray {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSPointerArray {}

#[cfg(feature = "Foundation_NSEnumerator")]
unsafe impl NSFastEnumeration for NSPointerArray {}

unsafe impl NSObjectProtocol for NSPointerArray {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSPointerArray {}

extern_methods!(
    unsafe impl NSPointerArray {
        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithOptions:)]
        pub unsafe fn initWithOptions(
            this: Allocated<Self>,
            options: NSPointerFunctionsOptions,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithPointerFunctions:)]
        pub unsafe fn initWithPointerFunctions(
            this: Allocated<Self>,
            functions: &NSPointerFunctions,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other pointerArrayWithOptions:)]
        pub unsafe fn pointerArrayWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSPointerArray>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other pointerArrayWithPointerFunctions:)]
        pub unsafe fn pointerArrayWithPointerFunctions(
            functions: &NSPointerFunctions,
        ) -> Id<NSPointerArray>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other pointerFunctions)]
        pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions>;

        #[method(pointerAtIndex:)]
        pub unsafe fn pointerAtIndex(&self, index: NSUInteger) -> *mut c_void;

        #[method(addPointer:)]
        pub unsafe fn addPointer(&self, pointer: *mut c_void);

        #[method(removePointerAtIndex:)]
        pub unsafe fn removePointerAtIndex(&self, index: NSUInteger);

        #[method(insertPointer:atIndex:)]
        pub unsafe fn insertPointer_atIndex(&self, item: *mut c_void, index: NSUInteger);

        #[method(replacePointerAtIndex:withPointer:)]
        pub unsafe fn replacePointerAtIndex_withPointer(
            &self,
            index: NSUInteger,
            item: *mut c_void,
        );

        #[method(compact)]
        pub unsafe fn compact(&self);

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method(setCount:)]
        pub unsafe fn setCount(&self, count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPointerArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSPointerArrayConveniences
    unsafe impl NSPointerArray {
        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other pointerArrayWithStrongObjects)]
        pub unsafe fn pointerArrayWithStrongObjects() -> Id<AnyObject>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other pointerArrayWithWeakObjects)]
        pub unsafe fn pointerArrayWithWeakObjects() -> Id<AnyObject>;

        #[method_id(@__retain_semantics Other strongObjectsPointerArray)]
        pub unsafe fn strongObjectsPointerArray() -> Id<NSPointerArray>;

        #[method_id(@__retain_semantics Other weakObjectsPointerArray)]
        pub unsafe fn weakObjectsPointerArray() -> Id<NSPointerArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray>;
    }
);
