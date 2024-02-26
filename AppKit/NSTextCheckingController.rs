//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextCheckingController;

    unsafe impl ClassType for NSTextCheckingController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSTextCheckingController {}

extern_methods!(
    unsafe impl NSTextCheckingController {
        #[cfg(all(
            feature = "AppKit_NSTextCheckingClient",
            feature = "AppKit_NSTextInputClient"
        ))]
        #[method_id(@__retain_semantics Init initWithClient:)]
        pub unsafe fn initWithClient(
            this: Allocated<Self>,
            client: &ProtocolObject<dyn NSTextCheckingClient>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "AppKit_NSTextCheckingClient",
            feature = "AppKit_NSTextInputClient"
        ))]
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Id<ProtocolObject<dyn NSTextCheckingClient>>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(didChangeTextInRange:)]
        pub unsafe fn didChangeTextInRange(&self, range: NSRange);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(insertedTextInRange:)]
        pub unsafe fn insertedTextInRange(&self, range: NSRange);

        #[method(didChangeSelectedRange)]
        pub unsafe fn didChangeSelectedRange(&self);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(considerTextCheckingForRange:)]
        pub unsafe fn considerTextCheckingForRange(&self, range: NSRange);

        #[cfg(all(
            feature = "AppKit_NSSpellChecker",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method(checkTextInRange:types:options:)]
        pub unsafe fn checkTextInRange_types_options(
            &self,
            range: NSRange,
            checking_types: NSTextCheckingTypes,
            options: &NSDictionary<NSTextCheckingOptionKey, AnyObject>,
        );

        #[method(checkTextInSelection:)]
        pub unsafe fn checkTextInSelection(&self, sender: Option<&AnyObject>);

        #[method(checkTextInDocument:)]
        pub unsafe fn checkTextInDocument(&self, sender: Option<&AnyObject>);

        #[method(orderFrontSubstitutionsPanel:)]
        pub unsafe fn orderFrontSubstitutionsPanel(&self, sender: Option<&AnyObject>);

        #[method(checkSpelling:)]
        pub unsafe fn checkSpelling(&self, sender: Option<&AnyObject>);

        #[method(showGuessPanel:)]
        pub unsafe fn showGuessPanel(&self, sender: Option<&AnyObject>);

        #[method(changeSpelling:)]
        pub unsafe fn changeSpelling(&self, sender: Option<&AnyObject>);

        #[method(ignoreSpelling:)]
        pub unsafe fn ignoreSpelling(&self, sender: Option<&AnyObject>);

        #[method(updateCandidates)]
        pub unsafe fn updateCandidates(&self);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other validAnnotations)]
        pub unsafe fn validAnnotations(&self) -> Id<NSArray<NSAttributedStringKey>>;

        #[cfg(all(feature = "AppKit_NSMenu", feature = "Foundation_NSRange"))]
        #[method_id(@__retain_semantics Other menuAtIndex:clickedOnSelection:effectiveRange:)]
        pub unsafe fn menuAtIndex_clickedOnSelection_effectiveRange(
            &self,
            location: NSUInteger,
            clicked_on_selection: bool,
            effective_range: NSRangePointer,
            mtm: MainThreadMarker,
        ) -> Option<Id<NSMenu>>;

        #[method(spellCheckerDocumentTag)]
        pub unsafe fn spellCheckerDocumentTag(&self) -> NSInteger;

        #[method(setSpellCheckerDocumentTag:)]
        pub unsafe fn setSpellCheckerDocumentTag(&self, spell_checker_document_tag: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextCheckingController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
