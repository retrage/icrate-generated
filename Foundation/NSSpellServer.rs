//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSpellServer;

    unsafe impl ClassType for NSSpellServer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSSpellServer {}

extern_methods!(
    unsafe impl NSSpellServer {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSSpellServerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSpellServerDelegate>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerLanguage:byVendor:)]
        pub unsafe fn registerLanguage_byVendor(
            &self,
            language: Option<&NSString>,
            vendor: Option<&NSString>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isWordInUserDictionaries:caseSensitive:)]
        pub unsafe fn isWordInUserDictionaries_caseSensitive(
            &self,
            word: &NSString,
            flag: bool,
        ) -> bool;

        #[method(run)]
        pub unsafe fn run(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSpellServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSGrammarRange: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSGrammarUserDescription: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSGrammarCorrections: &'static NSString);

extern_protocol!(
    pub unsafe trait NSSpellServerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[optional]
        #[method(spellServer:findMisspelledWordInString:language:wordCount:countOnly:)]
        unsafe fn spellServer_findMisspelledWordInString_language_wordCount_countOnly(
            &self,
            sender: &NSSpellServer,
            string_to_check: &NSString,
            language: &NSString,
            word_count: NonNull<NSInteger>,
            count_only: bool,
        ) -> NSRange;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other spellServer:suggestGuessesForWord:inLanguage:)]
        unsafe fn spellServer_suggestGuessesForWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        ) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(spellServer:didLearnWord:inLanguage:)]
        unsafe fn spellServer_didLearnWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(spellServer:didForgetWord:inLanguage:)]
        unsafe fn spellServer_didForgetWord_inLanguage(
            &self,
            sender: &NSSpellServer,
            word: &NSString,
            language: &NSString,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other spellServer:suggestCompletionsForPartialWordRange:inString:language:)]
        unsafe fn spellServer_suggestCompletionsForPartialWordRange_inString_language(
            &self,
            sender: &NSSpellServer,
            range: NSRange,
            string: &NSString,
            language: &NSString,
        ) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method(spellServer:checkGrammarInString:language:details:)]
        unsafe fn spellServer_checkGrammarInString_language_details(
            &self,
            sender: &NSSpellServer,
            string_to_check: &NSString,
            language: Option<&NSString>,
            details: Option<&mut Option<Id<NSArray<NSDictionary<NSString, AnyObject>>>>>,
        ) -> NSRange;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSOrthography",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other spellServer:checkString:offset:types:options:orthography:wordCount:)]
        unsafe fn spellServer_checkString_offset_types_options_orthography_wordCount(
            &self,
            sender: &NSSpellServer,
            string_to_check: &NSString,
            offset: NSUInteger,
            checking_types: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            orthography: Option<&NSOrthography>,
            word_count: NonNull<NSInteger>,
        ) -> Option<Id<NSArray<NSTextCheckingResult>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(spellServer:recordResponse:toCorrection:forWord:language:)]
        unsafe fn spellServer_recordResponse_toCorrection_forWord_language(
            &self,
            sender: &NSSpellServer,
            response: NSUInteger,
            correction: &NSString,
            word: &NSString,
            language: &NSString,
        );
    }

    unsafe impl ProtocolType for dyn NSSpellServerDelegate {}
);
