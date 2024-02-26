//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKAttachment;

    unsafe impl ClassType for HKAttachment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for HKAttachment {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for HKAttachment {}

unsafe impl NSObjectProtocol for HKAttachment {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for HKAttachment {}

extern_methods!(
    unsafe impl HKAttachment {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "UniformTypeIdentifiers_UTType")]
        #[method_id(@__retain_semantics Other contentType)]
        pub unsafe fn contentType(&self) -> Id<UTType>;

        #[method(size)]
        pub unsafe fn size(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Id<NSDate>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
