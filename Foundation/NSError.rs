//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSErrorDomain = NSString;

extern_static!(NSCocoaErrorDomain: &'static NSErrorDomain);

extern_static!(NSPOSIXErrorDomain: &'static NSErrorDomain);

extern_static!(NSOSStatusErrorDomain: &'static NSErrorDomain);

extern_static!(NSMachErrorDomain: &'static NSErrorDomain);

pub type NSErrorUserInfoKey = NSString;

extern_static!(NSUnderlyingErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSMultipleUnderlyingErrorsKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedDescriptionKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedFailureReasonErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedRecoverySuggestionErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedRecoveryOptionsErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSRecoveryAttempterErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSHelpAnchorErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSDebugDescriptionErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedFailureErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSStringEncodingErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSURLErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSFilePathErrorKey: &'static NSErrorUserInfoKey);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSError")]
    pub struct NSError;

    #[cfg(feature = "Foundation_NSError")]
    unsafe impl ClassType for NSError {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSError")]
unsafe impl Send for NSError {}

#[cfg(feature = "Foundation_NSError")]
unsafe impl Sync for NSError {}

#[cfg(feature = "Foundation_NSError")]
unsafe impl NSCoding for NSError {}

#[cfg(feature = "Foundation_NSError")]
unsafe impl NSCopying for NSError {}

#[cfg(feature = "Foundation_NSError")]
unsafe impl NSObjectProtocol for NSError {}

#[cfg(feature = "Foundation_NSError")]
unsafe impl NSSecureCoding for NSError {}

extern_methods!(
    #[cfg(feature = "Foundation_NSError")]
    unsafe impl NSError {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithDomain:code:userInfo:)]
        pub unsafe fn initWithDomain_code_userInfo(
            this: Allocated<Self>,
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other errorWithDomain:code:userInfo:)]
        pub unsafe fn errorWithDomain_code_userInfo(
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other domain)]
        pub fn domain(&self) -> Id<NSErrorDomain>;

        #[method(code)]
        pub fn code(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub fn userInfo(&self) -> Id<NSDictionary<NSErrorUserInfoKey, AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedDescription)]
        pub fn localizedDescription(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedFailureReason)]
        pub unsafe fn localizedFailureReason(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedRecoverySuggestion)]
        pub unsafe fn localizedRecoverySuggestion(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedRecoveryOptions)]
        pub unsafe fn localizedRecoveryOptions(&self) -> Option<Id<NSArray<NSString>>>;

        #[method_id(@__retain_semantics Other recoveryAttempter)]
        pub unsafe fn recoveryAttempter(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other underlyingErrors)]
        pub unsafe fn underlyingErrors(&self) -> Id<NSArray<NSError>>;

        #[method(setUserInfoValueProviderForDomain:provider:)]
        pub unsafe fn setUserInfoValueProviderForDomain_provider(
            error_domain: &NSErrorDomain,
            provider: Option<
                &Block<dyn Fn(NonNull<NSError>, NonNull<NSErrorUserInfoKey>) -> *mut AnyObject>,
            >,
        );

        #[method(userInfoValueProviderForDomain:)]
        pub unsafe fn userInfoValueProviderForDomain(
            error_domain: &NSErrorDomain,
        ) -> *mut Block<dyn Fn(NonNull<NSError>, NonNull<NSErrorUserInfoKey>) -> *mut AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSError")]
    unsafe impl NSError {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_category!(
    /// Category "NSErrorRecoveryAttempting" on [`NSObject`].
    #[doc(alias = "NSErrorRecoveryAttempting")]
    pub unsafe trait NSObjectNSErrorRecoveryAttempting {
        #[cfg(feature = "Foundation_NSError")]
        #[method(attemptRecoveryFromError:optionIndex:delegate:didRecoverSelector:contextInfo:)]
        unsafe fn attemptRecoveryFromError_optionIndex_delegate_didRecoverSelector_contextInfo(
            &self,
            error: &NSError,
            recovery_option_index: NSUInteger,
            delegate: Option<&AnyObject>,
            did_recover_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(attemptRecoveryFromError:optionIndex:)]
        unsafe fn attemptRecoveryFromError_optionIndex(
            &self,
            error: &NSError,
            recovery_option_index: NSUInteger,
        ) -> bool;
    }

    unsafe impl NSObjectNSErrorRecoveryAttempting for NSObject {}
);
