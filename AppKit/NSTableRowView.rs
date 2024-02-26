//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    pub struct NSTableRowView;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl ClassType for NSTableRowView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityGroup for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityRow for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSTableRowView {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
unsafe impl NSObjectProtocol for NSTableRowView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTableRowView {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableRowView {
        #[cfg(feature = "AppKit_NSTableView")]
        #[method(selectionHighlightStyle)]
        pub unsafe fn selectionHighlightStyle(&self) -> NSTableViewSelectionHighlightStyle;

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(setSelectionHighlightStyle:)]
        pub unsafe fn setSelectionHighlightStyle(
            &self,
            selection_highlight_style: NSTableViewSelectionHighlightStyle,
        );

        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[method(isGroupRowStyle)]
        pub unsafe fn isGroupRowStyle(&self) -> bool;

        #[method(setGroupRowStyle:)]
        pub unsafe fn setGroupRowStyle(&self, group_row_style: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(isPreviousRowSelected)]
        pub unsafe fn isPreviousRowSelected(&self) -> bool;

        #[method(setPreviousRowSelected:)]
        pub unsafe fn setPreviousRowSelected(&self, previous_row_selected: bool);

        #[method(isNextRowSelected)]
        pub unsafe fn isNextRowSelected(&self) -> bool;

        #[method(setNextRowSelected:)]
        pub unsafe fn setNextRowSelected(&self, next_row_selected: bool);

        #[method(isFloating)]
        pub unsafe fn isFloating(&self) -> bool;

        #[method(setFloating:)]
        pub unsafe fn setFloating(&self, floating: bool);

        #[method(isTargetForDropOperation)]
        pub unsafe fn isTargetForDropOperation(&self) -> bool;

        #[method(setTargetForDropOperation:)]
        pub unsafe fn setTargetForDropOperation(&self, target_for_drop_operation: bool);

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(draggingDestinationFeedbackStyle)]
        pub unsafe fn draggingDestinationFeedbackStyle(
            &self,
        ) -> NSTableViewDraggingDestinationFeedbackStyle;

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(setDraggingDestinationFeedbackStyle:)]
        pub unsafe fn setDraggingDestinationFeedbackStyle(
            &self,
            dragging_destination_feedback_style: NSTableViewDraggingDestinationFeedbackStyle,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(indentationForDropOperation)]
        pub unsafe fn indentationForDropOperation(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setIndentationForDropOperation:)]
        pub unsafe fn setIndentationForDropOperation(
            &self,
            indentation_for_drop_operation: CGFloat,
        );

        #[cfg(feature = "AppKit_NSCell")]
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(drawBackgroundInRect:)]
        pub unsafe fn drawBackgroundInRect(&self, dirty_rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(drawSelectionInRect:)]
        pub unsafe fn drawSelectionInRect(&self, dirty_rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(drawSeparatorInRect:)]
        pub unsafe fn drawSeparatorInRect(&self, dirty_rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(drawDraggingDestinationFeedbackInRect:)]
        pub unsafe fn drawDraggingDestinationFeedbackInRect(&self, dirty_rect: NSRect);

        #[method_id(@__retain_semantics Other viewAtColumn:)]
        pub unsafe fn viewAtColumn(&self, column: NSInteger) -> Option<Id<AnyObject>>;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableRowView {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableRowView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSTableRowView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
