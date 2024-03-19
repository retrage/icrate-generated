//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAppleEventSendOptions(pub NSUInteger);
impl NSAppleEventSendOptions {
    pub const NSAppleEventSendNoReply: Self = Self(1);
    pub const NSAppleEventSendQueueReply: Self = Self(2);
    pub const NSAppleEventSendWaitForReply: Self = Self(3);
    pub const NSAppleEventSendNeverInteract: Self = Self(16);
    pub const NSAppleEventSendCanInteract: Self = Self(32);
    pub const NSAppleEventSendAlwaysInteract: Self = Self(48);
    pub const NSAppleEventSendCanSwitchLayer: Self = Self(64);
    pub const NSAppleEventSendDontRecord: Self = Self(4096);
    pub const NSAppleEventSendDontExecute: Self = Self(8192);
    pub const NSAppleEventSendDontAnnotate: Self = Self(65536);
    pub const NSAppleEventSendDefaultOptions: Self = Self(35);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSAppleEventSendOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSAppleEventSendOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppleEventDescriptor;

    unsafe impl ClassType for NSAppleEventDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSAppleEventDescriptor {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSAppleEventDescriptor {}

unsafe impl NSObjectProtocol for NSAppleEventDescriptor {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSAppleEventDescriptor {}

extern_methods!(
    unsafe impl NSAppleEventDescriptor {
        #[method_id(@__retain_semantics Other nullDescriptor)]
        pub unsafe fn nullDescriptor() -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other descriptorWithBoolean:)]
        pub unsafe fn descriptorWithBoolean(boolean: Boolean) -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other descriptorWithEnumCode:)]
        pub unsafe fn descriptorWithEnumCode(enumerator: OSType) -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other descriptorWithInt32:)]
        pub unsafe fn descriptorWithInt32(signed_int: i32) -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other descriptorWithDouble:)]
        pub unsafe fn descriptorWithDouble(double_value: c_double) -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other descriptorWithTypeCode:)]
        pub unsafe fn descriptorWithTypeCode(type_code: OSType) -> Id<NSAppleEventDescriptor>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptorWithString:)]
        pub unsafe fn descriptorWithString(string: &NSString) -> Id<NSAppleEventDescriptor>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other descriptorWithDate:)]
        pub unsafe fn descriptorWithDate(date: &NSDate) -> Id<NSAppleEventDescriptor>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other descriptorWithFileURL:)]
        pub unsafe fn descriptorWithFileURL(file_url: &NSURL) -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other listDescriptor)]
        pub unsafe fn listDescriptor() -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other recordDescriptor)]
        pub unsafe fn recordDescriptor() -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Other currentProcessDescriptor)]
        pub unsafe fn currentProcessDescriptor() -> Id<NSAppleEventDescriptor>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptorWithBundleIdentifier:)]
        pub unsafe fn descriptorWithBundleIdentifier(
            bundle_identifier: &NSString,
        ) -> Id<NSAppleEventDescriptor>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other descriptorWithApplicationURL:)]
        pub unsafe fn descriptorWithApplicationURL(
            application_url: &NSURL,
        ) -> Id<NSAppleEventDescriptor>;

        #[method_id(@__retain_semantics Init initListDescriptor)]
        pub unsafe fn initListDescriptor(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initRecordDescriptor)]
        pub unsafe fn initRecordDescriptor(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData>;

        #[method(booleanValue)]
        pub unsafe fn booleanValue(&self) -> Boolean;

        #[method(enumCodeValue)]
        pub unsafe fn enumCodeValue(&self) -> OSType;

        #[method(int32Value)]
        pub unsafe fn int32Value(&self) -> i32;

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(typeCodeValue)]
        pub unsafe fn typeCodeValue(&self) -> OSType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateValue)]
        pub unsafe fn dateValue(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other fileURLValue)]
        pub unsafe fn fileURLValue(&self) -> Option<Id<NSURL>>;

        #[method(isRecordDescriptor)]
        pub unsafe fn isRecordDescriptor(&self) -> bool;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(insertDescriptor:atIndex:)]
        pub unsafe fn insertDescriptor_atIndex(
            &self,
            descriptor: &NSAppleEventDescriptor,
            index: NSInteger,
        );

        #[method_id(@__retain_semantics Other descriptorAtIndex:)]
        pub unsafe fn descriptorAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSAppleEventDescriptor>>;

        #[method(removeDescriptorAtIndex:)]
        pub unsafe fn removeDescriptorAtIndex(&self, index: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAppleEventDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
