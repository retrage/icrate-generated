//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTableColumnResizingOptions(pub NSUInteger);
impl NSTableColumnResizingOptions {
    pub const NSTableColumnNoResizing: Self = Self(0);
    pub const NSTableColumnAutoresizingMask: Self = Self(1 << 0);
    pub const NSTableColumnUserResizingMask: Self = Self(1 << 1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSTableColumnResizingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSTableColumnResizingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableColumn;

    unsafe impl ClassType for NSTableColumn {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSTableColumn {}

unsafe impl NSObjectProtocol for NSTableColumn {}

#[cfg(feature = "AppKit_NSUserInterfaceItemIdentification")]
unsafe impl NSUserInterfaceItemIdentification for NSTableColumn {}

extern_methods!(
    unsafe impl NSTableColumn {
        #[cfg(all(
            feature = "AppKit_NSUserInterfaceItemIdentification",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSUserInterfaceItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceItemIdentification",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUserInterfaceItemIdentifier>;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceItemIdentification",
            feature = "Foundation_NSString"
        ))]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Other tableView)]
        pub unsafe fn tableView(&self, mtm: MainThreadMarker) -> Option<Id<NSTableView>>;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, table_view: Option<&NSTableView>);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(width)]
        pub unsafe fn width(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(minWidth)]
        pub unsafe fn minWidth(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setMinWidth:)]
        pub unsafe fn setMinWidth(&self, min_width: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(maxWidth)]
        pub unsafe fn maxWidth(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setMaxWidth:)]
        pub unsafe fn setMaxWidth(&self, max_width: CGFloat);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTableHeaderCell",
            feature = "AppKit_NSTextFieldCell"
        ))]
        #[method_id(@__retain_semantics Other headerCell)]
        pub unsafe fn headerCell(&self, mtm: MainThreadMarker) -> Id<NSTableHeaderCell>;

        #[cfg(all(
            feature = "AppKit_NSActionCell",
            feature = "AppKit_NSCell",
            feature = "AppKit_NSTableHeaderCell",
            feature = "AppKit_NSTextFieldCell"
        ))]
        #[method(setHeaderCell:)]
        pub unsafe fn setHeaderCell(&self, header_cell: &NSTableHeaderCell);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[cfg(feature = "Foundation_NSSortDescriptor")]
        #[method_id(@__retain_semantics Other sortDescriptorPrototype)]
        pub unsafe fn sortDescriptorPrototype(&self) -> Option<Id<NSSortDescriptor>>;

        #[cfg(feature = "Foundation_NSSortDescriptor")]
        #[method(setSortDescriptorPrototype:)]
        pub unsafe fn setSortDescriptorPrototype(
            &self,
            sort_descriptor_prototype: Option<&NSSortDescriptor>,
        );

        #[method(resizingMask)]
        pub unsafe fn resizingMask(&self) -> NSTableColumnResizingOptions;

        #[method(setResizingMask:)]
        pub unsafe fn setResizingMask(&self, resizing_mask: NSTableColumnResizingOptions);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other headerToolTip)]
        pub unsafe fn headerToolTip(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHeaderToolTip:)]
        pub unsafe fn setHeaderToolTip(&self, header_tool_tip: Option<&NSString>);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTableColumn {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSTableColumn {
        #[deprecated]
        #[method(setResizable:)]
        pub unsafe fn setResizable(&self, flag: bool);

        #[deprecated]
        #[method(isResizable)]
        pub unsafe fn isResizable(&self) -> bool;

        #[method_id(@__retain_semantics Other dataCell)]
        pub unsafe fn dataCell(&self) -> Id<AnyObject>;

        #[method(setDataCell:)]
        pub unsafe fn setDataCell(&self, data_cell: &AnyObject);

        #[method_id(@__retain_semantics Other dataCellForRow:)]
        pub unsafe fn dataCellForRow(&self, row: NSInteger) -> Id<AnyObject>;
    }
);
