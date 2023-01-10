//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CARemoteLayerServer;

    unsafe impl ClassType for CARemoteLayerServer {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl CARemoteLayerServer {
        #[method_id(@__retain_semantics Other sharedServer)]
        pub unsafe fn sharedServer() -> Id<CARemoteLayerServer, Shared>;
    }
);

extern_methods!(
    /// CARemoteLayerServer
    unsafe impl CALayer {
        #[method_id(@__retain_semantics Other layerWithRemoteClientId:)]
        pub unsafe fn layerWithRemoteClientId(client_id: u32) -> Id<CALayer, Shared>;
    }
);