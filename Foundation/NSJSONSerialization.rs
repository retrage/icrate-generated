//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSJSONReadingOptions {
        NSJSONReadingMutableContainers = 1 << 0,
        NSJSONReadingMutableLeaves = 1 << 1,
        NSJSONReadingFragmentsAllowed = 1 << 2,
        NSJSONReadingJSON5Allowed = 1 << 3,
        NSJSONReadingTopLevelDictionaryAssumed = 1 << 4,
        #[deprecated]
        NSJSONReadingAllowFragments = NSJSONReadingOptions::NSJSONReadingFragmentsAllowed.0,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSJSONWritingOptions {
        NSJSONWritingPrettyPrinted = 1 << 0,
        NSJSONWritingSortedKeys = 1 << 1,
        NSJSONWritingFragmentsAllowed = 1 << 2,
        NSJSONWritingWithoutEscapingSlashes = 1 << 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSJSONSerialization;

    unsafe impl ClassType for NSJSONSerialization {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSJSONSerialization {}

extern_methods!(
    unsafe impl NSJSONSerialization {
        #[method(isValidJSONObject:)]
        pub unsafe fn isValidJSONObject(obj: &AnyObject) -> bool;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other dataWithJSONObject:options:error:_)]
        pub unsafe fn dataWithJSONObject_options_error(
            obj: &AnyObject,
            opt: NSJSONWritingOptions,
        ) -> Result<Id<NSData>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other JSONObjectWithData:options:error:_)]
        pub unsafe fn JSONObjectWithData_options_error(
            data: &NSData,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSStream"))]
        #[method_id(@__retain_semantics Other JSONObjectWithStream:options:error:_)]
        pub unsafe fn JSONObjectWithStream_options_error(
            stream: &NSInputStream,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<AnyObject>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSJSONSerialization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
