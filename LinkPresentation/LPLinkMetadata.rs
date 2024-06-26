//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LPLinkMetadata;

    unsafe impl ClassType for LPLinkMetadata {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for LPLinkMetadata {}

unsafe impl NSCopying for LPLinkMetadata {}

unsafe impl NSObjectProtocol for LPLinkMetadata {}

unsafe impl NSSecureCoding for LPLinkMetadata {}

extern_methods!(
    unsafe impl LPLinkMetadata {
        #[method_id(@__retain_semantics Other originalURL)]
        pub unsafe fn originalURL(&self) -> Option<Id<NSURL>>;

        #[method(setOriginalURL:)]
        pub unsafe fn setOriginalURL(&self, original_url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other iconProvider)]
        pub unsafe fn iconProvider(&self) -> Option<Id<NSItemProvider>>;

        #[method(setIconProvider:)]
        pub unsafe fn setIconProvider(&self, icon_provider: Option<&NSItemProvider>);

        #[method_id(@__retain_semantics Other imageProvider)]
        pub unsafe fn imageProvider(&self) -> Option<Id<NSItemProvider>>;

        #[method(setImageProvider:)]
        pub unsafe fn setImageProvider(&self, image_provider: Option<&NSItemProvider>);

        #[method_id(@__retain_semantics Other videoProvider)]
        pub unsafe fn videoProvider(&self) -> Option<Id<NSItemProvider>>;

        #[method(setVideoProvider:)]
        pub unsafe fn setVideoProvider(&self, video_provider: Option<&NSItemProvider>);

        #[method_id(@__retain_semantics Other remoteVideoURL)]
        pub unsafe fn remoteVideoURL(&self) -> Option<Id<NSURL>>;

        #[method(setRemoteVideoURL:)]
        pub unsafe fn setRemoteVideoURL(&self, remote_video_url: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LPLinkMetadata {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
