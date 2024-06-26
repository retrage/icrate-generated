//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MEMessageDecoder: NSObjectProtocol {
        #[cfg(feature = "MEDecodedMessage")]
        #[method_id(@__retain_semantics Other decodedMessageForMessageData:)]
        unsafe fn decodedMessageForMessageData(
            &self,
            data: &NSData,
        ) -> Option<Id<MEDecodedMessage>>;
    }

    unsafe impl ProtocolType for dyn MEMessageDecoder {}
);
