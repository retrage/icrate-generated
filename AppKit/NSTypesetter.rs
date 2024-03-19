//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTypesetter;

    unsafe impl ClassType for NSTypesetter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSTypesetter {}

extern_methods!(
    unsafe impl NSTypesetter {
        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, uses_font_leading: bool);

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(typesetterBehavior)]
        pub unsafe fn typesetterBehavior(&self) -> NSTypesetterBehavior;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(setTypesetterBehavior:)]
        pub unsafe fn setTypesetterBehavior(&self, typesetter_behavior: NSTypesetterBehavior);

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenation_factor: c_float);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, line_fragment_padding: CGFloat);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other substituteFontForFont:)]
        pub unsafe fn substituteFontForFont(&self, original_font: &NSFont) -> Id<NSFont>;

        #[cfg(all(
            feature = "AppKit_NSParagraphStyle",
            feature = "AppKit_NSText",
            feature = "Foundation_NSGeometry"
        ))]
        #[method_id(@__retain_semantics Other textTabForGlyphLocation:writingDirection:maxLocation:)]
        pub unsafe fn textTabForGlyphLocation_writingDirection_maxLocation(
            &self,
            glyph_location: CGFloat,
            direction: NSWritingDirection,
            max_location: CGFloat,
        ) -> Option<Id<NSTextTab>>;

        #[method(bidiProcessingEnabled)]
        pub unsafe fn bidiProcessingEnabled(&self) -> bool;

        #[method(setBidiProcessingEnabled:)]
        pub unsafe fn setBidiProcessingEnabled(&self, bidi_processing_enabled: bool);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attributed_string: Option<&NSAttributedString>);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setParagraphGlyphRange:separatorGlyphRange:)]
        pub unsafe fn setParagraphGlyphRange_separatorGlyphRange(
            &self,
            paragraph_range: NSRange,
            paragraph_separator_range: NSRange,
        );

        #[cfg(feature = "Foundation_NSRange")]
        #[method(paragraphGlyphRange)]
        pub unsafe fn paragraphGlyphRange(&self) -> NSRange;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(paragraphSeparatorGlyphRange)]
        pub unsafe fn paragraphSeparatorGlyphRange(&self) -> NSRange;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(paragraphCharacterRange)]
        pub unsafe fn paragraphCharacterRange(&self) -> NSRange;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(paragraphSeparatorCharacterRange)]
        pub unsafe fn paragraphSeparatorCharacterRange(&self) -> NSRange;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(layoutParagraphAtPoint:)]
        pub unsafe fn layoutParagraphAtPoint(
            &self,
            line_fragment_origin: NSPointPointer,
        ) -> NSUInteger;

        #[method(beginParagraph)]
        pub unsafe fn beginParagraph(&self);

        #[method(endParagraph)]
        pub unsafe fn endParagraph(&self);

        #[method(beginLineWithGlyphAtIndex:)]
        pub unsafe fn beginLineWithGlyphAtIndex(&self, glyph_index: NSUInteger);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(endLineWithGlyphRange:)]
        pub unsafe fn endLineWithGlyphRange(&self, line_glyph_range: NSRange);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(lineSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyph_index: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(paragraphSpacingBeforeGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyph_index: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(paragraphSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyph_index: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;

        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSRange"))]
        #[method(getLineFragmentRect:usedRect:forParagraphSeparatorGlyphRange:atProposedOrigin:)]
        pub unsafe fn getLineFragmentRect_usedRect_forParagraphSeparatorGlyphRange_atProposedOrigin(
            &self,
            line_fragment_rect: NSRectPointer,
            line_fragment_used_rect: NSRectPointer,
            paragraph_separator_glyph_range: NSRange,
            line_origin: NSPoint,
        );

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other attributesForExtraLineFragment)]
        pub unsafe fn attributesForExtraLineFragment(
            &self,
        ) -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<NSLayoutManager>>;

        #[cfg(all(feature = "AppKit_NSTextContainer", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textContainers)]
        pub unsafe fn textContainers(&self) -> Option<Id<NSArray<NSTextContainer>>>;

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method_id(@__retain_semantics Other currentTextContainer)]
        pub unsafe fn currentTextContainer(&self) -> Option<Id<NSTextContainer>>;

        #[cfg(feature = "AppKit_NSParagraphStyle")]
        #[method_id(@__retain_semantics Other currentParagraphStyle)]
        pub unsafe fn currentParagraphStyle(&self) -> Option<Id<NSParagraphStyle>>;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setHardInvalidation:forGlyphRange:)]
        pub unsafe fn setHardInvalidation_forGlyphRange(&self, flag: bool, glyph_range: NSRange);

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(layoutGlyphsInLayoutManager:startingAtGlyphIndex:maxNumberOfLineFragments:nextGlyphIndex:)]
        pub unsafe fn layoutGlyphsInLayoutManager_startingAtGlyphIndex_maxNumberOfLineFragments_nextGlyphIndex(
            &self,
            layout_manager: &NSLayoutManager,
            start_glyph_index: NSUInteger,
            max_num_lines: NSUInteger,
            next_glyph: NonNull<NSUInteger>,
        );

        #[cfg(all(feature = "AppKit_NSLayoutManager", feature = "Foundation_NSRange"))]
        #[method(layoutCharactersInRange:forLayoutManager:maximumNumberOfLineFragments:)]
        pub unsafe fn layoutCharactersInRange_forLayoutManager_maximumNumberOfLineFragments(
            &self,
            character_range: NSRange,
            layout_manager: &NSLayoutManager,
            max_num_lines: NSUInteger,
        ) -> NSRange;

        #[cfg(all(
            feature = "AppKit_NSLayoutManager",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSRange"
        ))]
        #[method(printingAdjustmentInLayoutManager:forNominallySpacedGlyphRange:packedGlyphs:count:)]
        pub unsafe fn printingAdjustmentInLayoutManager_forNominallySpacedGlyphRange_packedGlyphs_count(
            layout_mgr: &NSLayoutManager,
            nominally_spaced_glyphs_range: NSRange,
            packed_glyphs: NonNull<c_uchar>,
            packed_glyphs_count: NSUInteger,
        ) -> NSSize;

        #[cfg(all(feature = "AppKit_NSLayoutManager", feature = "Foundation_NSGeometry"))]
        #[method(baselineOffsetInLayoutManager:glyphIndex:)]
        pub unsafe fn baselineOffsetInLayoutManager_glyphIndex(
            &self,
            layout_mgr: &NSLayoutManager,
            glyph_index: NSUInteger,
        ) -> CGFloat;

        #[method_id(@__retain_semantics Other sharedSystemTypesetter)]
        pub unsafe fn sharedSystemTypesetter() -> Id<NSTypesetter>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method_id(@__retain_semantics Other sharedSystemTypesetterForBehavior:)]
        pub unsafe fn sharedSystemTypesetterForBehavior(
            behavior: NSTypesetterBehavior,
        ) -> Id<AnyObject>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(defaultTypesetterBehavior)]
        pub unsafe fn defaultTypesetterBehavior() -> NSTypesetterBehavior;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTypesetter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSLayoutPhaseInterface
    unsafe impl NSTypesetter {
        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSRange"))]
        #[method(willSetLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn willSetLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            line_rect: NSRectPointer,
            glyph_range: NSRange,
            used_rect: NSRectPointer,
            baseline_offset: NonNull<CGFloat>,
        );

        #[method(shouldBreakLineByWordBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByWordBeforeCharacterAtIndex(
            &self,
            char_index: NSUInteger,
        ) -> bool;

        #[method(shouldBreakLineByHyphenatingBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByHyphenatingBeforeCharacterAtIndex(
            &self,
            char_index: NSUInteger,
        ) -> bool;

        #[method(hyphenationFactorForGlyphAtIndex:)]
        pub unsafe fn hyphenationFactorForGlyphAtIndex(&self, glyph_index: NSUInteger) -> c_float;

        #[method(hyphenCharacterForGlyphAtIndex:)]
        pub unsafe fn hyphenCharacterForGlyphAtIndex(&self, glyph_index: NSUInteger) -> UTF32Char;

        #[cfg(all(feature = "AppKit_NSTextContainer", feature = "Foundation_NSGeometry"))]
        #[method(boundingBoxForControlGlyphAtIndex:forTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        pub unsafe fn boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            glyph_index: NSUInteger,
            text_container: &NSTextContainer,
            proposed_rect: NSRect,
            glyph_position: NSPoint,
            char_index: NSUInteger,
        ) -> NSRect;
    }
);

extern_methods!(
    /// NSGlyphStorageInterface
    unsafe impl NSTypesetter {
        #[cfg(feature = "Foundation_NSRange")]
        #[method(characterRangeForGlyphRange:actualGlyphRange:)]
        pub unsafe fn characterRangeForGlyphRange_actualGlyphRange(
            &self,
            glyph_range: NSRange,
            actual_glyph_range: NSRangePointer,
        ) -> NSRange;

        #[cfg(feature = "Foundation_NSRange")]
        #[method(glyphRangeForCharacterRange:actualCharacterRange:)]
        pub unsafe fn glyphRangeForCharacterRange_actualCharacterRange(
            &self,
            char_range: NSRange,
            actual_char_range: NSRangePointer,
        ) -> NSRange;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(getLineFragmentRect:usedRect:remainingRect:forStartingGlyphAtIndex:proposedRect:lineSpacing:paragraphSpacingBefore:paragraphSpacingAfter:)]
        pub unsafe fn getLineFragmentRect_usedRect_remainingRect_forStartingGlyphAtIndex_proposedRect_lineSpacing_paragraphSpacingBefore_paragraphSpacingAfter(
            &self,
            line_fragment_rect: NSRectPointer,
            line_fragment_used_rect: NSRectPointer,
            remaining_rect: NSRectPointer,
            starting_glyph_index: NSUInteger,
            proposed_rect: NSRect,
            line_spacing: CGFloat,
            paragraph_spacing_before: CGFloat,
            paragraph_spacing_after: CGFloat,
        );

        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSRange"))]
        #[method(setLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn setLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            fragment_rect: NSRect,
            glyph_range: NSRange,
            used_rect: NSRect,
            baseline_offset: CGFloat,
        );

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setNotShownAttribute:forGlyphRange:)]
        pub unsafe fn setNotShownAttribute_forGlyphRange(&self, flag: bool, glyph_range: NSRange);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setDrawsOutsideLineFragment:forGlyphRange:)]
        pub unsafe fn setDrawsOutsideLineFragment_forGlyphRange(
            &self,
            flag: bool,
            glyph_range: NSRange,
        );

        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSRange"))]
        #[method(setLocation:withAdvancements:forStartOfGlyphRange:)]
        pub unsafe fn setLocation_withAdvancements_forStartOfGlyphRange(
            &self,
            location: NSPoint,
            advancements: *mut CGFloat,
            glyph_range: NSRange,
        );

        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSRange"))]
        #[method(setAttachmentSize:forGlyphRange:)]
        pub unsafe fn setAttachmentSize_forGlyphRange(
            &self,
            attachment_size: NSSize,
            glyph_range: NSRange,
        );

        #[cfg(feature = "Foundation_NSRange")]
        #[method(setBidiLevels:forGlyphRange:)]
        pub unsafe fn setBidiLevels_forGlyphRange(&self, levels: *mut u8, glyph_range: NSRange);
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTypesetterControlCharacterAction(pub NSUInteger);
impl NSTypesetterControlCharacterAction {
    pub const NSTypesetterZeroAdvancementAction: Self = Self(1 << 0);
    pub const NSTypesetterWhitespaceAction: Self = Self(1 << 1);
    pub const NSTypesetterHorizontalTabAction: Self = Self(1 << 2);
    pub const NSTypesetterLineBreakAction: Self = Self(1 << 3);
    pub const NSTypesetterParagraphBreakAction: Self = Self(1 << 4);
    pub const NSTypesetterContainerBreakAction: Self = Self(1 << 5);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTypesetterControlCharacterAction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTypesetterControlCharacterAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSTypesetter_Deprecated
    unsafe impl NSTypesetter {
        #[method(actionForControlCharacterAtIndex:)]
        pub unsafe fn actionForControlCharacterAtIndex(
            &self,
            char_index: NSUInteger,
        ) -> NSTypesetterControlCharacterAction;

        #[cfg(all(
            feature = "AppKit_NSFont",
            feature = "AppKit_NSLayoutManager",
            feature = "Foundation_NSRange"
        ))]
        #[deprecated]
        #[method(getGlyphsInRange:glyphs:characterIndexes:glyphInscriptions:elasticBits:bidiLevels:)]
        pub unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_bidiLevels(
            &self,
            glyphs_range: NSRange,
            glyph_buffer: *mut NSGlyph,
            char_index_buffer: *mut NSUInteger,
            inscribe_buffer: *mut NSGlyphInscription,
            elastic_buffer: *mut Bool,
            bidi_level_buffer: *mut c_uchar,
        ) -> NSUInteger;

        #[cfg(all(feature = "AppKit_NSFont", feature = "Foundation_NSRange"))]
        #[deprecated]
        #[method(substituteGlyphsInRange:withGlyphs:)]
        pub unsafe fn substituteGlyphsInRange_withGlyphs(
            &self,
            glyph_range: NSRange,
            glyphs: *mut NSGlyph,
        );

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated]
        #[method(insertGlyph:atGlyphIndex:characterIndex:)]
        pub unsafe fn insertGlyph_atGlyphIndex_characterIndex(
            &self,
            glyph: NSGlyph,
            glyph_index: NSUInteger,
            character_index: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSRange")]
        #[deprecated]
        #[method(deleteGlyphsInRange:)]
        pub unsafe fn deleteGlyphsInRange(&self, glyph_range: NSRange);
    }
);
