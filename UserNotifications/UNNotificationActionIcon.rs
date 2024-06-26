//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationActionIcon;

    unsafe impl ClassType for UNNotificationActionIcon {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNNotificationActionIcon {}

unsafe impl NSCopying for UNNotificationActionIcon {}

unsafe impl NSObjectProtocol for UNNotificationActionIcon {}

unsafe impl NSSecureCoding for UNNotificationActionIcon {}

extern_methods!(
    unsafe impl UNNotificationActionIcon {
        #[method_id(@__retain_semantics Other iconWithTemplateImageName:)]
        pub unsafe fn iconWithTemplateImageName(template_image_name: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Other iconWithSystemImageName:)]
        pub unsafe fn iconWithSystemImageName(system_image_name: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationActionIcon {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
