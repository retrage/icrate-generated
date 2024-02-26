//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::QuartzCore::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CATransaction;

    unsafe impl ClassType for CATransaction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CATransaction {}

extern_methods!(
    unsafe impl CATransaction {
        #[method(begin)]
        pub fn begin();

        #[method(commit)]
        pub fn commit();

        #[method(flush)]
        pub fn flush();

        #[method(lock)]
        pub unsafe fn lock();

        #[method(unlock)]
        pub unsafe fn unlock();

        #[method(animationDuration)]
        pub fn animationDuration() -> CFTimeInterval;

        #[method(setAnimationDuration:)]
        pub fn setAnimationDuration(dur: CFTimeInterval);

        #[cfg(feature = "QuartzCore_CAMediaTimingFunction")]
        #[method_id(@__retain_semantics Other animationTimingFunction)]
        pub fn animationTimingFunction() -> Option<Id<CAMediaTimingFunction>>;

        #[cfg(feature = "QuartzCore_CAMediaTimingFunction")]
        #[method(setAnimationTimingFunction:)]
        pub fn setAnimationTimingFunction(function: Option<&CAMediaTimingFunction>);

        #[method(disableActions)]
        pub fn disableActions() -> bool;

        #[method(setDisableActions:)]
        pub fn setDisableActions(flag: bool);

        #[method(completionBlock)]
        pub unsafe fn completionBlock() -> *mut Block<dyn Fn()>;

        #[method(setCompletionBlock:)]
        pub unsafe fn setCompletionBlock(block: Option<&Block<dyn Fn()>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(an_object: Option<&AnyObject>, key: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CATransaction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCATransactionAnimationDuration: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCATransactionDisableActions: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCATransactionAnimationTimingFunction: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCATransactionCompletionBlock: &'static NSString);
