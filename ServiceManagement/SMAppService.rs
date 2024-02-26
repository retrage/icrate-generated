//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::ServiceManagement::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum SMAppServiceStatus {
        #[doc(alias = "SMAppServiceStatusNotRegistered")]
        NotRegistered = 0,
        #[doc(alias = "SMAppServiceStatusEnabled")]
        Enabled = 1,
        #[doc(alias = "SMAppServiceStatusRequiresApproval")]
        RequiresApproval = 2,
        #[doc(alias = "SMAppServiceStatusNotFound")]
        NotFound = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SMAppService;

    unsafe impl ClassType for SMAppService {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SMAppService {}

extern_methods!(
    unsafe impl SMAppService {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other loginItemServiceWithIdentifier:)]
        pub unsafe fn loginItemServiceWithIdentifier(identifier: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Other mainAppService)]
        pub unsafe fn mainAppService() -> Id<SMAppService>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other agentServiceWithPlistName:)]
        pub unsafe fn agentServiceWithPlistName(plist_name: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other daemonServiceWithPlistName:)]
        pub unsafe fn daemonServiceWithPlistName(plist_name: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(registerAndReturnError:_)]
        pub unsafe fn registerAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(unregisterAndReturnError:_)]
        pub unsafe fn unregisterAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(unregisterWithCompletionHandler:)]
        pub unsafe fn unregisterWithCompletionHandler(&self, handler: &Block<dyn Fn(*mut NSError)>);

        #[method(status)]
        pub unsafe fn status(&self) -> SMAppServiceStatus;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(statusForLegacyURL:)]
        pub unsafe fn statusForLegacyURL(url: &NSURL) -> SMAppServiceStatus;

        #[method(openSystemSettingsLoginItems)]
        pub unsafe fn openSystemSettingsLoginItems();
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SMAppService {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
