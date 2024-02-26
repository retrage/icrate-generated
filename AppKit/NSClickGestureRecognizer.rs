//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    pub struct NSClickGestureRecognizer;

    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl ClassType for NSClickGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSGestureRecognizer",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSClickGestureRecognizer {}

#[cfg(feature = "AppKit_NSGestureRecognizer")]
unsafe impl NSObjectProtocol for NSClickGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSClickGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;

        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, button_mask: NSUInteger);

        #[method(numberOfClicksRequired)]
        pub unsafe fn numberOfClicksRequired(&self) -> NSInteger;

        #[method(setNumberOfClicksRequired:)]
        pub unsafe fn setNumberOfClicksRequired(&self, number_of_clicks_required: NSInteger);

        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSGestureRecognizer`
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSClickGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSGestureRecognizer")]
    unsafe impl NSClickGestureRecognizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
