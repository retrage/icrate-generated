//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKGameCenterViewControllerState {
        #[doc(alias = "GKGameCenterViewControllerStateDefault")]
        Default = -1,
        #[doc(alias = "GKGameCenterViewControllerStateLeaderboards")]
        Leaderboards = 0,
        #[doc(alias = "GKGameCenterViewControllerStateAchievements")]
        Achievements = 1,
        #[doc(alias = "GKGameCenterViewControllerStateChallenges")]
        Challenges = 2,
        #[doc(alias = "GKGameCenterViewControllerStateLocalPlayerProfile")]
        LocalPlayerProfile = 3,
        #[doc(alias = "GKGameCenterViewControllerStateDashboard")]
        Dashboard = 4,
        #[doc(alias = "GKGameCenterViewControllerStateLocalPlayerFriendsList")]
        LocalPlayerFriendsList = 5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    pub struct GKGameCenterViewController;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ClassType for GKGameCenterViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "GameKit_GKDialogController"
))]
unsafe impl GKViewController for GKGameCenterViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for GKGameCenterViewController {}

#[cfg(all(
    feature = "AppKit_NSKeyValueBinding",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSEditor for GKGameCenterViewController {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
unsafe impl NSObjectProtocol for GKGameCenterViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSStoryboardSegue",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSSeguePerforming for GKGameCenterViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for GKGameCenterViewController {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl GKGameCenterViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl GKGameCenterViewController {
        #[cfg(all(
            feature = "AppKit_NSNib",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics Other gameCenterDelegate)]
        pub unsafe fn gameCenterDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKGameCenterControllerDelegate>>>;

        #[method(setGameCenterDelegate:)]
        pub unsafe fn setGameCenterDelegate(
            &self,
            game_center_delegate: Option<&ProtocolObject<dyn GKGameCenterControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Init initWithState:)]
        pub unsafe fn initWithState(
            this: Allocated<Self>,
            state: GKGameCenterViewControllerState,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKLeaderboard"))]
        #[method_id(@__retain_semantics Init initWithLeaderboardID:playerScope:timeScope:)]
        pub unsafe fn initWithLeaderboardID_playerScope_timeScope(
            this: Allocated<Self>,
            leaderboard_id: &NSString,
            player_scope: GKLeaderboardPlayerScope,
            time_scope: GKLeaderboardTimeScope,
        ) -> Id<Self>;

        #[cfg(feature = "GameKit_GKLeaderboard")]
        #[method_id(@__retain_semantics Init initWithLeaderboard:playerScope:)]
        pub unsafe fn initWithLeaderboard_playerScope(
            this: Allocated<Self>,
            leaderboard: &GKLeaderboard,
            player_scope: GKLeaderboardPlayerScope,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAchievementID:)]
        pub unsafe fn initWithAchievementID(
            this: Allocated<Self>,
            achievement_id: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl GKGameCenterViewController {
        #[deprecated]
        #[method(viewState)]
        pub unsafe fn viewState(&self) -> GKGameCenterViewControllerState;

        #[deprecated]
        #[method(setViewState:)]
        pub unsafe fn setViewState(&self, view_state: GKGameCenterViewControllerState);

        #[cfg(feature = "GameKit_GKLeaderboard")]
        #[deprecated]
        #[method(leaderboardTimeScope)]
        pub unsafe fn leaderboardTimeScope(&self) -> GKLeaderboardTimeScope;

        #[cfg(feature = "GameKit_GKLeaderboard")]
        #[deprecated]
        #[method(setLeaderboardTimeScope:)]
        pub unsafe fn setLeaderboardTimeScope(
            &self,
            leaderboard_time_scope: GKLeaderboardTimeScope,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other leaderboardIdentifier)]
        pub unsafe fn leaderboardIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setLeaderboardIdentifier:)]
        pub unsafe fn setLeaderboardIdentifier(&self, leaderboard_identifier: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other leaderboardCategory)]
        pub unsafe fn leaderboardCategory(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setLeaderboardCategory:)]
        pub unsafe fn setLeaderboardCategory(&self, leaderboard_category: Option<&NSString>);
    }
);

extern_protocol!(
    pub unsafe trait GKGameCenterControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
        #[method(gameCenterViewControllerDidFinish:)]
        unsafe fn gameCenterViewControllerDidFinish(
            &self,
            game_center_view_controller: &GKGameCenterViewController,
        );
    }

    unsafe impl ProtocolType for dyn GKGameCenterControllerDelegate {}
);
