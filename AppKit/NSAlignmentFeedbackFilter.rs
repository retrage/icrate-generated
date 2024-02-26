//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSAlignmentFeedbackToken: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn NSAlignmentFeedbackToken {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAlignmentFeedbackFilter;

    unsafe impl ClassType for NSAlignmentFeedbackFilter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSAlignmentFeedbackFilter {}

extern_methods!(
    unsafe impl NSAlignmentFeedbackFilter {
        #[cfg(feature = "AppKit_NSEvent")]
        #[method(inputEventMask)]
        pub unsafe fn inputEventMask() -> NSEventMask;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(updateWithEvent:)]
        pub unsafe fn updateWithEvent(&self, event: &NSEvent);

        #[cfg(all(
            feature = "AppKit_NSGestureRecognizer",
            feature = "AppKit_NSPanGestureRecognizer"
        ))]
        #[method(updateWithPanRecognizer:)]
        pub unsafe fn updateWithPanRecognizer(&self, pan_recognizer: &NSPanGestureRecognizer);

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForMovementInView:previousPoint:alignedPoint:defaultPoint:)]
        pub unsafe fn alignmentFeedbackTokenForMovementInView_previousPoint_alignedPoint_defaultPoint(
            &self,
            view: Option<&NSView>,
            previous_point: NSPoint,
            aligned_point: NSPoint,
            default_point: NSPoint,
        ) -> Option<Id<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForHorizontalMovementInView:previousX:alignedX:defaultX:)]
        pub unsafe fn alignmentFeedbackTokenForHorizontalMovementInView_previousX_alignedX_defaultX(
            &self,
            view: Option<&NSView>,
            previous_x: CGFloat,
            aligned_x: CGFloat,
            default_x: CGFloat,
        ) -> Option<Id<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForVerticalMovementInView:previousY:alignedY:defaultY:)]
        pub unsafe fn alignmentFeedbackTokenForVerticalMovementInView_previousY_alignedY_defaultY(
            &self,
            view: Option<&NSView>,
            previous_y: CGFloat,
            aligned_y: CGFloat,
            default_y: CGFloat,
        ) -> Option<Id<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

        #[cfg(all(feature = "AppKit_NSHapticFeedback", feature = "Foundation_NSArray"))]
        #[method(performFeedback:performanceTime:)]
        pub unsafe fn performFeedback_performanceTime(
            &self,
            alignment_feedback_tokens: &NSArray<ProtocolObject<dyn NSAlignmentFeedbackToken>>,
            performance_time: NSHapticFeedbackPerformanceTime,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAlignmentFeedbackFilter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
