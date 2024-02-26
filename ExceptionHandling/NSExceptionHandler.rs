//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ExceptionHandling::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSUncaughtSystemExceptionException: Option<&'static NSString>);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSUncaughtRuntimeErrorException: Option<&'static NSString>);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSStackTraceKey: Option<&'static NSString>);

extern_fn!(
    pub unsafe fn NSExceptionHandlerResume();
);

pub const NSLogUncaughtExceptionMask: c_uint = 1 << 0;
pub const NSHandleUncaughtExceptionMask: c_uint = 1 << 1;
pub const NSLogUncaughtSystemExceptionMask: c_uint = 1 << 2;
pub const NSHandleUncaughtSystemExceptionMask: c_uint = 1 << 3;
pub const NSLogUncaughtRuntimeErrorMask: c_uint = 1 << 4;
pub const NSHandleUncaughtRuntimeErrorMask: c_uint = 1 << 5;
pub const NSLogTopLevelExceptionMask: c_uint = 1 << 6;
pub const NSHandleTopLevelExceptionMask: c_uint = 1 << 7;
pub const NSLogOtherExceptionMask: c_uint = 1 << 8;
pub const NSHandleOtherExceptionMask: c_uint = 1 << 9;

pub const NSHangOnUncaughtExceptionMask: c_uint = 1 << 0;
pub const NSHangOnUncaughtSystemExceptionMask: c_uint = 1 << 1;
pub const NSHangOnUncaughtRuntimeErrorMask: c_uint = 1 << 2;
pub const NSHangOnTopLevelExceptionMask: c_uint = 1 << 3;
pub const NSHangOnOtherExceptionMask: c_uint = 1 << 4;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSExceptionHandler;

    unsafe impl ClassType for NSExceptionHandler {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSExceptionHandler {}

extern_methods!(
    unsafe impl NSExceptionHandler {
        #[method_id(@__retain_semantics Other defaultExceptionHandler)]
        pub unsafe fn defaultExceptionHandler() -> Option<Id<NSExceptionHandler>>;

        #[method(setExceptionHandlingMask:)]
        pub unsafe fn setExceptionHandlingMask(&self, a_mask: NSUInteger);

        #[method(exceptionHandlingMask)]
        pub unsafe fn exceptionHandlingMask(&self) -> NSUInteger;

        #[method(setExceptionHangingMask:)]
        pub unsafe fn setExceptionHangingMask(&self, a_mask: NSUInteger);

        #[method(exceptionHangingMask)]
        pub unsafe fn exceptionHangingMask(&self) -> NSUInteger;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, an_object: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSExceptionHandler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_category!(
    /// Category "NSExceptionHandlerDelegate" on [`NSObject`].
    #[doc(alias = "NSExceptionHandlerDelegate")]
    pub unsafe trait NSObjectNSExceptionHandlerDelegate {
        #[cfg(all(
            feature = "ExceptionHandling_NSExceptionHandler",
            feature = "Foundation_NSException"
        ))]
        #[method(exceptionHandler:shouldLogException:mask:)]
        unsafe fn exceptionHandler_shouldLogException_mask(
            &self,
            sender: Option<&NSExceptionHandler>,
            exception: Option<&NSException>,
            a_mask: NSUInteger,
        ) -> bool;

        #[cfg(all(
            feature = "ExceptionHandling_NSExceptionHandler",
            feature = "Foundation_NSException"
        ))]
        #[method(exceptionHandler:shouldHandleException:mask:)]
        unsafe fn exceptionHandler_shouldHandleException_mask(
            &self,
            sender: Option<&NSExceptionHandler>,
            exception: Option<&NSException>,
            a_mask: NSUInteger,
        ) -> bool;
    }

    unsafe impl NSObjectNSExceptionHandlerDelegate for NSObject {}
);
