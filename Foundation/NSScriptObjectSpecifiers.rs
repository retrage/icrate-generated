//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub const NSNoSpecifierError: NSInteger = 0;
pub const NSNoTopLevelContainersSpecifierError: NSInteger = 1;
pub const NSContainerSpecifierError: NSInteger = 2;
pub const NSUnknownKeySpecifierError: NSInteger = 3;
pub const NSInvalidIndexSpecifierError: NSInteger = 4;
pub const NSInternalSpecifierError: NSInteger = 5;
pub const NSOperationNotSupportedForKeySpecifierError: NSInteger = 6;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSInsertionPosition {
        NSPositionAfter = 0,
        NSPositionBefore = 1,
        NSPositionBeginning = 2,
        NSPositionEnd = 3,
        NSPositionReplace = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRelativePosition {
        NSRelativeAfter = 0,
        NSRelativeBefore = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSWhoseSubelementIdentifier {
        NSIndexSubelement = 0,
        NSEverySubelement = 1,
        NSMiddleSubelement = 2,
        NSRandomSubelement = 3,
        NSNoSubelement = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptObjectSpecifier;

    unsafe impl ClassType for NSScriptObjectSpecifier {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSScriptObjectSpecifier {}

unsafe impl NSObjectProtocol for NSScriptObjectSpecifier {}

extern_methods!(
    unsafe impl NSScriptObjectSpecifier {
        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other objectSpecifierWithDescriptor:)]
        pub unsafe fn objectSpecifierWithDescriptor(
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Id<NSScriptObjectSpecifier>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other childSpecifier)]
        pub unsafe fn childSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier>>;

        #[method(setChildSpecifier:)]
        pub unsafe fn setChildSpecifier(&self, child_specifier: Option<&NSScriptObjectSpecifier>);

        #[method_id(@__retain_semantics Other containerSpecifier)]
        pub unsafe fn containerSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier>>;

        #[method(setContainerSpecifier:)]
        pub unsafe fn setContainerSpecifier(
            &self,
            container_specifier: Option<&NSScriptObjectSpecifier>,
        );

        #[method(containerIsObjectBeingTested)]
        pub unsafe fn containerIsObjectBeingTested(&self) -> bool;

        #[method(setContainerIsObjectBeingTested:)]
        pub unsafe fn setContainerIsObjectBeingTested(
            &self,
            container_is_object_being_tested: bool,
        );

        #[method(containerIsRangeContainerObject)]
        pub unsafe fn containerIsRangeContainerObject(&self) -> bool;

        #[method(setContainerIsRangeContainerObject:)]
        pub unsafe fn setContainerIsRangeContainerObject(
            &self,
            container_is_range_container_object: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: &NSString);

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription"
        ))]
        #[method_id(@__retain_semantics Other containerClassDescription)]
        pub unsafe fn containerClassDescription(&self) -> Option<Id<NSScriptClassDescription>>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription"
        ))]
        #[method(setContainerClassDescription:)]
        pub unsafe fn setContainerClassDescription(
            &self,
            container_class_description: Option<&NSScriptClassDescription>,
        );

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription"
        ))]
        #[method_id(@__retain_semantics Other keyClassDescription)]
        pub unsafe fn keyClassDescription(&self) -> Option<Id<NSScriptClassDescription>>;

        #[method(indicesOfObjectsByEvaluatingWithContainer:count:)]
        pub unsafe fn indicesOfObjectsByEvaluatingWithContainer_count(
            &self,
            container: &AnyObject,
            count: NonNull<NSInteger>,
        ) -> *mut NSInteger;

        #[method_id(@__retain_semantics Other objectsByEvaluatingWithContainers:)]
        pub unsafe fn objectsByEvaluatingWithContainers(
            &self,
            containers: &AnyObject,
        ) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other objectsByEvaluatingSpecifier)]
        pub unsafe fn objectsByEvaluatingSpecifier(&self) -> Option<Id<AnyObject>>;

        #[method(evaluationErrorNumber)]
        pub unsafe fn evaluationErrorNumber(&self) -> NSInteger;

        #[method(setEvaluationErrorNumber:)]
        pub unsafe fn setEvaluationErrorNumber(&self, evaluation_error_number: NSInteger);

        #[method_id(@__retain_semantics Other evaluationErrorSpecifier)]
        pub unsafe fn evaluationErrorSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier>>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Option<Id<NSAppleEventDescriptor>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScriptObjectSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_category!(
    /// Category "NSScriptObjectSpecifiers" on [`NSObject`].
    #[doc(alias = "NSScriptObjectSpecifiers")]
    pub unsafe trait NSObjectNSScriptObjectSpecifiers {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other objectSpecifier)]
        unsafe fn objectSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSScriptObjectSpecifiers",
            feature = "Foundation_NSValue"
        ))]
        #[method_id(@__retain_semantics Other indicesOfObjectsByEvaluatingObjectSpecifier:)]
        unsafe fn indicesOfObjectsByEvaluatingObjectSpecifier(
            &self,
            specifier: &NSScriptObjectSpecifier,
        ) -> Option<Id<NSArray<NSNumber>>>;
    }

    unsafe impl NSObjectNSScriptObjectSpecifiers for NSObject {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSIndexSpecifier;

    unsafe impl ClassType for NSIndexSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSIndexSpecifier {}

unsafe impl NSObjectProtocol for NSIndexSpecifier {}

extern_methods!(
    unsafe impl NSIndexSpecifier {
        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:index:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_index(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            index: NSInteger,
        ) -> Id<Self>;

        #[method(index)]
        pub unsafe fn index(&self) -> NSInteger;

        #[method(setIndex:)]
        pub unsafe fn setIndex(&self, index: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSIndexSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSIndexSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMiddleSpecifier;

    unsafe impl ClassType for NSMiddleSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSMiddleSpecifier {}

unsafe impl NSObjectProtocol for NSMiddleSpecifier {}

extern_methods!(
    unsafe impl NSMiddleSpecifier {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSMiddleSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMiddleSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNameSpecifier;

    unsafe impl ClassType for NSNameSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSNameSpecifier {}

unsafe impl NSObjectProtocol for NSNameSpecifier {}

extern_methods!(
    unsafe impl NSNameSpecifier {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:name:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_name(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            name: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSNameSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNameSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPositionalSpecifier;

    unsafe impl ClassType for NSPositionalSpecifier {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSPositionalSpecifier {}

extern_methods!(
    unsafe impl NSPositionalSpecifier {
        #[method_id(@__retain_semantics Init initWithPosition:objectSpecifier:)]
        pub unsafe fn initWithPosition_objectSpecifier(
            this: Allocated<Self>,
            position: NSInsertionPosition,
            specifier: &NSScriptObjectSpecifier,
        ) -> Id<Self>;

        #[method(position)]
        pub unsafe fn position(&self) -> NSInsertionPosition;

        #[method_id(@__retain_semantics Other objectSpecifier)]
        pub unsafe fn objectSpecifier(&self) -> Id<NSScriptObjectSpecifier>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription"
        ))]
        #[method(setInsertionClassDescription:)]
        pub unsafe fn setInsertionClassDescription(
            &self,
            class_description: &NSScriptClassDescription,
        );

        #[method(evaluate)]
        pub unsafe fn evaluate(&self);

        #[method_id(@__retain_semantics Other insertionContainer)]
        pub unsafe fn insertionContainer(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other insertionKey)]
        pub unsafe fn insertionKey(&self) -> Option<Id<NSString>>;

        #[method(insertionIndex)]
        pub unsafe fn insertionIndex(&self) -> NSInteger;

        #[method(insertionReplaces)]
        pub unsafe fn insertionReplaces(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPositionalSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPropertySpecifier;

    unsafe impl ClassType for NSPropertySpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSPropertySpecifier {}

unsafe impl NSObjectProtocol for NSPropertySpecifier {}

extern_methods!(
    unsafe impl NSPropertySpecifier {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSPropertySpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPropertySpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRandomSpecifier;

    unsafe impl ClassType for NSRandomSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSRandomSpecifier {}

unsafe impl NSObjectProtocol for NSRandomSpecifier {}

extern_methods!(
    unsafe impl NSRandomSpecifier {}
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSRandomSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSRandomSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRangeSpecifier;

    unsafe impl ClassType for NSRangeSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSRangeSpecifier {}

unsafe impl NSObjectProtocol for NSRangeSpecifier {}

extern_methods!(
    unsafe impl NSRangeSpecifier {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:startSpecifier:endSpecifier:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_startSpecifier_endSpecifier(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            start_spec: Option<&NSScriptObjectSpecifier>,
            end_spec: Option<&NSScriptObjectSpecifier>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other startSpecifier)]
        pub unsafe fn startSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier>>;

        #[method(setStartSpecifier:)]
        pub unsafe fn setStartSpecifier(&self, start_specifier: Option<&NSScriptObjectSpecifier>);

        #[method_id(@__retain_semantics Other endSpecifier)]
        pub unsafe fn endSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier>>;

        #[method(setEndSpecifier:)]
        pub unsafe fn setEndSpecifier(&self, end_specifier: Option<&NSScriptObjectSpecifier>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSRangeSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSRangeSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRelativeSpecifier;

    unsafe impl ClassType for NSRelativeSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSRelativeSpecifier {}

unsafe impl NSObjectProtocol for NSRelativeSpecifier {}

extern_methods!(
    unsafe impl NSRelativeSpecifier {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:relativePosition:baseSpecifier:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_relativePosition_baseSpecifier(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            rel_pos: NSRelativePosition,
            base_specifier: Option<&NSScriptObjectSpecifier>,
        ) -> Id<Self>;

        #[method(relativePosition)]
        pub unsafe fn relativePosition(&self) -> NSRelativePosition;

        #[method(setRelativePosition:)]
        pub unsafe fn setRelativePosition(&self, relative_position: NSRelativePosition);

        #[method_id(@__retain_semantics Other baseSpecifier)]
        pub unsafe fn baseSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier>>;

        #[method(setBaseSpecifier:)]
        pub unsafe fn setBaseSpecifier(&self, base_specifier: Option<&NSScriptObjectSpecifier>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSRelativeSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSRelativeSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUniqueIDSpecifier;

    unsafe impl ClassType for NSUniqueIDSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSUniqueIDSpecifier {}

unsafe impl NSObjectProtocol for NSUniqueIDSpecifier {}

extern_methods!(
    unsafe impl NSUniqueIDSpecifier {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:uniqueID:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_uniqueID(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            unique_id: &AnyObject,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other uniqueID)]
        pub unsafe fn uniqueID(&self) -> Id<AnyObject>;

        #[method(setUniqueID:)]
        pub unsafe fn setUniqueID(&self, unique_id: &AnyObject);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSUniqueIDSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUniqueIDSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWhoseSpecifier;

    unsafe impl ClassType for NSWhoseSpecifier {
        #[inherits(NSObject)]
        type Super = NSScriptObjectSpecifier;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSWhoseSpecifier {}

unsafe impl NSObjectProtocol for NSWhoseSpecifier {}

extern_methods!(
    unsafe impl NSWhoseSpecifier {
        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, in_coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSScriptWhoseTests",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:test:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_test(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            test: &NSScriptWhoseTest,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSScriptWhoseTests")]
        #[method_id(@__retain_semantics Other test)]
        pub unsafe fn test(&self) -> Id<NSScriptWhoseTest>;

        #[cfg(feature = "Foundation_NSScriptWhoseTests")]
        #[method(setTest:)]
        pub unsafe fn setTest(&self, test: &NSScriptWhoseTest);

        #[method(startSubelementIdentifier)]
        pub unsafe fn startSubelementIdentifier(&self) -> NSWhoseSubelementIdentifier;

        #[method(setStartSubelementIdentifier:)]
        pub unsafe fn setStartSubelementIdentifier(
            &self,
            start_subelement_identifier: NSWhoseSubelementIdentifier,
        );

        #[method(startSubelementIndex)]
        pub unsafe fn startSubelementIndex(&self) -> NSInteger;

        #[method(setStartSubelementIndex:)]
        pub unsafe fn setStartSubelementIndex(&self, start_subelement_index: NSInteger);

        #[method(endSubelementIdentifier)]
        pub unsafe fn endSubelementIdentifier(&self) -> NSWhoseSubelementIdentifier;

        #[method(setEndSubelementIdentifier:)]
        pub unsafe fn setEndSubelementIdentifier(
            &self,
            end_subelement_identifier: NSWhoseSubelementIdentifier,
        );

        #[method(endSubelementIndex)]
        pub unsafe fn endSubelementIndex(&self) -> NSInteger;

        #[method(setEndSubelementIndex:)]
        pub unsafe fn setEndSubelementIndex(&self, end_subelement_index: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptObjectSpecifier`
    unsafe impl NSWhoseSpecifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            this: Allocated<Self>,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSClassDescription",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            this: Allocated<Self>,
            class_desc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWhoseSpecifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
