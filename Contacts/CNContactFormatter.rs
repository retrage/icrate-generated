//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNContactFormatterStyle {
        #[doc(alias = "CNContactFormatterStyleFullName")]
        FullName = 0,
        #[doc(alias = "CNContactFormatterStylePhoneticFullName")]
        PhoneticFullName = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNContactDisplayNameOrder {
        #[doc(alias = "CNContactDisplayNameOrderUserDefault")]
        UserDefault = 0,
        #[doc(alias = "CNContactDisplayNameOrderGivenNameFirst")]
        GivenNameFirst = 1,
        #[doc(alias = "CNContactDisplayNameOrderFamilyNameFirst")]
        FamilyNameFirst = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFormatter")]
    pub struct CNContactFormatter;

    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl ClassType for CNContactFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for CNContactFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSCopying for CNContactFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl NSObjectProtocol for CNContactFormatter {}

#[cfg(all(feature = "Foundation_NSFormatter", feature = "Foundation_NSObject"))]
unsafe impl NSSecureCoding for CNContactFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl CNContactFormatter {
        #[cfg(all(feature = "Contacts_CNContact", feature = "Foundation_NSObject"))]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeysForStyle:)]
        pub unsafe fn descriptorForRequiredKeysForStyle(
            style: CNContactFormatterStyle,
        ) -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(all(feature = "Contacts_CNContact", feature = "Foundation_NSObject"))]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeysForNameOrder)]
        pub unsafe fn descriptorForRequiredKeysForNameOrder(
        ) -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(all(feature = "Contacts_CNContact", feature = "Foundation_NSObject"))]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeysForDelimiter)]
        pub unsafe fn descriptorForRequiredKeysForDelimiter(
        ) -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(all(feature = "Contacts_CNContact", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromContact:style:)]
        pub unsafe fn stringFromContact_style(
            contact: &CNContact,
            style: CNContactFormatterStyle,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other attributedStringFromContact:style:defaultAttributes:)]
        pub unsafe fn attributedStringFromContact_style_defaultAttributes(
            contact: &CNContact,
            style: CNContactFormatterStyle,
            attributes: Option<&NSDictionary>,
        ) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Contacts_CNContact")]
        #[method(nameOrderForContact:)]
        pub unsafe fn nameOrderForContact(contact: &CNContact) -> CNContactDisplayNameOrder;

        #[cfg(all(feature = "Contacts_CNContact", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other delimiterForContact:)]
        pub unsafe fn delimiterForContact(contact: &CNContact) -> Id<NSString>;

        #[method(style)]
        pub unsafe fn style(&self) -> CNContactFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: CNContactFormatterStyle);

        #[cfg(all(feature = "Contacts_CNContact", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromContact:)]
        pub unsafe fn stringFromContact(&self, contact: &CNContact) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "Contacts_CNContact",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other attributedStringFromContact:defaultAttributes:)]
        pub unsafe fn attributedStringFromContact_defaultAttributes(
            &self,
            contact: &CNContact,
            attributes: Option<&NSDictionary>,
        ) -> Option<Id<NSAttributedString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl CNContactFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(CNContactPropertyAttribute: &'static NSString);
