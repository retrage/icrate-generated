//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFontTraitMask(pub NSUInteger);
impl NSFontTraitMask {
    pub const NSItalicFontMask: Self = Self(0x00000001);
    pub const NSBoldFontMask: Self = Self(0x00000002);
    pub const NSUnboldFontMask: Self = Self(0x00000004);
    pub const NSNonStandardCharacterSetFontMask: Self = Self(0x00000008);
    pub const NSNarrowFontMask: Self = Self(0x00000010);
    pub const NSExpandedFontMask: Self = Self(0x00000020);
    pub const NSCondensedFontMask: Self = Self(0x00000040);
    pub const NSSmallCapsFontMask: Self = Self(0x00000080);
    pub const NSPosterFontMask: Self = Self(0x00000100);
    pub const NSCompressedFontMask: Self = Self(0x00000200);
    pub const NSFixedPitchFontMask: Self = Self(0x00000400);
    pub const NSUnitalicFontMask: Self = Self(0x01000000);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSFontTraitMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSFontTraitMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFontCollectionOptions(pub NSUInteger);
impl NSFontCollectionOptions {
    pub const NSFontCollectionApplicationOnlyMask: Self = Self(1 << 0);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSFontCollectionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSFontCollectionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFontAction(pub NSUInteger);
impl NSFontAction {
    pub const NSNoFontChangeAction: Self = Self(0);
    pub const NSViaPanelFontAction: Self = Self(1);
    pub const NSAddTraitFontAction: Self = Self(2);
    pub const NSSizeUpFontAction: Self = Self(3);
    pub const NSSizeDownFontAction: Self = Self(4);
    pub const NSHeavierFontAction: Self = Self(5);
    pub const NSLighterFontAction: Self = Self(6);
    pub const NSRemoveTraitFontAction: Self = Self(7);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for NSFontAction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for NSFontAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFontManager;

    unsafe impl ClassType for NSFontManager {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSMenuItemValidation for NSFontManager {}

unsafe impl NSObjectProtocol for NSFontManager {}

extern_methods!(
    unsafe impl NSFontManager {
        #[method(setFontPanelFactory:)]
        pub unsafe fn setFontPanelFactory(factory_id: Option<&AnyClass>, mtm: MainThreadMarker);

        #[method(setFontManagerFactory:)]
        pub unsafe fn setFontManagerFactory(factory_id: Option<&AnyClass>, mtm: MainThreadMarker);

        #[method_id(@__retain_semantics Other sharedFontManager)]
        pub unsafe fn sharedFontManager(mtm: MainThreadMarker) -> Id<NSFontManager>;

        #[method(isMultiple)]
        pub unsafe fn isMultiple(&self) -> bool;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other selectedFont)]
        pub unsafe fn selectedFont(&self) -> Option<Id<NSFont>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setSelectedFont:isMultiple:)]
        pub unsafe fn setSelectedFont_isMultiple(&self, font_obj: &NSFont, flag: bool);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setFontMenu:)]
        pub unsafe fn setFontMenu(&self, new_menu: &NSMenu);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other fontMenu:)]
        pub unsafe fn fontMenu(&self, create: bool) -> Option<Id<NSMenu>>;

        #[cfg(all(
            feature = "AppKit_NSFontPanel",
            feature = "AppKit_NSPanel",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSWindow"
        ))]
        #[method_id(@__retain_semantics Other fontPanel:)]
        pub unsafe fn fontPanel(&self, create: bool) -> Option<Id<NSFontPanel>>;

        #[cfg(all(
            feature = "AppKit_NSFont",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fontWithFamily:traits:weight:size:)]
        pub unsafe fn fontWithFamily_traits_weight_size(
            &self,
            family: &NSString,
            traits: NSFontTraitMask,
            weight: NSInteger,
            size: CGFloat,
        ) -> Option<Id<NSFont>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(traitsOfFont:)]
        pub unsafe fn traitsOfFont(&self, font_obj: &NSFont) -> NSFontTraitMask;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(weightOfFont:)]
        pub unsafe fn weightOfFont(&self, font_obj: &NSFont) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other availableFonts)]
        pub unsafe fn availableFonts(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other availableFontFamilies)]
        pub unsafe fn availableFontFamilies(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other availableMembersOfFontFamily:)]
        pub unsafe fn availableMembersOfFontFamily(
            &self,
            fam: &NSString,
        ) -> Option<Id<NSArray<NSArray>>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other convertFont:)]
        pub unsafe fn convertFont(&self, font_obj: &NSFont) -> Id<NSFont>;

        #[cfg(all(feature = "AppKit_NSFont", feature = "Foundation_NSGeometry"))]
        #[method_id(@__retain_semantics Other convertFont:toSize:)]
        pub unsafe fn convertFont_toSize(&self, font_obj: &NSFont, size: CGFloat) -> Id<NSFont>;

        #[cfg(all(feature = "AppKit_NSFont", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other convertFont:toFace:)]
        pub unsafe fn convertFont_toFace(
            &self,
            font_obj: &NSFont,
            typeface: &NSString,
        ) -> Option<Id<NSFont>>;

        #[cfg(all(feature = "AppKit_NSFont", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other convertFont:toFamily:)]
        pub unsafe fn convertFont_toFamily(
            &self,
            font_obj: &NSFont,
            family: &NSString,
        ) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other convertFont:toHaveTrait:)]
        pub unsafe fn convertFont_toHaveTrait(
            &self,
            font_obj: &NSFont,
            r#trait: NSFontTraitMask,
        ) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other convertFont:toNotHaveTrait:)]
        pub unsafe fn convertFont_toNotHaveTrait(
            &self,
            font_obj: &NSFont,
            r#trait: NSFontTraitMask,
        ) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other convertWeight:ofFont:)]
        pub unsafe fn convertWeight_ofFont(&self, up_flag: bool, font_obj: &NSFont) -> Id<NSFont>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(action)]
        pub unsafe fn action(&self) -> Sel;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Sel);

        #[deprecated = "NSFontManager doesn't have any delegate method. This property should not be used."]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AnyObject>>;

        #[deprecated = "NSFontManager doesn't have any delegate method. This property should not be used."]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AnyObject>);

        #[method(sendAction)]
        pub unsafe fn sendAction(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedNameForFamily:face:)]
        pub unsafe fn localizedNameForFamily_face(
            &self,
            family: &NSString,
            face_key: Option<&NSString>,
        ) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setSelectedAttributes:isMultiple:)]
        pub unsafe fn setSelectedAttributes_isMultiple(
            &self,
            attributes: &NSDictionary<NSString, AnyObject>,
            flag: bool,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other convertAttributes:)]
        pub unsafe fn convertAttributes(
            &self,
            attributes: &NSDictionary<NSString, AnyObject>,
        ) -> Id<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[deprecated = "Use -[NSFontDescriptor matchingFontDescriptorsWithMandatoryKeys:] instead"]
        #[method_id(@__retain_semantics Other availableFontNamesMatchingFontDescriptor:)]
        pub unsafe fn availableFontNamesMatchingFontDescriptor(
            &self,
            descriptor: &NSFontDescriptor,
        ) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use +[NSFontCollection allFontCollectionNames] instead"]
        #[method_id(@__retain_semantics Other collectionNames)]
        pub unsafe fn collectionNames(&self) -> Id<NSArray>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -[NSFontCollection matchingDescriptors] instead"]
        #[method_id(@__retain_semantics Other fontDescriptorsInCollection:)]
        pub unsafe fn fontDescriptorsInCollection(
            &self,
            collection_names: &NSString,
        ) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use +[NSFontCollection showFontCollection:withName:visibility:name:] instead"]
        #[method(addCollection:options:)]
        pub unsafe fn addCollection_options(
            &self,
            collection_name: &NSString,
            collection_options: NSFontCollectionOptions,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use +[NSFontCollection hideFontCollectionWithName:visibility:error:] instead"]
        #[method(removeCollection:)]
        pub unsafe fn removeCollection(&self, collection_name: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -[NSMutableFontCollection addQueryForDescriptors:] instead"]
        #[method(addFontDescriptors:toCollection:)]
        pub unsafe fn addFontDescriptors_toCollection(
            &self,
            descriptors: &NSArray,
            collection_name: &NSString,
        );

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSString"))]
        #[deprecated = "Use -[NSMutableFontCollection removeQueryForDescriptors:] instead"]
        #[method(removeFontDescriptor:fromCollection:)]
        pub unsafe fn removeFontDescriptor_fromCollection(
            &self,
            descriptor: &NSFontDescriptor,
            collection: &NSString,
        );

        #[method(currentFontAction)]
        pub unsafe fn currentFontAction(&self) -> NSFontAction;

        #[method(convertFontTraits:)]
        pub unsafe fn convertFontTraits(&self, traits: NSFontTraitMask) -> NSFontTraitMask;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFontManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSFontManagerMenuActionMethods
    unsafe impl NSFontManager {
        #[cfg(feature = "Foundation_NSString")]
        #[method(fontNamed:hasTraits:)]
        pub unsafe fn fontNamed_hasTraits(
            &self,
            f_name: &NSString,
            some_traits: NSFontTraitMask,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other availableFontNamesWithTraits:)]
        pub unsafe fn availableFontNamesWithTraits(
            &self,
            some_traits: NSFontTraitMask,
        ) -> Option<Id<NSArray<NSString>>>;

        #[method(addFontTrait:)]
        pub unsafe fn addFontTrait(&self, sender: Option<&AnyObject>);

        #[method(removeFontTrait:)]
        pub unsafe fn removeFontTrait(&self, sender: Option<&AnyObject>);

        #[method(modifyFontViaPanel:)]
        pub unsafe fn modifyFontViaPanel(&self, sender: Option<&AnyObject>);

        #[method(modifyFont:)]
        pub unsafe fn modifyFont(&self, sender: Option<&AnyObject>);

        #[method(orderFrontFontPanel:)]
        pub unsafe fn orderFrontFontPanel(&self, sender: Option<&AnyObject>);

        #[method(orderFrontStylesPanel:)]
        pub unsafe fn orderFrontStylesPanel(&self, sender: Option<&AnyObject>);
    }
);
