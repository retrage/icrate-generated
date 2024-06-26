//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static WebPlugInBaseURLKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebPlugInAttributesKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebPlugInContainerKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebPlugInContainingElementKey: Option<&'static NSString>;
}

extern "C" {
    pub static WebPlugInShouldLoadMainResourceKey: Option<&'static NSString>;
}

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebPlugInViewFactory: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[deprecated]
        #[method_id(@__retain_semantics Other plugInViewWithArguments:)]
        unsafe fn plugInViewWithArguments(
            arguments: Option<&NSDictionary>,
            mtm: MainThreadMarker,
        ) -> Option<Id<NSView>>;
    }

    unsafe impl ProtocolType for dyn WebPlugInViewFactory {}
);
