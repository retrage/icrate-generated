//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerControllerQueue;

    unsafe impl ClassType for MPMusicPlayerControllerQueue {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPMusicPlayerControllerQueue {}

extern_methods!(
    unsafe impl MPMusicPlayerControllerQueue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItem"
        ))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Id<NSArray<MPMediaItem>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerControllerMutableQueue;

    unsafe impl ClassType for MPMusicPlayerControllerMutableQueue {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerControllerQueue;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPMusicPlayerControllerMutableQueue {}

extern_methods!(
    unsafe impl MPMusicPlayerControllerMutableQueue {
        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItem",
            feature = "MediaPlayer_MPMusicPlayerQueueDescriptor"
        ))]
        #[method(insertQueueDescriptor:afterItem:)]
        pub unsafe fn insertQueueDescriptor_afterItem(
            &self,
            queue_descriptor: &MPMusicPlayerQueueDescriptor,
            after_item: Option<&MPMediaItem>,
        );

        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItem"
        ))]
        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &MPMediaItem);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMusicPlayerControllerQueue`
    unsafe impl MPMusicPlayerControllerMutableQueue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
    pub struct MPMusicPlayerApplicationController;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
    unsafe impl ClassType for MPMusicPlayerApplicationController {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "MediaPlayer_MPMediaPlayback",
    feature = "MediaPlayer_MPMusicPlayerController"
))]
unsafe impl MPMediaPlayback for MPMusicPlayerApplicationController {}

#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
unsafe impl NSObjectProtocol for MPMusicPlayerApplicationController {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
    unsafe impl MPMusicPlayerApplicationController {
        #[cfg(feature = "Foundation_NSError")]
        #[method(performQueueTransaction:completionHandler:)]
        pub unsafe fn performQueueTransaction_completionHandler(
            &self,
            queue_transaction: &Block<dyn Fn(NonNull<MPMusicPlayerControllerMutableQueue>)>,
            completion_handler: &Block<dyn Fn(NonNull<MPMusicPlayerControllerQueue>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMusicPlayerController`
    #[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
    unsafe impl MPMusicPlayerApplicationController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(MPMusicPlayerControllerQueueDidChangeNotification: &'static NSString);
