// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `ExtensionKit` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "ExtensionKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "ExtensionKit_EXAppExtensionBrowserViewController")]
#[path = "EXAppExtensionBrowserViewController.rs"]
mod __EXAppExtensionBrowserViewController;
#[cfg(feature = "ExtensionKit_EXHostViewController")]
#[path = "EXHostViewController.rs"]
mod __EXHostViewController;

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "ExtensionKit_EXAppExtensionBrowserViewController"
))]
pub use self::__EXAppExtensionBrowserViewController::EXAppExtensionBrowserViewController;
#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "ExtensionKit_EXHostViewController"
))]
pub use self::__EXHostViewController::EXHostViewController;
#[cfg(feature = "ExtensionKit_EXHostViewController")]
pub use self::__EXHostViewController::EXHostViewControllerDelegate;
