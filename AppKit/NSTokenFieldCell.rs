//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTokenStyle {
        #[doc(alias = "NSTokenStyleDefault")]
        Default = 0,
        #[doc(alias = "NSTokenStyleNone")]
        None = 1,
        #[doc(alias = "NSTokenStyleRounded")]
        Rounded = 2,
        #[doc(alias = "NSTokenStyleSquared")]
        Squared = 3,
        #[doc(alias = "NSTokenStylePlainSquared")]
        PlainSquared = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSCell",
        feature = "AppKit_NSTextFieldCell"
    ))]
    pub struct NSTokenFieldCell;

    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSCell",
        feature = "AppKit_NSTextFieldCell"
    ))]
    unsafe impl ClassType for NSTokenFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSTextFieldCell"
))]
unsafe impl NSAccessibility for NSTokenFieldCell {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSTextFieldCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSTokenFieldCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSTextFieldCell",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSTokenFieldCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSTextFieldCell",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCopying for NSTokenFieldCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSTextFieldCell"
))]
unsafe impl NSObjectProtocol for NSTokenFieldCell {}

#[cfg(all(
    feature = "AppKit_NSActionCell",
    feature = "AppKit_NSCell",
    feature = "AppKit_NSTextFieldCell",
    feature = "AppKit_NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTokenFieldCell {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSCell",
        feature = "AppKit_NSTextFieldCell"
    ))]
    unsafe impl NSTokenFieldCell {
        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, token_style: NSTokenStyle);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completion_delay: NSTimeInterval);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay(mtm: MainThreadMarker) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Id<NSCharacterSet>;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizing_character_set: Option<&NSCharacterSet>,
        );

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet(mtm: MainThreadMarker) -> Id<NSCharacterSet>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSTokenFieldCellDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTokenFieldCellDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSCell",
        feature = "AppKit_NSTextFieldCell"
    ))]
    unsafe impl NSTokenFieldCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(this: Allocated<Self>, image: Option<&NSImage>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSCell",
        feature = "AppKit_NSTextFieldCell"
    ))]
    unsafe impl NSTokenFieldCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSActionCell",
        feature = "AppKit_NSCell",
        feature = "AppKit_NSTextFieldCell"
    ))]
    unsafe impl NSTokenFieldCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSTokenFieldCellDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTextFieldCell",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
        unsafe fn tokenFieldCell_completionsForSubstring_indexOfToken_indexOfSelectedItem(
            &self,
            token_field_cell: &NSTokenFieldCell,
            substring: &NSString,
            token_index: NSInteger,
            selected_index: NonNull<NSInteger>,
        ) -> Id<NSArray>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTextFieldCell",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:shouldAddObjects:atIndex:)]
        unsafe fn tokenFieldCell_shouldAddObjects_atIndex(
            &self,
            token_field_cell: &NSTokenFieldCell,
            tokens: &NSArray,
            index: NSUInteger,
        ) -> Id<NSArray>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTextFieldCell",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:displayStringForRepresentedObject:)]
        unsafe fn tokenFieldCell_displayStringForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTextFieldCell",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:editingStringForRepresentedObject:)]
        unsafe fn tokenFieldCell_editingStringForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTextFieldCell",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:representedObjectForEditingString:)]
        unsafe fn tokenFieldCell_representedObjectForEditingString(
            &self,
            token_field_cell: &NSTokenFieldCell,
            editing_string: &NSString,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSTextFieldCell",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(tokenFieldCell:writeRepresentedObjects:toPasteboard:)]
        unsafe fn tokenFieldCell_writeRepresentedObjects_toPasteboard(
            &self,
            token_field_cell: &NSTokenFieldCell,
            objects: &NSArray,
            pboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSTextFieldCell",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:readFromPasteboard:)]
        unsafe fn tokenFieldCell_readFromPasteboard(
            &self,
            token_field_cell: &NSTokenFieldCell,
            pboard: &NSPasteboard,
        ) -> Option<Id<NSArray>>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSMenu",
            feature = "AppKit_NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:menuForRepresentedObject:)]
        unsafe fn tokenFieldCell_menuForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> Option<Id<NSMenu>>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTextFieldCell"
        ))]
        #[optional]
        #[method(tokenFieldCell:hasMenuForRepresentedObject:)]
        unsafe fn tokenFieldCell_hasMenuForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTextFieldCell"
        ))]
        #[optional]
        #[method(tokenFieldCell:styleForRepresentedObject:)]
        unsafe fn tokenFieldCell_styleForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> NSTokenStyle;
    }

    unsafe impl ProtocolType for dyn NSTokenFieldCellDelegate {}
);

extern_static!(NSDefaultTokenStyle: NSTokenStyle = NSTokenStyle(NSTokenStyle::Default.0));

extern_static!(NSPlainTextTokenStyle: NSTokenStyle = NSTokenStyle(NSTokenStyle::None.0));

extern_static!(NSRoundedTokenStyle: NSTokenStyle = NSTokenStyle(NSTokenStyle::Rounded.0));
