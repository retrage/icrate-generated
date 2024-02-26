//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSImageRep")]
    pub struct NSPDFImageRep;

    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl ClassType for NSPDFImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for NSPDFImageRep {}

#[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for NSPDFImageRep {}

#[cfg(feature = "AppKit_NSImageRep")]
unsafe impl NSObjectProtocol for NSPDFImageRep {}

extern_methods!(
    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl NSPDFImageRep {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(pdf_data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, pdf_data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other PDFRepresentation)]
        pub unsafe fn PDFRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;

        #[method(currentPage)]
        pub unsafe fn currentPage(&self) -> NSInteger;

        #[method(setCurrentPage:)]
        pub unsafe fn setCurrentPage(&self, current_page: NSInteger);

        #[method(pageCount)]
        pub unsafe fn pageCount(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSImageRep`
    #[cfg(feature = "AppKit_NSImageRep")]
    unsafe impl NSPDFImageRep {
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
    unsafe impl NSPDFImageRep {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
