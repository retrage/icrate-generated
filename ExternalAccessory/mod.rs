//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[cfg_attr(
    feature = "apple",
    link(name = "ExternalAccessory", kind = "framework")
)]
extern "C" {}
#[path = "EAAccessory.rs"]
mod __EAAccessory;
#[path = "EAAccessoryManager.rs"]
mod __EAAccessoryManager;
#[path = "EASession.rs"]
mod __EASession;
#[path = "EAWiFiUnconfiguredAccessory.rs"]
mod __EAWiFiUnconfiguredAccessory;
#[path = "EAWiFiUnconfiguredAccessoryBrowser.rs"]
mod __EAWiFiUnconfiguredAccessoryBrowser;
#[path = "ExternalAccessoryDefines.rs"]
mod __ExternalAccessoryDefines;

#[cfg(feature = "ExternalAccessory_EAAccessory")]
pub use self::__EAAccessory::EAAccessory;
pub use self::__EAAccessory::EAAccessoryDelegate;
pub use self::__EAAccessory::EAConnectionIDNone;
pub use self::__EAAccessoryManager::EAAccessoryDidConnectNotification;
pub use self::__EAAccessoryManager::EAAccessoryDidDisconnectNotification;
pub use self::__EAAccessoryManager::EAAccessoryKey;
#[cfg(feature = "ExternalAccessory_EAAccessoryManager")]
pub use self::__EAAccessoryManager::EAAccessoryManager;
pub use self::__EAAccessoryManager::EAAccessorySelectedKey;
pub use self::__EAAccessoryManager::EABluetoothAccessoryPickerCompletion;
pub use self::__EAAccessoryManager::EABluetoothAccessoryPickerErrorDomain;
pub use self::__EAAccessoryManager::{
    EABluetoothAccessoryPickerAlreadyConnected, EABluetoothAccessoryPickerErrorCode,
    EABluetoothAccessoryPickerResultCancelled, EABluetoothAccessoryPickerResultFailed,
    EABluetoothAccessoryPickerResultNotFound,
};
#[cfg(feature = "ExternalAccessory_EASession")]
pub use self::__EASession::EASession;
#[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory")]
pub use self::__EAWiFiUnconfiguredAccessory::EAWiFiUnconfiguredAccessory;
pub use self::__EAWiFiUnconfiguredAccessory::{
    EAWiFiUnconfiguredAccessoryProperties, EAWiFiUnconfiguredAccessoryPropertySupportsAirPlay,
    EAWiFiUnconfiguredAccessoryPropertySupportsAirPrint,
    EAWiFiUnconfiguredAccessoryPropertySupportsHomeKit,
};
#[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::EAWiFiUnconfiguredAccessoryBrowser;
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::EAWiFiUnconfiguredAccessoryBrowserDelegate;
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::{
    EAWiFiUnconfiguredAccessoryBrowserState, EAWiFiUnconfiguredAccessoryBrowserStateConfiguring,
    EAWiFiUnconfiguredAccessoryBrowserStateSearching,
    EAWiFiUnconfiguredAccessoryBrowserStateStopped,
    EAWiFiUnconfiguredAccessoryBrowserStateWiFiUnavailable,
};
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::{
    EAWiFiUnconfiguredAccessoryConfigurationStatus,
    EAWiFiUnconfiguredAccessoryConfigurationStatusFailed,
    EAWiFiUnconfiguredAccessoryConfigurationStatusSuccess,
    EAWiFiUnconfiguredAccessoryConfigurationStatusUserCancelledConfiguration,
};
