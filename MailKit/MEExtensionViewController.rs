//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    pub struct MEExtensionViewController;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ClassType for MEExtensionViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for MEExtensionViewController {}

#[cfg(all(
    feature = "AppKit_NSKeyValueBinding",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSEditor for MEExtensionViewController {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
unsafe impl NSObjectProtocol for MEExtensionViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSStoryboardSegue",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSSeguePerforming for MEExtensionViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for MEExtensionViewController {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl MEExtensionViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl MEExtensionViewController {
        #[cfg(all(
            feature = "AppKit_NSNib",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl MEExtensionViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl MEExtensionViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
