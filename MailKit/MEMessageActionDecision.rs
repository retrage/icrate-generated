//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEMessageActionDecision")]
    pub struct MEMessageActionDecision;

    #[cfg(feature = "MailKit_MEMessageActionDecision")]
    unsafe impl ClassType for MEMessageActionDecision {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEMessageActionDecision")]
unsafe impl NSCoding for MEMessageActionDecision {}

#[cfg(feature = "MailKit_MEMessageActionDecision")]
unsafe impl NSObjectProtocol for MEMessageActionDecision {}

#[cfg(feature = "MailKit_MEMessageActionDecision")]
unsafe impl NSSecureCoding for MEMessageActionDecision {}

extern_methods!(
    #[cfg(feature = "MailKit_MEMessageActionDecision")]
    unsafe impl MEMessageActionDecision {
        #[method_id(@__retain_semantics Other invokeAgainWithBody)]
        pub unsafe fn invokeAgainWithBody() -> Id<MEMessageActionDecision, Shared>;

        #[cfg(feature = "MailKit_MEMessageAction")]
        #[method_id(@__retain_semantics Other decisionApplyingAction:)]
        pub unsafe fn decisionApplyingAction(action: &MEMessageAction) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MailKit_MEMessageAction"))]
        #[method_id(@__retain_semantics Other decisionApplyingActions:)]
        pub unsafe fn decisionApplyingActions(
            actions: &NSArray<MEMessageAction>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
