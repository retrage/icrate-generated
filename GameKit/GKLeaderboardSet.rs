//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKLeaderboardSet;

    unsafe impl ClassType for GKLeaderboardSet {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for GKLeaderboardSet {}

unsafe impl NSObjectProtocol for GKLeaderboardSet {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for GKLeaderboardSet {}

extern_methods!(
    unsafe impl GKLeaderboardSet {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadLeaderboardSetsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardSetsWithCompletionHandler(
            completion_handler: Option<
                &Block<dyn Fn(*mut NSArray<GKLeaderboardSet>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKLeaderboard"
        ))]
        #[method(loadLeaderboardsWithHandler:)]
        pub unsafe fn loadLeaderboardsWithHandler(
            &self,
            handler: &Block<dyn Fn(*mut NSArray<GKLeaderboard>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKLeaderboardSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// Deprecated
    unsafe impl GKLeaderboardSet {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKLeaderboard"
        ))]
        #[deprecated]
        #[method(loadLeaderboardsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKLeaderboard>, *mut NSError)>>,
        );
    }
);

extern_methods!(
    /// UI
    unsafe impl GKLeaderboardSet {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSImage, *mut NSError)>>,
        );
    }
);
