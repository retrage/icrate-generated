//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::QuartzCore::*;

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type CAGradientLayerType = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "QuartzCore_CALayer")]
    pub struct CAGradientLayer;

    #[cfg(feature = "QuartzCore_CALayer")]
    unsafe impl ClassType for CAGradientLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "QuartzCore_CALayer", feature = "QuartzCore_CAMediaTiming"))]
unsafe impl CAMediaTiming for CAGradientLayer {}

#[cfg(all(feature = "Foundation_NSObject", feature = "QuartzCore_CALayer"))]
unsafe impl NSCoding for CAGradientLayer {}

#[cfg(feature = "QuartzCore_CALayer")]
unsafe impl NSObjectProtocol for CAGradientLayer {}

#[cfg(all(feature = "Foundation_NSObject", feature = "QuartzCore_CALayer"))]
unsafe impl NSSecureCoding for CAGradientLayer {}

extern_methods!(
    #[cfg(feature = "QuartzCore_CALayer")]
    unsafe impl CAGradientLayer {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other colors)]
        pub unsafe fn colors(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setColors:)]
        pub unsafe fn setColors(&self, colors: Option<&NSArray>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other locations)]
        pub unsafe fn locations(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method(setLocations:)]
        pub unsafe fn setLocations(&self, locations: Option<&NSArray<NSNumber>>);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(startPoint)]
        pub unsafe fn startPoint(&self) -> CGPoint;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setStartPoint:)]
        pub unsafe fn setStartPoint(&self, start_point: CGPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(endPoint)]
        pub unsafe fn endPoint(&self) -> CGPoint;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setEndPoint:)]
        pub unsafe fn setEndPoint(&self, end_point: CGPoint);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<CAGradientLayerType>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &CAGradientLayerType);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "QuartzCore_CALayer")]
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "QuartzCore_CALayer")]
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCAGradientLayerAxial: &'static CAGradientLayerType);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCAGradientLayerRadial: &'static CAGradientLayerType);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCAGradientLayerConic: &'static CAGradientLayerType);
