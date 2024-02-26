//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub const NSAttachmentCharacter: c_uint = 0xFFFC;

extern_protocol!(
    pub unsafe trait NSTextAttachmentContainer: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "AppKit_NSTextContainer",
            feature = "Foundation_NSGeometry"
        ))]
        #[method_id(@__retain_semantics Other imageForBounds:textContainer:characterIndex:)]
        unsafe fn imageForBounds_textContainer_characterIndex(
            &self,
            image_bounds: CGRect,
            text_container: Option<&NSTextContainer>,
            char_index: NSUInteger,
        ) -> Option<Id<NSImage>>;

        #[cfg(all(feature = "AppKit_NSTextContainer", feature = "Foundation_NSGeometry"))]
        #[method(attachmentBoundsForTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        unsafe fn attachmentBoundsForTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            text_container: Option<&NSTextContainer>,
            line_frag: CGRect,
            position: CGPoint,
            char_index: NSUInteger,
        ) -> CGRect;
    }

    unsafe impl ProtocolType for dyn NSTextAttachmentContainer {}
);

extern_protocol!(
    pub unsafe trait NSTextAttachmentLayout: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "AppKit_NSTextContainer",
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other imageForBounds:attributes:location:textContainer:)]
        unsafe fn imageForBounds_attributes_location_textContainer(
            &self,
            bounds: CGRect,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
        ) -> Option<Id<NSImage>>;

        #[cfg(all(
            feature = "AppKit_NSTextContainer",
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
            proposed_line_fragment: CGRect,
            position: CGPoint,
        ) -> CGRect;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTextContainer",
            feature = "AppKit_NSTextRange",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Other viewProviderForParentView:location:textContainer:)]
        unsafe fn viewProviderForParentView_location_textContainer(
            &self,
            parent_view: Option<&NSView>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
        ) -> Option<Id<NSTextAttachmentViewProvider>>;
    }

    unsafe impl ProtocolType for dyn NSTextAttachmentLayout {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAttachment;

    unsafe impl ClassType for NSTextAttachment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSTextAttachment {}

unsafe impl NSObjectProtocol for NSTextAttachment {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSTextAttachment {}

unsafe impl NSTextAttachmentContainer for NSTextAttachment {}

unsafe impl NSTextAttachmentLayout for NSTextAttachment {}

extern_methods!(
    unsafe impl NSTextAttachment {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithData:ofType:)]
        pub unsafe fn initWithData_ofType(
            this: Allocated<Self>,
            content_data: Option<&NSData>,
            uti: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Init initWithFileWrapper:)]
        pub unsafe fn initWithFileWrapper(
            this: Allocated<Self>,
            file_wrapper: Option<&NSFileWrapper>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&NSData>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: Option<&NSString>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setBounds:)]
        pub unsafe fn setBounds(&self, bounds: CGRect);

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Other fileWrapper)]
        pub unsafe fn fileWrapper(&self) -> Option<Id<NSFileWrapper>>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method(setFileWrapper:)]
        pub unsafe fn setFileWrapper(&self, file_wrapper: Option<&NSFileWrapper>);

        #[cfg(feature = "AppKit_NSTextAttachmentCell")]
        #[method_id(@__retain_semantics Other attachmentCell)]
        pub unsafe fn attachmentCell(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextAttachmentCellProtocol>>>;

        #[cfg(feature = "AppKit_NSTextAttachmentCell")]
        #[method(setAttachmentCell:)]
        pub unsafe fn setAttachmentCell(
            &self,
            attachment_cell: Option<&ProtocolObject<dyn NSTextAttachmentCellProtocol>>,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(lineLayoutPadding)]
        pub unsafe fn lineLayoutPadding(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setLineLayoutPadding:)]
        pub unsafe fn setLineLayoutPadding(&self, line_layout_padding: CGFloat);

        #[cfg(feature = "Foundation_NSString")]
        #[method(textAttachmentViewProviderClassForFileType:)]
        pub unsafe fn textAttachmentViewProviderClassForFileType(
            file_type: &NSString,
        ) -> Option<&'static AnyClass>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerTextAttachmentViewProviderClass:forFileType:)]
        pub unsafe fn registerTextAttachmentViewProviderClass_forFileType(
            text_attachment_view_provider_class: &AnyClass,
            file_type: &NSString,
        );

        #[method(allowsTextAttachmentView)]
        pub unsafe fn allowsTextAttachmentView(&self) -> bool;

        #[method(setAllowsTextAttachmentView:)]
        pub unsafe fn setAllowsTextAttachmentView(&self, allows_text_attachment_view: bool);

        #[method(usesTextAttachmentView)]
        pub unsafe fn usesTextAttachmentView(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextAttachment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_category!(
    /// Category on [`NSAttributedString`].
    pub unsafe trait NSAttributedStringAttachmentConveniences {
        #[cfg(all(
            feature = "AppKit_NSTextAttachment",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method_id(@__retain_semantics Other attributedStringWithAttachment:)]
        unsafe fn attributedStringWithAttachment(
            attachment: &NSTextAttachment,
        ) -> Id<NSAttributedString>;
    }

    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSAttributedStringAttachmentConveniences for NSAttributedString {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAttachmentViewProvider;

    unsafe impl ClassType for NSTextAttachmentViewProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSTextAttachmentViewProvider {}

extern_methods!(
    unsafe impl NSTextAttachmentViewProvider {
        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTextLayoutManager",
            feature = "AppKit_NSTextRange",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Init initWithTextAttachment:parentView:textLayoutManager:location:)]
        pub unsafe fn initWithTextAttachment_parentView_textLayoutManager_location(
            this: Allocated<Self>,
            text_attachment: &NSTextAttachment,
            parent_view: Option<&NSView>,
            text_layout_manager: Option<&NSTextLayoutManager>,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Other textAttachment)]
        pub unsafe fn textAttachment(&self) -> Option<Id<NSTextAttachment>>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Id<ProtocolObject<dyn NSTextLocation>>;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Id<NSView>>;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(loadView)]
        pub unsafe fn loadView(&self);

        #[method(tracksTextAttachmentViewBounds)]
        pub unsafe fn tracksTextAttachmentViewBounds(&self) -> bool;

        #[method(setTracksTextAttachmentViewBounds:)]
        pub unsafe fn setTracksTextAttachmentViewBounds(
            &self,
            tracks_text_attachment_view_bounds: bool,
        );

        #[cfg(all(
            feature = "AppKit_NSTextContainer",
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSGeometry",
            feature = "Foundation_NSString"
        ))]
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        pub unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
            proposed_line_fragment: CGRect,
            position: CGPoint,
        ) -> CGRect;
    }
);

extern_category!(
    /// Category on [`NSMutableAttributedString`].
    pub unsafe trait NSMutableAttributedStringAttachmentConveniences {
        #[cfg(feature = "Foundation_NSString")]
        #[method(updateAttachmentsFromPath:)]
        unsafe fn updateAttachmentsFromPath(&self, path: &NSString);
    }

    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSMutableAttributedStringAttachmentConveniences for NSMutableAttributedString {}
);
