//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSNetServicesErrorCode: &'static NSString);

#[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
extern_static!(NSNetServicesErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSNetServicesError {
        NSNetServicesUnknownError = -72000,
        NSNetServicesCollisionError = -72001,
        NSNetServicesNotFoundError = -72002,
        NSNetServicesActivityInProgress = -72003,
        NSNetServicesBadArgumentError = -72004,
        NSNetServicesCancelledError = -72005,
        NSNetServicesInvalidError = -72006,
        NSNetServicesTimeoutError = -72007,
        NSNetServicesMissingRequiredConfigurationError = -72008,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSNetServiceOptions {
        NSNetServiceNoAutoRename = 1 << 0,
        NSNetServiceListenForConnections = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
    pub struct NSNetService;

    unsafe impl ClassType for NSNetService {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSNetService {}

extern_methods!(
    unsafe impl NSNetService {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Init initWithDomain:type:name:port:)]
        pub unsafe fn initWithDomain_type_name_port(
            this: Allocated<Self>,
            domain: &NSString,
            r#type: &NSString,
            name: &NSString,
            port: c_int,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Init initWithDomain:type:name:)]
        pub unsafe fn initWithDomain_type_name(
            this: Allocated<Self>,
            domain: &NSString,
            r#type: &NSString,
            name: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSRunLoop",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(all(
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSRunLoop",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSNetServiceDelegate>>>;

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSNetServiceDelegate>>,
        );

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includes_peer_to_peer: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other hostName)]
        pub unsafe fn hostName(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other addresses)]
        pub unsafe fn addresses(&self) -> Option<Id<NSArray<NSData>>>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(publish)]
        pub unsafe fn publish(&self);

        #[method(publishWithOptions:)]
        pub unsafe fn publishWithOptions(&self, options: NSNetServiceOptions);

        #[deprecated = "Not supported"]
        #[method(resolve)]
        pub unsafe fn resolve(&self);

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(stop)]
        pub unsafe fn stop(&self);

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other dictionaryFromTXTRecordData:)]
        pub unsafe fn dictionaryFromTXTRecordData(
            txt_data: &NSData,
        ) -> Id<NSDictionary<NSString, NSData>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other dataFromTXTRecordDictionary:)]
        pub unsafe fn dataFromTXTRecordDictionary(
            txt_dictionary: &NSDictionary<NSString, NSData>,
        ) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(resolveWithTimeout:)]
        pub unsafe fn resolveWithTimeout(&self, timeout: NSTimeInterval);

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(setTXTRecordData:)]
        pub unsafe fn setTXTRecordData(&self, record_data: Option<&NSData>) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method_id(@__retain_semantics Other TXTRecordData)]
        pub unsafe fn TXTRecordData(&self) -> Option<Id<NSData>>;

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(startMonitoring)]
        pub unsafe fn startMonitoring(&self);

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(stopMonitoring)]
        pub unsafe fn stopMonitoring(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNetService {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use nw_browser_t in Network framework instead"]
    pub struct NSNetServiceBrowser;

    unsafe impl ClassType for NSNetServiceBrowser {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSNetServiceBrowser {}

extern_methods!(
    unsafe impl NSNetServiceBrowser {
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSNetServiceBrowserDelegate>>>;

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSNetServiceBrowserDelegate>>,
        );

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includes_peer_to_peer: bool);

        #[cfg(all(
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSRunLoop",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(all(
            feature = "Foundation_NSObjCRuntime",
            feature = "Foundation_NSRunLoop",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(searchForBrowsableDomains)]
        pub unsafe fn searchForBrowsableDomains(&self);

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(searchForRegistrationDomains)]
        pub unsafe fn searchForRegistrationDomains(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(searchForServicesOfType:inDomain:)]
        pub unsafe fn searchForServicesOfType_inDomain(
            &self,
            r#type: &NSString,
            domain_string: &NSString,
        );

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(stop)]
        pub unsafe fn stop(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNetServiceBrowser {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSNetServiceDelegate: NSObjectProtocol {
        #[optional]
        #[method(netServiceWillPublish:)]
        unsafe fn netServiceWillPublish(&self, sender: &NSNetService);

        #[optional]
        #[method(netServiceDidPublish:)]
        unsafe fn netServiceDidPublish(&self, sender: &NSNetService);

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSValue"
        ))]
        #[optional]
        #[method(netService:didNotPublish:)]
        unsafe fn netService_didNotPublish(
            &self,
            sender: &NSNetService,
            error_dict: &NSDictionary<NSString, NSNumber>,
        );

        #[optional]
        #[method(netServiceWillResolve:)]
        unsafe fn netServiceWillResolve(&self, sender: &NSNetService);

        #[optional]
        #[method(netServiceDidResolveAddress:)]
        unsafe fn netServiceDidResolveAddress(&self, sender: &NSNetService);

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSValue"
        ))]
        #[optional]
        #[method(netService:didNotResolve:)]
        unsafe fn netService_didNotResolve(
            &self,
            sender: &NSNetService,
            error_dict: &NSDictionary<NSString, NSNumber>,
        );

        #[optional]
        #[method(netServiceDidStop:)]
        unsafe fn netServiceDidStop(&self, sender: &NSNetService);

        #[cfg(feature = "Foundation_NSData")]
        #[optional]
        #[method(netService:didUpdateTXTRecordData:)]
        unsafe fn netService_didUpdateTXTRecordData(&self, sender: &NSNetService, data: &NSData);

        #[cfg(feature = "Foundation_NSStream")]
        #[optional]
        #[method(netService:didAcceptConnectionWithInputStream:outputStream:)]
        unsafe fn netService_didAcceptConnectionWithInputStream_outputStream(
            &self,
            sender: &NSNetService,
            input_stream: &NSInputStream,
            output_stream: &NSOutputStream,
        );
    }

    unsafe impl ProtocolType for dyn NSNetServiceDelegate {}
);

extern_protocol!(
    pub unsafe trait NSNetServiceBrowserDelegate: NSObjectProtocol {
        #[optional]
        #[method(netServiceBrowserWillSearch:)]
        unsafe fn netServiceBrowserWillSearch(&self, browser: &NSNetServiceBrowser);

        #[optional]
        #[method(netServiceBrowserDidStopSearch:)]
        unsafe fn netServiceBrowserDidStopSearch(&self, browser: &NSNetServiceBrowser);

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSValue"
        ))]
        #[optional]
        #[method(netServiceBrowser:didNotSearch:)]
        unsafe fn netServiceBrowser_didNotSearch(
            &self,
            browser: &NSNetServiceBrowser,
            error_dict: &NSDictionary<NSString, NSNumber>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(netServiceBrowser:didFindDomain:moreComing:)]
        unsafe fn netServiceBrowser_didFindDomain_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            domain_string: &NSString,
            more_coming: bool,
        );

        #[optional]
        #[method(netServiceBrowser:didFindService:moreComing:)]
        unsafe fn netServiceBrowser_didFindService_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            service: &NSNetService,
            more_coming: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method(netServiceBrowser:didRemoveDomain:moreComing:)]
        unsafe fn netServiceBrowser_didRemoveDomain_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            domain_string: &NSString,
            more_coming: bool,
        );

        #[optional]
        #[method(netServiceBrowser:didRemoveService:moreComing:)]
        unsafe fn netServiceBrowser_didRemoveService_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            service: &NSNetService,
            more_coming: bool,
        );
    }

    unsafe impl ProtocolType for dyn NSNetServiceBrowserDelegate {}
);
