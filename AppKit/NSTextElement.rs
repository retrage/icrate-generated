//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextElement;

    unsafe impl ClassType for NSTextElement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSTextElement {}

extern_methods!(
    unsafe impl NSTextElement {
        #[cfg(feature = "NSTextContentManager")]
        #[method_id(@__retain_semantics Init initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            this: Allocated<Self>,
            text_content_manager: Option<&NSTextContentManager>,
        ) -> Id<Self>;

        #[cfg(feature = "NSTextContentManager")]
        #[method_id(@__retain_semantics Other textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Id<NSTextContentManager>>;

        #[cfg(feature = "NSTextContentManager")]
        #[method(setTextContentManager:)]
        pub unsafe fn setTextContentManager(
            &self,
            text_content_manager: Option<&NSTextContentManager>,
        );

        #[cfg(feature = "NSTextRange")]
        #[method_id(@__retain_semantics Other elementRange)]
        pub unsafe fn elementRange(&self) -> Option<Id<NSTextRange>>;

        #[cfg(feature = "NSTextRange")]
        #[method(setElementRange:)]
        pub unsafe fn setElementRange(&self, element_range: Option<&NSTextRange>);

        #[method_id(@__retain_semantics Other childElements)]
        pub unsafe fn childElements(&self) -> Id<NSArray<NSTextElement>>;

        #[method_id(@__retain_semantics Other parentElement)]
        pub unsafe fn parentElement(&self) -> Option<Id<NSTextElement>>;

        #[method(isRepresentedElement)]
        pub unsafe fn isRepresentedElement(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextParagraph;

    unsafe impl ClassType for NSTextParagraph {
        #[inherits(NSObject)]
        type Super = NSTextElement;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSTextParagraph {}

extern_methods!(
    unsafe impl NSTextParagraph {
        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Allocated<Self>,
            attributed_string: Option<&NSAttributedString>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "NSTextRange")]
        #[method_id(@__retain_semantics Other paragraphContentRange)]
        pub unsafe fn paragraphContentRange(&self) -> Option<Id<NSTextRange>>;

        #[cfg(feature = "NSTextRange")]
        #[method_id(@__retain_semantics Other paragraphSeparatorRange)]
        pub unsafe fn paragraphSeparatorRange(&self) -> Option<Id<NSTextRange>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextElement`
    unsafe impl NSTextParagraph {
        #[cfg(feature = "NSTextContentManager")]
        #[method_id(@__retain_semantics Init initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            this: Allocated<Self>,
            text_content_manager: Option<&NSTextContentManager>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextParagraph {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
