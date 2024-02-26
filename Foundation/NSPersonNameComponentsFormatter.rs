//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersonNameComponentsFormatterStyle {
        #[doc(alias = "NSPersonNameComponentsFormatterStyleDefault")]
        Default = 0,
        #[doc(alias = "NSPersonNameComponentsFormatterStyleShort")]
        Short = 1,
        #[doc(alias = "NSPersonNameComponentsFormatterStyleMedium")]
        Medium = 2,
        #[doc(alias = "NSPersonNameComponentsFormatterStyleLong")]
        Long = 3,
        #[doc(alias = "NSPersonNameComponentsFormatterStyleAbbreviated")]
        Abbreviated = 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPersonNameComponentsFormatterOptions {
        NSPersonNameComponentsFormatterPhonetic = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFormatter")]
    pub struct NSPersonNameComponentsFormatter;

    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl ClassType for NSPersonNameComponentsFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl Send for NSPersonNameComponentsFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl Sync for NSPersonNameComponentsFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for NSPersonNameComponentsFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for NSPersonNameComponentsFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl NSObjectProtocol for NSPersonNameComponentsFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSPersonNameComponentsFormatter {
        #[method(style)]
        pub unsafe fn style(&self) -> NSPersonNameComponentsFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSPersonNameComponentsFormatterStyle);

        #[method(isPhonetic)]
        pub unsafe fn isPhonetic(&self) -> bool;

        #[method(setPhonetic:)]
        pub unsafe fn setPhonetic(&self, phonetic: bool);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other localizedStringFromPersonNameComponents:style:options:)]
        pub unsafe fn localizedStringFromPersonNameComponents_style_options(
            components: &NSPersonNameComponents,
            name_format_style: NSPersonNameComponentsFormatterStyle,
            name_options: NSPersonNameComponentsFormatterOptions,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other stringFromPersonNameComponents:)]
        pub unsafe fn stringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSPersonNameComponents"
        ))]
        #[method_id(@__retain_semantics Other annotatedStringFromPersonNameComponents:)]
        pub unsafe fn annotatedStringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSAttributedString>;

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other personNameComponentsFromString:)]
        pub unsafe fn personNameComponentsFromString(
            &self,
            string: &NSString,
        ) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSPersonNameComponentsFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentGivenName: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentFamilyName: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentMiddleName: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentPrefix: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentSuffix: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentNickname: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersonNameComponentDelimiter: &'static NSString);
