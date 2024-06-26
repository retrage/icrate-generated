//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFileProviderItemDecorationIdentifier = NSString;

extern_protocol!(
    #[cfg(feature = "NSFileProviderItem")]
    pub unsafe trait NSFileProviderItemDecorating: NSFileProviderItemProtocol {
        #[method_id(@__retain_semantics Other decorations)]
        unsafe fn decorations(&self)
            -> Option<Id<NSArray<NSFileProviderItemDecorationIdentifier>>>;
    }

    #[cfg(feature = "NSFileProviderItem")]
    unsafe impl ProtocolType for dyn NSFileProviderItemDecorating {}
);
