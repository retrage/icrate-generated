//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSLayoutPriority = c_float;

pub static NSLayoutPriorityRequired: NSLayoutPriority = 1000 as _;

pub static NSLayoutPriorityDefaultHigh: NSLayoutPriority = 750 as _;

pub static NSLayoutPriorityDragThatCanResizeWindow: NSLayoutPriority = 510 as _;

pub static NSLayoutPriorityWindowSizeStayPut: NSLayoutPriority = 500 as _;

pub static NSLayoutPriorityDragThatCannotResizeWindow: NSLayoutPriority = 490 as _;

pub static NSLayoutPriorityDefaultLow: NSLayoutPriority = 250 as _;

pub static NSLayoutPriorityFittingSizeCompression: NSLayoutPriority = 50 as _;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutConstraintOrientation(pub NSInteger);
impl NSLayoutConstraintOrientation {
    #[doc(alias = "NSLayoutConstraintOrientationHorizontal")]
    pub const Horizontal: Self = Self(0);
    #[doc(alias = "NSLayoutConstraintOrientationVertical")]
    pub const Vertical: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSLayoutConstraintOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSLayoutConstraintOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutRelation(pub NSInteger);
impl NSLayoutRelation {
    #[doc(alias = "NSLayoutRelationLessThanOrEqual")]
    pub const LessThanOrEqual: Self = Self(-1);
    #[doc(alias = "NSLayoutRelationEqual")]
    pub const Equal: Self = Self(0);
    #[doc(alias = "NSLayoutRelationGreaterThanOrEqual")]
    pub const GreaterThanOrEqual: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSLayoutRelation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSLayoutRelation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutAttribute(pub NSInteger);
impl NSLayoutAttribute {
    #[doc(alias = "NSLayoutAttributeLeft")]
    pub const Left: Self = Self(1);
    #[doc(alias = "NSLayoutAttributeRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "NSLayoutAttributeTop")]
    pub const Top: Self = Self(3);
    #[doc(alias = "NSLayoutAttributeBottom")]
    pub const Bottom: Self = Self(4);
    #[doc(alias = "NSLayoutAttributeLeading")]
    pub const Leading: Self = Self(5);
    #[doc(alias = "NSLayoutAttributeTrailing")]
    pub const Trailing: Self = Self(6);
    #[doc(alias = "NSLayoutAttributeWidth")]
    pub const Width: Self = Self(7);
    #[doc(alias = "NSLayoutAttributeHeight")]
    pub const Height: Self = Self(8);
    #[doc(alias = "NSLayoutAttributeCenterX")]
    pub const CenterX: Self = Self(9);
    #[doc(alias = "NSLayoutAttributeCenterY")]
    pub const CenterY: Self = Self(10);
    #[doc(alias = "NSLayoutAttributeLastBaseline")]
    pub const LastBaseline: Self = Self(11);
    #[doc(alias = "NSLayoutAttributeBaseline")]
    pub const Baseline: Self = Self(NSLayoutAttribute::LastBaseline.0);
    #[doc(alias = "NSLayoutAttributeFirstBaseline")]
    pub const FirstBaseline: Self = Self(12);
    #[doc(alias = "NSLayoutAttributeNotAnAttribute")]
    pub const NotAnAttribute: Self = Self(0);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSLayoutAttribute {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSLayoutAttribute {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutFormatOptions(pub NSUInteger);
impl NSLayoutFormatOptions {
    pub const NSLayoutFormatAlignAllLeft: Self = Self(1 << NSLayoutAttribute::Left.0);
    pub const NSLayoutFormatAlignAllRight: Self = Self(1 << NSLayoutAttribute::Right.0);
    pub const NSLayoutFormatAlignAllTop: Self = Self(1 << NSLayoutAttribute::Top.0);
    pub const NSLayoutFormatAlignAllBottom: Self = Self(1 << NSLayoutAttribute::Bottom.0);
    pub const NSLayoutFormatAlignAllLeading: Self = Self(1 << NSLayoutAttribute::Leading.0);
    pub const NSLayoutFormatAlignAllTrailing: Self = Self(1 << NSLayoutAttribute::Trailing.0);
    pub const NSLayoutFormatAlignAllCenterX: Self = Self(1 << NSLayoutAttribute::CenterX.0);
    pub const NSLayoutFormatAlignAllCenterY: Self = Self(1 << NSLayoutAttribute::CenterY.0);
    pub const NSLayoutFormatAlignAllLastBaseline: Self =
        Self(1 << NSLayoutAttribute::LastBaseline.0);
    pub const NSLayoutFormatAlignAllFirstBaseline: Self =
        Self(1 << NSLayoutAttribute::FirstBaseline.0);
    pub const NSLayoutFormatAlignAllBaseline: Self =
        Self(NSLayoutFormatOptions::NSLayoutFormatAlignAllLastBaseline.0);
    pub const NSLayoutFormatAlignmentMask: Self = Self(0xFFFF);
    pub const NSLayoutFormatDirectionLeadingToTrailing: Self = Self(0 << 16);
    pub const NSLayoutFormatDirectionLeftToRight: Self = Self(1 << 16);
    pub const NSLayoutFormatDirectionRightToLeft: Self = Self(2 << 16);
    pub const NSLayoutFormatDirectionMask: Self = Self(0x3 << 16);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSLayoutFormatOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSLayoutFormatOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutConstraint;

    unsafe impl ClassType for NSLayoutConstraint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSLayoutConstraint {}

extern_methods!(
    unsafe impl NSLayoutConstraint {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other constraintsWithVisualFormat:options:metrics:views:)]
        pub unsafe fn constraintsWithVisualFormat_options_metrics_views(
            format: &NSString,
            opts: NSLayoutFormatOptions,
            metrics: Option<&NSDictionary<NSString, AnyObject>>,
            views: &NSDictionary<NSString, AnyObject>,
        ) -> Id<NSArray<NSLayoutConstraint>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:)]
        pub unsafe fn constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
            view1: &AnyObject,
            attr1: NSLayoutAttribute,
            relation: NSLayoutRelation,
            view2: Option<&AnyObject>,
            attr2: NSLayoutAttribute,
            multiplier: CGFloat,
            c: CGFloat,
        ) -> Id<Self>;

        #[method(priority)]
        pub unsafe fn priority(&self) -> NSLayoutPriority;

        #[method(setPriority:)]
        pub unsafe fn setPriority(&self, priority: NSLayoutPriority);

        #[method(shouldBeArchived)]
        pub unsafe fn shouldBeArchived(&self) -> bool;

        #[method(setShouldBeArchived:)]
        pub unsafe fn setShouldBeArchived(&self, should_be_archived: bool);

        #[method_id(@__retain_semantics Other firstItem)]
        pub unsafe fn firstItem(&self) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other secondItem)]
        pub unsafe fn secondItem(&self) -> Option<Id<AnyObject>>;

        #[method(firstAttribute)]
        pub unsafe fn firstAttribute(&self) -> NSLayoutAttribute;

        #[method(secondAttribute)]
        pub unsafe fn secondAttribute(&self) -> NSLayoutAttribute;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other firstAnchor)]
        pub unsafe fn firstAnchor(&self) -> Id<NSLayoutAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other secondAnchor)]
        pub unsafe fn secondAnchor(&self) -> Option<Id<NSLayoutAnchor>>;

        #[method(relation)]
        pub unsafe fn relation(&self) -> NSLayoutRelation;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(multiplier)]
        pub unsafe fn multiplier(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(constant)]
        pub unsafe fn constant(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setConstant:)]
        pub unsafe fn setConstant(&self, constant: CGFloat);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(activateConstraints:)]
        pub unsafe fn activateConstraints(constraints: &NSArray<NSLayoutConstraint>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deactivateConstraints:)]
        pub unsafe fn deactivateConstraints(constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSIdentifier
    unsafe impl NSLayoutConstraint {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);
    }
);

extern_methods!(
    unsafe impl NSLayoutConstraint {}
);

#[cfg(feature = "AppKit_NSAnimation")]
unsafe impl NSAnimatablePropertyContainer for NSLayoutConstraint {}

extern_methods!(
    /// NSConstraintBasedLayoutInstallingConstraints
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        pub unsafe fn topAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Id<NSLayoutDimension>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Id<NSLayoutDimension>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other firstBaselineAnchor)]
        pub unsafe fn firstBaselineAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other lastBaselineAnchor)]
        pub unsafe fn lastBaselineAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other constraints)]
        pub unsafe fn constraints(&self) -> Id<NSArray<NSLayoutConstraint>>;

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(addConstraint:)]
        pub unsafe fn addConstraint(&self, constraint: &NSLayoutConstraint);

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method(addConstraints:)]
        pub unsafe fn addConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(removeConstraint:)]
        pub unsafe fn removeConstraint(&self, constraint: &NSLayoutConstraint);

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method(removeConstraints:)]
        pub unsafe fn removeConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSWindow"))]
    unsafe impl NSWindow {
        #[method(updateConstraintsIfNeeded)]
        pub unsafe fn updateConstraintsIfNeeded(&self);

        #[method(layoutIfNeeded)]
        pub unsafe fn layoutIfNeeded(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[method(updateConstraintsForSubtreeIfNeeded)]
        pub unsafe fn updateConstraintsForSubtreeIfNeeded(&self);

        #[method(updateConstraints)]
        pub unsafe fn updateConstraints(&self);

        #[method(needsUpdateConstraints)]
        pub unsafe fn needsUpdateConstraints(&self) -> bool;

        #[method(setNeedsUpdateConstraints:)]
        pub unsafe fn setNeedsUpdateConstraints(&self, needs_update_constraints: bool);
    }
);

extern_methods!(
    /// NSConstraintBasedCompatibility
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[method(translatesAutoresizingMaskIntoConstraints)]
        pub unsafe fn translatesAutoresizingMaskIntoConstraints(&self) -> bool;

        #[method(setTranslatesAutoresizingMaskIntoConstraints:)]
        pub unsafe fn setTranslatesAutoresizingMaskIntoConstraints(
            &self,
            translates_autoresizing_mask_into_constraints: bool,
        );

        #[method(requiresConstraintBasedLayout)]
        pub unsafe fn requiresConstraintBasedLayout(mtm: MainThreadMarker) -> bool;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSGeometry")]
    pub static NSViewNoInstrinsicMetric: CGFloat;
}

extern "C" {
    #[cfg(feature = "Foundation_NSGeometry")]
    pub static NSViewNoIntrinsicMetric: CGFloat;
}

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(alignmentRectForFrame:)]
        pub unsafe fn alignmentRectForFrame(&self, frame: NSRect) -> NSRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(frameForAlignmentRect:)]
        pub unsafe fn frameForAlignmentRect(&self, alignment_rect: NSRect) -> NSRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(alignmentRectInsets)]
        pub unsafe fn alignmentRectInsets(&self) -> NSEdgeInsets;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(firstBaselineOffsetFromTop)]
        pub unsafe fn firstBaselineOffsetFromTop(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(lastBaselineOffsetFromBottom)]
        pub unsafe fn lastBaselineOffsetFromBottom(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(baselineOffsetFromBottom)]
        pub unsafe fn baselineOffsetFromBottom(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(intrinsicContentSize)]
        pub unsafe fn intrinsicContentSize(&self) -> NSSize;

        #[method(invalidateIntrinsicContentSize)]
        pub unsafe fn invalidateIntrinsicContentSize(&self);

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(contentHuggingPriorityForOrientation:)]
        pub unsafe fn contentHuggingPriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(setContentHuggingPriority:forOrientation:)]
        pub unsafe fn setContentHuggingPriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(contentCompressionResistancePriorityForOrientation:)]
        pub unsafe fn contentCompressionResistancePriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(setContentCompressionResistancePriority:forOrientation:)]
        pub unsafe fn setContentCompressionResistancePriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(isHorizontalContentSizeConstraintActive)]
        pub unsafe fn isHorizontalContentSizeConstraintActive(&self) -> bool;

        #[method(setHorizontalContentSizeConstraintActive:)]
        pub unsafe fn setHorizontalContentSizeConstraintActive(
            &self,
            horizontal_content_size_constraint_active: bool,
        );

        #[method(isVerticalContentSizeConstraintActive)]
        pub unsafe fn isVerticalContentSizeConstraintActive(&self) -> bool;

        #[method(setVerticalContentSizeConstraintActive:)]
        pub unsafe fn setVerticalContentSizeConstraintActive(
            &self,
            vertical_content_size_constraint_active: bool,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSControl {
        #[cfg(feature = "AppKit_NSCell")]
        #[method(invalidateIntrinsicContentSizeForCell:)]
        pub unsafe fn invalidateIntrinsicContentSizeForCell(&self, cell: &NSCell);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutAnchoring
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSWindow"))]
    unsafe impl NSWindow {
        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(anchorAttributeForOrientation:)]
        pub unsafe fn anchorAttributeForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutAttribute;

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(setAnchorAttribute:forOrientation:)]
        pub unsafe fn setAnchorAttribute_forOrientation(
            &self,
            attr: NSLayoutAttribute,
            orientation: NSLayoutConstraintOrientation,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutFittingSize
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(fittingSize)]
        pub unsafe fn fittingSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Id<NSArray<NSLayoutConstraint>>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[method(exerciseAmbiguityInLayout)]
        pub unsafe fn exerciseAmbiguityInLayout(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSWindow"))]
    unsafe impl NSWindow {
        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method(visualizeConstraints:)]
        pub unsafe fn visualizeConstraints(
            &self,
            constraints: Option<&NSArray<NSLayoutConstraint>>,
        );
    }
);
