//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSTouchBarItem")]
    pub struct NSCandidateListTouchBarItem<CandidateType: ?Sized = AnyObject> {
        __superclass: NSTouchBarItem,
        _inner0: PhantomData<*mut CandidateType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl<CandidateType: ?Sized + Message> ClassType
        for NSCandidateListTouchBarItem<CandidateType>
    {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = MainThreadOnly;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "NSTouchBarItem")]
unsafe impl<CandidateType: ?Sized + NSCoding> NSCoding
    for NSCandidateListTouchBarItem<CandidateType>
{
}

#[cfg(feature = "NSTouchBarItem")]
unsafe impl<CandidateType: ?Sized> NSObjectProtocol for NSCandidateListTouchBarItem<CandidateType> {}

extern_methods!(
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl<CandidateType: Message> NSCandidateListTouchBarItem<CandidateType> {
        #[cfg(all(
            feature = "NSResponder",
            feature = "NSTextInputClient",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<NSView>>;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSTextInputClient",
            feature = "NSView"
        ))]
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&NSView>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSCandidateListTouchBarItemDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSCandidateListTouchBarItemDelegate>>,
        );

        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;

        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);

        #[method(allowsCollapsing)]
        pub unsafe fn allowsCollapsing(&self) -> bool;

        #[method(setAllowsCollapsing:)]
        pub unsafe fn setAllowsCollapsing(&self, allows_collapsing: bool);

        #[method(isCandidateListVisible)]
        pub unsafe fn isCandidateListVisible(&self) -> bool;

        #[method(updateWithInsertionPointVisibility:)]
        pub unsafe fn updateWithInsertionPointVisibility(&self, is_visible: bool);

        #[method(allowsTextInputContextCandidates)]
        pub unsafe fn allowsTextInputContextCandidates(&self) -> bool;

        #[method(setAllowsTextInputContextCandidates:)]
        pub unsafe fn setAllowsTextInputContextCandidates(
            &self,
            allows_text_input_context_candidates: bool,
        );

        #[cfg(feature = "block2")]
        #[method(attributedStringForCandidate)]
        pub unsafe fn attributedStringForCandidate(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CandidateType>, NSInteger) -> NonNull<NSAttributedString>>;

        #[cfg(feature = "block2")]
        #[method(setAttributedStringForCandidate:)]
        pub unsafe fn setAttributedStringForCandidate(
            &self,
            attributed_string_for_candidate: Option<
                &Block<dyn Fn(NonNull<CandidateType>, NSInteger) -> NonNull<NSAttributedString>>,
            >,
        );

        #[method_id(@__retain_semantics Other candidates)]
        pub unsafe fn candidates(&self) -> Id<NSArray<CandidateType>>;

        #[method(setCandidates:forSelectedRange:inString:)]
        pub unsafe fn setCandidates_forSelectedRange_inString(
            &self,
            candidates: &NSArray<CandidateType>,
            selected_range: NSRange,
            original_string: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl<CandidateType: Message> NSCandidateListTouchBarItem<CandidateType> {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl<CandidateType: Message> NSCandidateListTouchBarItem<CandidateType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSCandidateListTouchBarItemDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(feature = "NSTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:beginSelectingCandidateAtIndex:)]
        unsafe fn candidateListTouchBarItem_beginSelectingCandidateAtIndex(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            index: NSInteger,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:changeSelectionFromCandidateAtIndex:toIndex:)]
        unsafe fn candidateListTouchBarItem_changeSelectionFromCandidateAtIndex_toIndex(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            previous_index: NSInteger,
            index: NSInteger,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:endSelectingCandidateAtIndex:)]
        unsafe fn candidateListTouchBarItem_endSelectingCandidateAtIndex(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            index: NSInteger,
        );

        #[cfg(feature = "NSTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:changedCandidateListVisibility:)]
        unsafe fn candidateListTouchBarItem_changedCandidateListVisibility(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            is_visible: bool,
        );
    }

    unsafe impl ProtocolType for dyn NSCandidateListTouchBarItemDelegate {}
);

extern_methods!(
    /// NSCandidateListTouchBarItem
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other candidateListTouchBarItem)]
        pub unsafe fn candidateListTouchBarItem(&self) -> Option<Id<NSCandidateListTouchBarItem>>;
    }
);

extern "C" {
    #[cfg(feature = "NSTouchBarItem")]
    pub static NSTouchBarItemIdentifierCandidateList: &'static NSTouchBarItemIdentifier;
}
