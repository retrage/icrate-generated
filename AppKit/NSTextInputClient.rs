//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextCursorAccessoryPlacement {
        #[doc(alias = "NSTextCursorAccessoryPlacementUnspecified")]
        Unspecified = 0,
        #[doc(alias = "NSTextCursorAccessoryPlacementBackward")]
        Backward = 1,
        #[doc(alias = "NSTextCursorAccessoryPlacementForward")]
        Forward = 2,
        #[doc(alias = "NSTextCursorAccessoryPlacementInvisible")]
        Invisible = 3,
        #[doc(alias = "NSTextCursorAccessoryPlacementCenter")]
        Center = 4,
        #[doc(alias = "NSTextCursorAccessoryPlacementOffscreenLeft")]
        OffscreenLeft = 5,
        #[doc(alias = "NSTextCursorAccessoryPlacementOffscreenTop")]
        OffscreenTop = 6,
        #[doc(alias = "NSTextCursorAccessoryPlacementOffscreenRight")]
        OffscreenRight = 7,
        #[doc(alias = "NSTextCursorAccessoryPlacementOffscreenBottom")]
        OffscreenBottom = 8,
    }
);

extern_protocol!(
    pub unsafe trait NSTextInputClient {
        #[cfg(feature = "Foundation_NSRange")]
        #[method(insertText:replacementRange:)]
        unsafe fn insertText_replacementRange(
            &self,
            string: &AnyObject,
            replacement_range: NSRange,
        );

        #[method(doCommandBySelector:)]
        unsafe fn doCommandBySelector(&self, selector: Sel);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setMarkedText:selectedRange:replacementRange:)]
        unsafe fn setMarkedText_selectedRange_replacementRange(
            &self,
            string: &AnyObject,
            selected_range: NSRange,
            replacement_range: NSRange,
        );

        #[method(unmarkText)]
        unsafe fn unmarkText(&self);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(selectedRange)]
        unsafe fn selectedRange(&self) -> NSRange;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(markedRange)]
        unsafe fn markedRange(&self) -> NSRange;

        #[method(hasMarkedText)]
        unsafe fn hasMarkedText(&self) -> bool;

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSRange"
        ))]
        #[method_id(@__retain_semantics Other attributedSubstringForProposedRange:actualRange:)]
        unsafe fn attributedSubstringForProposedRange_actualRange(
            &self,
            range: NSRange,
            actual_range: NSRangePointer,
        ) -> Option<Id<NSAttributedString>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other validAttributesForMarkedText)]
        unsafe fn validAttributesForMarkedText(&self) -> Id<NSArray<NSAttributedStringKey>>;

        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSRange"))]
        #[method(firstRectForCharacterRange:actualRange:)]
        unsafe fn firstRectForCharacterRange_actualRange(
            &self,
            range: NSRange,
            actual_range: NSRangePointer,
        ) -> NSRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(characterIndexForPoint:)]
        unsafe fn characterIndexForPoint(&self, point: NSPoint) -> NSUInteger;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[optional]
        #[method_id(@__retain_semantics Other attributedString)]
        unsafe fn attributedString(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[optional]
        #[method(fractionOfDistanceThroughGlyphForPoint:)]
        unsafe fn fractionOfDistanceThroughGlyphForPoint(&self, point: NSPoint) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[optional]
        #[method(baselineDeltaForCharacterAtIndex:)]
        unsafe fn baselineDeltaForCharacterAtIndex(&self, an_index: NSUInteger) -> CGFloat;

        #[optional]
        #[method(windowLevel)]
        unsafe fn windowLevel(&self) -> NSInteger;

        #[optional]
        #[method(drawsVerticallyForCharacterAtIndex:)]
        unsafe fn drawsVerticallyForCharacterAtIndex(&self, char_index: NSUInteger) -> bool;

        #[optional]
        #[method(preferredTextAccessoryPlacement)]
        unsafe fn preferredTextAccessoryPlacement(&self) -> NSTextCursorAccessoryPlacement;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[optional]
        #[method(unionRectInVisibleSelectedRange)]
        unsafe fn unionRectInVisibleSelectedRange(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[optional]
        #[method(documentVisibleRect)]
        unsafe fn documentVisibleRect(&self) -> NSRect;
    }

    unsafe impl ProtocolType for dyn NSTextInputClient {}
);
