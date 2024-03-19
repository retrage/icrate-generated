//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSBindingName = NSString;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSBindingOption = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBindingSelectionMarker;

    unsafe impl ClassType for NSBindingSelectionMarker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSBindingSelectionMarker {}

unsafe impl NSObjectProtocol for NSBindingSelectionMarker {}

extern_methods!(
    unsafe impl NSBindingSelectionMarker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other multipleValuesSelectionMarker)]
        pub unsafe fn multipleValuesSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[method_id(@__retain_semantics Other noSelectionMarker)]
        pub unsafe fn noSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[method_id(@__retain_semantics Other notApplicableSelectionMarker)]
        pub unsafe fn notApplicableSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDefaultPlaceholder:forMarker:onClass:withBinding:)]
        pub unsafe fn setDefaultPlaceholder_forMarker_onClass_withBinding(
            placeholder: Option<&AnyObject>,
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &AnyClass,
            binding: &NSBindingName,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultPlaceholderForMarker:onClass:withBinding:)]
        pub unsafe fn defaultPlaceholderForMarker_onClass_withBinding(
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &AnyClass,
            binding: &NSBindingName,
        ) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBindingSelectionMarker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static NSMultipleValuesMarker: &'static AnyObject;
}

extern "C" {
    pub static NSNoSelectionMarker: &'static AnyObject;
}

extern "C" {
    pub static NSNotApplicableMarker: &'static AnyObject;
}

extern "C" {
    pub fn NSIsControllerMarker(object: Option<&AnyObject>) -> Bool;
}

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSBindingInfoKey = NSString;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSObservedObjectKey: &'static NSBindingInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSObservedKeyPathKey: &'static NSBindingInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSOptionsKey: &'static NSBindingInfoKey;
}

extern_category!(
    /// Category "NSKeyValueBindingCreation" on [`NSObject`].
    #[doc(alias = "NSKeyValueBindingCreation")]
    pub unsafe trait NSObjectNSKeyValueBindingCreation {
        #[cfg(all(feature = "AppKit_NSKeyValueBinding", feature = "Foundation_NSString"))]
        #[method(exposeBinding:)]
        unsafe fn exposeBinding(binding: &NSBindingName);

        #[cfg(all(
            feature = "AppKit_NSKeyValueBinding",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other exposedBindings)]
        unsafe fn exposedBindings(&self) -> Id<NSArray<NSBindingName>>;

        #[cfg(all(feature = "AppKit_NSKeyValueBinding", feature = "Foundation_NSString"))]
        #[method(valueClassForBinding:)]
        unsafe fn valueClassForBinding(&self, binding: &NSBindingName)
            -> Option<&'static AnyClass>;

        #[cfg(all(
            feature = "AppKit_NSKeyValueBinding",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(bind:toObject:withKeyPath:options:)]
        unsafe fn bind_toObject_withKeyPath_options(
            &self,
            binding: &NSBindingName,
            observable: &AnyObject,
            key_path: &NSString,
            options: Option<&NSDictionary<NSBindingOption, AnyObject>>,
        );

        #[cfg(all(feature = "AppKit_NSKeyValueBinding", feature = "Foundation_NSString"))]
        #[method(unbind:)]
        unsafe fn unbind(&self, binding: &NSBindingName);

        #[cfg(all(
            feature = "AppKit_NSKeyValueBinding",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other infoForBinding:)]
        unsafe fn infoForBinding(
            &self,
            binding: &NSBindingName,
        ) -> Option<Id<NSDictionary<NSBindingInfoKey, AnyObject>>>;

        #[cfg(all(
            feature = "AppKit_NSKeyValueBinding",
            feature = "CoreData_NSAttributeDescription",
            feature = "CoreData_NSPropertyDescription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other optionDescriptionsForBinding:)]
        unsafe fn optionDescriptionsForBinding(
            &self,
            binding: &NSBindingName,
        ) -> Id<NSArray<NSAttributeDescription>>;
    }

    unsafe impl NSObjectNSKeyValueBindingCreation for NSObject {}
);

extern_protocol!(
    pub unsafe trait NSEditor: NSObjectProtocol + IsMainThreadOnly {
        #[method(discardEditing)]
        unsafe fn discardEditing(&self);

        #[method(commitEditing)]
        unsafe fn commitEditing(&self) -> bool;

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&AnyObject>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(commitEditingAndReturnError:_)]
        unsafe fn commitEditingAndReturnError(&self) -> Result<(), Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn NSEditor {}
);

extern_protocol!(
    pub unsafe trait NSEditorRegistration: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(objectDidBeginEditing:)]
        unsafe fn objectDidBeginEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[optional]
        #[method(objectDidEndEditing:)]
        unsafe fn objectDidEndEditing(&self, editor: &ProtocolObject<dyn NSEditor>);
    }

    unsafe impl ProtocolType for dyn NSEditorRegistration {}
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAlignmentBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAlternateImageBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAlternateTitleBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAnimateBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAnimationDelayBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSArgumentBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAttributedStringBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentArrayBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentArrayForMultipleSelectionBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentDictionaryBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentHeightBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentObjectBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentObjectsBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentSetBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentValuesBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentWidthBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSCriticalValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDataBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDisplayPatternTitleBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDisplayPatternValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDocumentEditedBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDoubleClickArgumentBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDoubleClickTargetBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSEditableBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSEnabledBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSExcludedKeysBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFilterPredicateBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFontBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFontBoldBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFontFamilyNameBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFontItalicBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFontNameBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSFontSizeBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSHeaderTitleBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSHiddenBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSImageBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSIncludedKeysBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInitialKeyBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInitialValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSIsIndeterminateBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLabelBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLocalizedKeyDictionaryBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSManagedObjectContextBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMaximumRecentsBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMaxValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMaxWidthBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMinValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMinWidthBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMixedStateImageBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSOffStateImageBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSOnStateImageBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPositioningRectBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPredicateBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSRecentSearchesBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSRepresentedFilenameBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSRowHeightBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedIdentifierBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedIndexBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedLabelBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedObjectBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedObjectsBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedTagBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectedValuesBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectionIndexesBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectionIndexPathsBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSortDescriptorsBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTargetBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTextColorBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTitleBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSToolTipBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTransparentBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSValuePathBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSValueURLBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSVisibleBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSWarningValueBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSWidthBinding: &'static NSBindingName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAllowsEditingMultipleValuesSelectionBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAllowsNullArgumentBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAlwaysPresentsApplicationModalAlertsBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSConditionallySetsEditableBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSConditionallySetsEnabledBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSConditionallySetsHiddenBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContinuouslyUpdatesValueBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSCreatesSortDescriptorBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDeletesObjectsOnRemoveBindingsOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDisplayNameBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDisplayPatternBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSContentPlacementTagBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSHandlesContentAsCompoundValueBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInsertsNullPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInvokesSeparatelyWithArrayObjectsBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMultipleValuesPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSNoSelectionPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSNotApplicablePlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSNullPlaceholderBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSRaisesForNotApplicableKeysBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPredicateFormatBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectorNameBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSSelectsAllWhenSettingContentBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSValidatesImmediatelyBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSValueTransformerNameBindingOption: &'static NSBindingOption;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSValueTransformerBindingOption: &'static NSBindingOption;
}
