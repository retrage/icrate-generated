//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileHandle;

    unsafe impl ClassType for NSFileHandle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSFileHandle {}

unsafe impl Sync for NSFileHandle {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSFileHandle {}

unsafe impl NSObjectProtocol for NSFileHandle {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSFileHandle {}

extern_methods!(
    unsafe impl NSFileHandle {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other availableData)]
        pub unsafe fn availableData(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics Init initWithFileDescriptor:closeOnDealloc:)]
        pub unsafe fn initWithFileDescriptor_closeOnDealloc(
            this: Allocated<Self>,
            fd: c_int,
            closeopt: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other readDataToEndOfFileAndReturnError:_)]
        pub unsafe fn readDataToEndOfFileAndReturnError(&self) -> Result<Id<NSData>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other readDataUpToLength:error:_)]
        pub unsafe fn readDataUpToLength_error(
            &self,
            length: NSUInteger,
        ) -> Result<Id<NSData>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(writeData:error:_)]
        pub unsafe fn writeData_error(&self, data: &NSData) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(getOffset:error:_)]
        pub unsafe fn getOffset_error(
            &self,
            offset_in_file: NonNull<c_ulonglong>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(seekToEndReturningOffset:error:_)]
        pub unsafe fn seekToEndReturningOffset_error(
            &self,
            offset_in_file: *mut c_ulonglong,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(seekToOffset:error:_)]
        pub unsafe fn seekToOffset_error(&self, offset: c_ulonglong) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(truncateAtOffset:error:_)]
        pub unsafe fn truncateAtOffset_error(&self, offset: c_ulonglong)
            -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(synchronizeAndReturnError:_)]
        pub unsafe fn synchronizeAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(closeAndReturnError:_)]
        pub unsafe fn closeAndReturnError(&self) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSFileHandleCreation
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Other fileHandleWithStandardInput)]
        pub unsafe fn fileHandleWithStandardInput() -> Id<NSFileHandle>;

        #[method_id(@__retain_semantics Other fileHandleWithStandardOutput)]
        pub unsafe fn fileHandleWithStandardOutput() -> Id<NSFileHandle>;

        #[method_id(@__retain_semantics Other fileHandleWithStandardError)]
        pub unsafe fn fileHandleWithStandardError() -> Id<NSFileHandle>;

        #[method_id(@__retain_semantics Other fileHandleWithNullDevice)]
        pub unsafe fn fileHandleWithNullDevice() -> Id<NSFileHandle>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileHandleForReadingAtPath:)]
        pub unsafe fn fileHandleForReadingAtPath(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileHandleForWritingAtPath:)]
        pub unsafe fn fileHandleForWritingAtPath(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileHandleForUpdatingAtPath:)]
        pub unsafe fn fileHandleForUpdatingAtPath(path: &NSString) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other fileHandleForReadingFromURL:error:_)]
        pub unsafe fn fileHandleForReadingFromURL_error(
            url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other fileHandleForWritingToURL:error:_)]
        pub unsafe fn fileHandleForWritingToURL_error(url: &NSURL)
            -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other fileHandleForUpdatingURL:error:_)]
        pub unsafe fn fileHandleForUpdatingURL_error(url: &NSURL) -> Result<Id<Self>, Id<NSError>>;
    }
);

#[cfg(all(feature = "Foundation_NSObjCRuntime", feature = "Foundation_NSString"))]
extern_static!(NSFileHandleOperationException: &'static NSExceptionName);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSFileHandleReadCompletionNotification: &'static NSNotificationName);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSFileHandleReadToEndOfFileCompletionNotification: &'static NSNotificationName);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSFileHandleConnectionAcceptedNotification: &'static NSNotificationName);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSFileHandleDataAvailableNotification: &'static NSNotificationName);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSFileHandleNotificationDataItem: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSFileHandleNotificationFileHandleItem: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSFileHandleNotificationMonitorModes: &'static NSString);

extern_methods!(
    /// NSFileHandleAsynchronousAccess
    unsafe impl NSFileHandle {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSString"
        ))]
        #[method(readInBackgroundAndNotifyForModes:)]
        pub unsafe fn readInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(readInBackgroundAndNotify)]
        pub unsafe fn readInBackgroundAndNotify(&self);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSString"
        ))]
        #[method(readToEndOfFileInBackgroundAndNotifyForModes:)]
        pub unsafe fn readToEndOfFileInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(readToEndOfFileInBackgroundAndNotify)]
        pub unsafe fn readToEndOfFileInBackgroundAndNotify(&self);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSString"
        ))]
        #[method(acceptConnectionInBackgroundAndNotifyForModes:)]
        pub unsafe fn acceptConnectionInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(acceptConnectionInBackgroundAndNotify)]
        pub unsafe fn acceptConnectionInBackgroundAndNotify(&self);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSString"
        ))]
        #[method(waitForDataInBackgroundAndNotifyForModes:)]
        pub unsafe fn waitForDataInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(waitForDataInBackgroundAndNotify)]
        pub unsafe fn waitForDataInBackgroundAndNotify(&self);

        #[method(readabilityHandler)]
        pub unsafe fn readabilityHandler(&self) -> *mut Block<dyn Fn(NonNull<NSFileHandle>)>;

        #[method(setReadabilityHandler:)]
        pub unsafe fn setReadabilityHandler(
            &self,
            readability_handler: Option<&Block<dyn Fn(NonNull<NSFileHandle>)>>,
        );

        #[method(writeabilityHandler)]
        pub unsafe fn writeabilityHandler(&self) -> *mut Block<dyn Fn(NonNull<NSFileHandle>)>;

        #[method(setWriteabilityHandler:)]
        pub unsafe fn setWriteabilityHandler(
            &self,
            writeability_handler: Option<&Block<dyn Fn(NonNull<NSFileHandle>)>>,
        );
    }
);

extern_methods!(
    /// NSFileHandlePlatformSpecific
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Init initWithFileDescriptor:)]
        pub unsafe fn initWithFileDescriptor(this: Allocated<Self>, fd: c_int) -> Id<Self>;

        #[method(fileDescriptor)]
        pub unsafe fn fileDescriptor(&self) -> c_int;
    }
);

extern_methods!(
    unsafe impl NSFileHandle {
        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other readDataToEndOfFile)]
        pub unsafe fn readDataToEndOfFile(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other readDataOfLength:)]
        pub unsafe fn readDataOfLength(&self, length: NSUInteger) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method(writeData:)]
        pub unsafe fn writeData(&self, data: &NSData);

        #[deprecated]
        #[method(offsetInFile)]
        pub unsafe fn offsetInFile(&self) -> c_ulonglong;

        #[deprecated]
        #[method(seekToEndOfFile)]
        pub unsafe fn seekToEndOfFile(&self) -> c_ulonglong;

        #[deprecated]
        #[method(seekToFileOffset:)]
        pub unsafe fn seekToFileOffset(&self, offset: c_ulonglong);

        #[deprecated]
        #[method(truncateFileAtOffset:)]
        pub unsafe fn truncateFileAtOffset(&self, offset: c_ulonglong);

        #[deprecated]
        #[method(synchronizeFile)]
        pub unsafe fn synchronizeFile(&self);

        #[deprecated]
        #[method(closeFile)]
        pub unsafe fn closeFile(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPipe;

    unsafe impl ClassType for NSPipe {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSPipe {}

unsafe impl Sync for NSPipe {}

unsafe impl NSObjectProtocol for NSPipe {}

extern_methods!(
    unsafe impl NSPipe {
        #[method_id(@__retain_semantics Other fileHandleForReading)]
        pub unsafe fn fileHandleForReading(&self) -> Id<NSFileHandle>;

        #[method_id(@__retain_semantics Other fileHandleForWriting)]
        pub unsafe fn fileHandleForWriting(&self) -> Id<NSFileHandle>;

        #[method_id(@__retain_semantics Other pipe)]
        pub unsafe fn pipe() -> Id<NSPipe>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPipe {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
