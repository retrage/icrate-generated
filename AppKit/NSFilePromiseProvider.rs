//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFilePromiseProvider;

    unsafe impl ClassType for NSFilePromiseProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSFilePromiseProvider {}

#[cfg(feature = "NSPasteboard")]
unsafe impl NSPasteboardWriting for NSFilePromiseProvider {}

extern_methods!(
    unsafe impl NSFilePromiseProvider {
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Id<NSString>;

        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: &NSString);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSFilePromiseProviderDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSFilePromiseProviderDelegate>>,
        );

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<AnyObject>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&AnyObject>);

        #[method_id(@__retain_semantics Init initWithFileType:delegate:)]
        pub unsafe fn initWithFileType_delegate(
            this: Allocated<Self>,
            file_type: &NSString,
            delegate: &ProtocolObject<dyn NSFilePromiseProviderDelegate>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFilePromiseProvider {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSFilePromiseProviderDelegate: NSObjectProtocol {
        #[method_id(@__retain_semantics Other filePromiseProvider:fileNameForType:)]
        unsafe fn filePromiseProvider_fileNameForType(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            file_type: &NSString,
            mtm: MainThreadMarker,
        ) -> Id<NSString>;

        #[cfg(feature = "block2")]
        #[method(filePromiseProvider:writePromiseToURL:completionHandler:)]
        unsafe fn filePromiseProvider_writePromiseToURL_completionHandler(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[optional]
        #[method_id(@__retain_semantics Other operationQueueForFilePromiseProvider:)]
        unsafe fn operationQueueForFilePromiseProvider(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            mtm: MainThreadMarker,
        ) -> Id<NSOperationQueue>;
    }

    unsafe impl ProtocolType for dyn NSFilePromiseProviderDelegate {}
);
