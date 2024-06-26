//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MKAnnotation: NSObjectProtocol {
        #[cfg(feature = "objc2-core-location")]
        #[method(coordinate)]
        unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[optional]
        #[method_id(@__retain_semantics Other title)]
        unsafe fn title(&self) -> Option<Id<NSString>>;

        #[optional]
        #[method_id(@__retain_semantics Other subtitle)]
        unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "objc2-core-location")]
        #[optional]
        #[method(setCoordinate:)]
        unsafe fn setCoordinate(&self, new_coordinate: CLLocationCoordinate2D);
    }

    unsafe impl ProtocolType for dyn MKAnnotation {}
);
