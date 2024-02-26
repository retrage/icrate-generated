//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSImageRep")]
    pub struct NSPICTImageRep;

    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl ClassType for NSPICTImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for NSPICTImageRep {}

#[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for NSPICTImageRep {}

#[cfg(feature = "AppKit_NSImageRep")]
unsafe impl NSObjectProtocol for NSPICTImageRep {}

extern_methods!(
    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl NSPICTImageRep {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(pict_data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, pict_data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other PICTRepresentation)]
        pub unsafe fn PICTRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSImageRep`
    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl NSPICTImageRep {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl NSPICTImageRep {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
