//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSInvalidArchiveOperationException: &'static NSExceptionName);

extern_static!(NSInvalidUnarchiveOperationException: &'static NSExceptionName);

extern_static!(NSKeyedArchiveRootObjectKey: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSKeyedArchiver")]
    pub struct NSKeyedArchiver;

    #[cfg(feature = "Foundation_NSKeyedArchiver")]
    unsafe impl ClassType for NSKeyedArchiver {
        #[inherits(NSObject)]
        type Super = NSCoder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSKeyedArchiver")]
unsafe impl NSObjectProtocol for NSKeyedArchiver {}

extern_methods!(
    #[cfg(feature = "Foundation_NSKeyedArchiver")]
    unsafe impl NSKeyedArchiver {
        #[method_id(@__retain_semantics Init initRequiringSecureCoding:)]
        pub unsafe fn initRequiringSecureCoding(
            this: Allocated<Self>,
            requires_secure_coding: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other archivedDataWithRootObject:requiringSecureCoding:error:_)]
        pub unsafe fn archivedDataWithRootObject_requiringSecureCoding_error(
            object: &AnyObject,
            requires_secure_coding: bool,
        ) -> Result<Id<NSData>, Id<NSError>>;

        #[deprecated = "Use -initRequiringSecureCoding: instead"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSMutableData")]
        #[deprecated = "Use -initRequiringSecureCoding: instead"]
        #[method_id(@__retain_semantics Init initForWritingWithMutableData:)]
        pub unsafe fn initForWritingWithMutableData(
            this: Allocated<Self>,
            data: &NSMutableData,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "Use +archivedDataWithRootObject:requiringSecureCoding:error: instead"]
        #[method_id(@__retain_semantics Other archivedDataWithRootObject:)]
        pub unsafe fn archivedDataWithRootObject(root_object: &AnyObject) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use +archivedDataWithRootObject:requiringSecureCoding:error: and -writeToURL:options:error: instead"]
        #[method(archiveRootObject:toFile:)]
        pub unsafe fn archiveRootObject_toFile(root_object: &AnyObject, path: &NSString) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSKeyedArchiverDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSKeyedArchiverDelegate>>,
        );

        #[method(outputFormat)]
        pub unsafe fn outputFormat(&self) -> NSPropertyListFormat;

        #[method(setOutputFormat:)]
        pub unsafe fn setOutputFormat(&self, output_format: NSPropertyListFormat);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other encodedData)]
        pub unsafe fn encodedData(&self) -> Id<NSData>;

        #[method(finishEncoding)]
        pub unsafe fn finishEncoding(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClassName:forClass:)]
        pub unsafe fn setClassName_forClass_class(coded_name: Option<&NSString>, cls: &AnyClass);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClassName:forClass:)]
        pub unsafe fn setClassName_forClass(&self, coded_name: Option<&NSString>, cls: &AnyClass);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other classNameForClass:)]
        pub unsafe fn classNameForClass_class(cls: &AnyClass) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other classNameForClass:)]
        pub unsafe fn classNameForClass(&self, cls: &AnyClass) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeObject:forKey:)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeConditionalObject:forKey:)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&AnyObject>,
            key: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeBool:forKey:)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeInt:forKey:)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeInt32:forKey:)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeInt64:forKey:)]
        pub unsafe fn encodeInt64_forKey(&self, value: i64, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeFloat:forKey:)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeDouble:forKey:)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeBytes:length:forKey:)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *mut u8,
            length: NSUInteger,
            key: &NSString,
        );

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[method(setRequiresSecureCoding:)]
        pub unsafe fn setRequiresSecureCoding(&self, requires_secure_coding: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSKeyedArchiver")]
    unsafe impl NSKeyedArchiver {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSKeyedUnarchiver")]
    pub struct NSKeyedUnarchiver;

    #[cfg(feature = "Foundation_NSKeyedUnarchiver")]
    unsafe impl ClassType for NSKeyedUnarchiver {
        #[inherits(NSObject)]
        type Super = NSCoder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSKeyedUnarchiver")]
unsafe impl NSObjectProtocol for NSKeyedUnarchiver {}

extern_methods!(
    #[cfg(feature = "Foundation_NSKeyedUnarchiver")]
    unsafe impl NSKeyedUnarchiver {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Init initForReadingFromData:error:_)]
        pub unsafe fn initForReadingFromData_error(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other unarchivedObjectOfClass:fromData:error:_)]
        pub unsafe fn unarchivedObjectOfClass_fromData_error(
            cls: &AnyClass,
            data: &NSData,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other unarchivedArrayOfObjectsOfClass:fromData:error:_)]
        pub unsafe fn unarchivedArrayOfObjectsOfClass_fromData_error(
            cls: &AnyClass,
            data: &NSData,
        ) -> Result<Id<NSArray>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other unarchivedDictionaryWithKeysOfClass:objectsOfClass:fromData:error:_)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClass_objectsOfClass_fromData_error(
            key_cls: &AnyClass,
            value_cls: &AnyClass,
            data: &NSData,
        ) -> Result<Id<NSDictionary>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other unarchivedObjectOfClasses:fromData:error:_)]
        pub unsafe fn unarchivedObjectOfClasses_fromData_error(
            classes: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other unarchivedArrayOfObjectsOfClasses:fromData:error:_)]
        pub unsafe fn unarchivedArrayOfObjectsOfClasses_fromData_error(
            classes: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<NSArray>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other unarchivedDictionaryWithKeysOfClasses:objectsOfClasses:fromData:error:_)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClasses_objectsOfClasses_fromData_error(
            key_classes: &NSSet<TodoClass>,
            value_classes: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<NSDictionary>, Id<NSError>>;

        #[deprecated = "Use -initForReadingFromData:error: instead"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "Use -initForReadingFromData:error: instead"]
        #[method_id(@__retain_semantics Init initForReadingWithData:)]
        pub unsafe fn initForReadingWithData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "Use +unarchivedObjectOfClass:fromData:error: instead"]
        #[method_id(@__retain_semantics Other unarchiveObjectWithData:)]
        pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Id<AnyObject>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[deprecated = "Use +unarchivedObjectOfClass:fromData:error: instead"]
        #[method_id(@__retain_semantics Other unarchiveTopLevelObjectWithData:error:_)]
        pub unsafe fn unarchiveTopLevelObjectWithData_error(
            data: &NSData,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use +unarchivedObjectOfClass:fromData:error: instead"]
        #[method_id(@__retain_semantics Other unarchiveObjectWithFile:)]
        pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSKeyedUnarchiverDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSKeyedUnarchiverDelegate>>,
        );

        #[method(finishDecoding)]
        pub unsafe fn finishDecoding(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClass:forClassName:)]
        pub unsafe fn setClass_forClassName_class(cls: Option<&AnyClass>, coded_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClass:forClassName:)]
        pub unsafe fn setClass_forClassName(&self, cls: Option<&AnyClass>, coded_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(classForClassName:)]
        pub unsafe fn classForClassName_class(coded_name: &NSString) -> Option<&'static AnyClass>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(classForClassName:)]
        pub unsafe fn classForClassName(&self, coded_name: &NSString) -> Option<&'static AnyClass>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(containsValueForKey:)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decodeObjectForKey:)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeBoolForKey:)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeIntForKey:)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeInt32ForKey:)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeInt64ForKey:)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> i64;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeFloatForKey:)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeDoubleForKey:)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeBytesForKey:returnedLength:)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *mut u8;

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[method(setRequiresSecureCoding:)]
        pub unsafe fn setRequiresSecureCoding(&self, requires_secure_coding: bool);

        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

        #[method(setDecodingFailurePolicy:)]
        pub unsafe fn setDecodingFailurePolicy(
            &self,
            decoding_failure_policy: NSDecodingFailurePolicy,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSKeyedUnarchiver")]
    unsafe impl NSKeyedUnarchiver {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSKeyedArchiverDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSKeyedArchiver")]
        #[optional]
        #[method_id(@__retain_semantics Other archiver:willEncodeObject:)]
        unsafe fn archiver_willEncodeObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: &AnyObject,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSKeyedArchiver")]
        #[optional]
        #[method(archiver:didEncodeObject:)]
        unsafe fn archiver_didEncodeObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: Option<&AnyObject>,
        );

        #[cfg(feature = "Foundation_NSKeyedArchiver")]
        #[optional]
        #[method(archiver:willReplaceObject:withObject:)]
        unsafe fn archiver_willReplaceObject_withObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: Option<&AnyObject>,
            new_object: Option<&AnyObject>,
        );

        #[cfg(feature = "Foundation_NSKeyedArchiver")]
        #[optional]
        #[method(archiverWillFinish:)]
        unsafe fn archiverWillFinish(&self, archiver: &NSKeyedArchiver);

        #[cfg(feature = "Foundation_NSKeyedArchiver")]
        #[optional]
        #[method(archiverDidFinish:)]
        unsafe fn archiverDidFinish(&self, archiver: &NSKeyedArchiver);
    }

    unsafe impl ProtocolType for dyn NSKeyedArchiverDelegate {}
);

extern_protocol!(
    pub unsafe trait NSKeyedUnarchiverDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSKeyedUnarchiver",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method(unarchiver:cannotDecodeObjectOfClassName:originalClasses:)]
        unsafe fn unarchiver_cannotDecodeObjectOfClassName_originalClasses(
            &self,
            unarchiver: &NSKeyedUnarchiver,
            name: &NSString,
            class_names: &NSArray<NSString>,
        ) -> Option<&'static AnyClass>;

        #[cfg(feature = "Foundation_NSKeyedUnarchiver")]
        #[optional]
        #[method(unarchiver:willReplaceObject:withObject:)]
        unsafe fn unarchiver_willReplaceObject_withObject(
            &self,
            unarchiver: &NSKeyedUnarchiver,
            object: &AnyObject,
            new_object: &AnyObject,
        );

        #[cfg(feature = "Foundation_NSKeyedUnarchiver")]
        #[optional]
        #[method(unarchiverWillFinish:)]
        unsafe fn unarchiverWillFinish(&self, unarchiver: &NSKeyedUnarchiver);

        #[cfg(feature = "Foundation_NSKeyedUnarchiver")]
        #[optional]
        #[method(unarchiverDidFinish:)]
        unsafe fn unarchiverDidFinish(&self, unarchiver: &NSKeyedUnarchiver);
    }

    unsafe impl ProtocolType for dyn NSKeyedUnarchiverDelegate {}
);

extern_category!(
    /// Category "NSKeyedArchiverObjectSubstitution" on [`NSObject`].
    #[doc(alias = "NSKeyedArchiverObjectSubstitution")]
    pub unsafe trait NSObjectNSKeyedArchiverObjectSubstitution {
        #[method(classForKeyedArchiver)]
        unsafe fn classForKeyedArchiver(&self) -> Option<&'static AnyClass>;

        #[cfg(feature = "Foundation_NSKeyedArchiver")]
        #[method_id(@__retain_semantics Other replacementObjectForKeyedArchiver:)]
        unsafe fn replacementObjectForKeyedArchiver(
            &self,
            archiver: &NSKeyedArchiver,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other classFallbacksForKeyedArchiver)]
        unsafe fn classFallbacksForKeyedArchiver() -> Id<NSArray<NSString>>;
    }

    unsafe impl NSObjectNSKeyedArchiverObjectSubstitution for NSObject {}
);

extern_category!(
    /// Category "NSKeyedUnarchiverObjectSubstitution" on [`NSObject`].
    #[doc(alias = "NSKeyedUnarchiverObjectSubstitution")]
    pub unsafe trait NSObjectNSKeyedUnarchiverObjectSubstitution {
        #[method(classForKeyedUnarchiver)]
        unsafe fn classForKeyedUnarchiver() -> &'static AnyClass;
    }

    unsafe impl NSObjectNSKeyedUnarchiverObjectSubstitution for NSObject {}
);
