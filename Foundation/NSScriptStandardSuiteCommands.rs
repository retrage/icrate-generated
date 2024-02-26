//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSaveOptions {
        #[doc(alias = "NSSaveOptionsYes")]
        Yes = 0,
        #[doc(alias = "NSSaveOptionsNo")]
        No = 1,
        #[doc(alias = "NSSaveOptionsAsk")]
        Ask = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSCloneCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSCloneCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSCloneCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCloneCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCloneCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCloneCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCloneCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSCloseCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSCloseCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSCloseCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCloseCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCloseCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCloseCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCloseCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSCountCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSCountCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSCountCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCountCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCountCommand {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCountCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCountCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSCreateCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSCreateCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSCreateCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSCreateCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCreateCommand {
        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription"
        ))]
        #[method_id(@__retain_semantics Other createClassDescription)]
        pub unsafe fn createClassDescription(&self) -> Id<NSScriptClassDescription>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other resolvedKeyDictionary)]
        pub unsafe fn resolvedKeyDictionary(&self) -> Id<NSDictionary<NSString, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCreateCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSCreateCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSDeleteCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSDeleteCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSDeleteCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSDeleteCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSDeleteCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSDeleteCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSDeleteCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSExistsCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSExistsCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSExistsCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSExistsCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSExistsCommand {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSExistsCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSExistsCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSGetCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSGetCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSGetCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSGetCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSGetCommand {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSGetCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSGetCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSMoveCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSMoveCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSMoveCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSMoveCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSMoveCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSMoveCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSMoveCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSQuitCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSQuitCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSQuitCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSQuitCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSQuitCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSQuitCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSQuitCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommand")]
    pub struct NSSetCommand;

    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl ClassType for NSSetCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "Foundation_NSObject",
    feature = "Foundation_NSScriptCommand"
))]
unsafe impl NSCoding for NSSetCommand {}

#[cfg(feature = "Foundation_NSScriptCommand")]
unsafe impl NSObjectProtocol for NSSetCommand {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSSetCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receivers_ref: Option<&NSScriptObjectSpecifier>);

        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSSetCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSScriptCommand")]
    unsafe impl NSSetCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
