//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKWindowFeatures;

    unsafe impl ClassType for WKWindowFeatures {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for WKWindowFeatures {}

extern_methods!(
    unsafe impl WKWindowFeatures {
        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other menuBarVisibility)]
        pub unsafe fn menuBarVisibility(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other statusBarVisibility)]
        pub unsafe fn statusBarVisibility(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other toolbarsVisibility)]
        pub unsafe fn toolbarsVisibility(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other allowsResizing)]
        pub unsafe fn allowsResizing(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other x)]
        pub unsafe fn x(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other y)]
        pub unsafe fn y(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other height)]
        pub unsafe fn height(&self) -> Option<Id<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKWindowFeatures {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
