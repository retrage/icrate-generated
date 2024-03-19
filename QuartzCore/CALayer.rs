//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::QuartzCore::*;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type CALayerContentsGravity = NSString;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type CALayerContentsFormat = NSString;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type CALayerContentsFilter = NSString;

// NS_TYPED_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type CALayerCornerCurve = NSString;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAAutoresizingMask(pub c_uint);
impl CAAutoresizingMask {
    pub const kCALayerNotSizable: Self = Self(0);
    pub const kCALayerMinXMargin: Self = Self(1 << 0);
    pub const kCALayerWidthSizable: Self = Self(1 << 1);
    pub const kCALayerMaxXMargin: Self = Self(1 << 2);
    pub const kCALayerMinYMargin: Self = Self(1 << 3);
    pub const kCALayerHeightSizable: Self = Self(1 << 4);
    pub const kCALayerMaxYMargin: Self = Self(1 << 5);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CAAutoresizingMask {
    const ENCODING: Encoding = c_uint::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CAAutoresizingMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAEdgeAntialiasingMask(pub c_uint);
impl CAEdgeAntialiasingMask {
    pub const kCALayerLeftEdge: Self = Self(1 << 0);
    pub const kCALayerRightEdge: Self = Self(1 << 1);
    pub const kCALayerBottomEdge: Self = Self(1 << 2);
    pub const kCALayerTopEdge: Self = Self(1 << 3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CAEdgeAntialiasingMask {
    const ENCODING: Encoding = c_uint::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CAEdgeAntialiasingMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CACornerMask(pub NSUInteger);
impl CACornerMask {
    pub const kCALayerMinXMinYCorner: Self = Self(1 << 0);
    pub const kCALayerMaxXMinYCorner: Self = Self(1 << 1);
    pub const kCALayerMinXMaxYCorner: Self = Self(1 << 2);
    pub const kCALayerMaxXMaxYCorner: Self = Self(1 << 3);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CACornerMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CACornerMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CALayer;

    unsafe impl ClassType for CALayer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "QuartzCore_CAMediaTiming")]
unsafe impl CAMediaTiming for CALayer {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for CALayer {}

unsafe impl NSObjectProtocol for CALayer {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for CALayer {}

extern_methods!(
    unsafe impl CALayer {
        #[method_id(@__retain_semantics Other layer)]
        pub fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Id<Self>;

        #[method_id(@__retain_semantics Other presentationLayer)]
        pub unsafe fn presentationLayer(&self) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other modelLayer)]
        pub unsafe fn modelLayer(&self) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(needsDisplayForKey:)]
        pub unsafe fn needsDisplayForKey(key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setBounds:)]
        pub fn setBounds(&self, bounds: CGRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(position)]
        pub fn position(&self) -> CGPoint;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setPosition:)]
        pub fn setPosition(&self, position: CGPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(zPosition)]
        pub fn zPosition(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setZPosition:)]
        pub fn setZPosition(&self, z_position: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(anchorPoint)]
        pub fn anchorPoint(&self) -> CGPoint;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setAnchorPoint:)]
        pub fn setAnchorPoint(&self, anchor_point: CGPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(anchorPointZ)]
        pub fn anchorPointZ(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setAnchorPointZ:)]
        pub fn setAnchorPointZ(&self, anchor_point_z: CGFloat);

        #[cfg(all(
            feature = "Foundation_NSGeometry",
            feature = "QuartzCore_CATransform3D"
        ))]
        #[method(transform)]
        pub fn transform(&self) -> CATransform3D;

        #[cfg(all(
            feature = "Foundation_NSGeometry",
            feature = "QuartzCore_CATransform3D"
        ))]
        #[method(setTransform:)]
        pub fn setTransform(&self, transform: CATransform3D);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(frame)]
        pub fn frame(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setFrame:)]
        pub fn setFrame(&self, frame: CGRect);

        #[method(isHidden)]
        pub fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub fn setHidden(&self, hidden: bool);

        #[method(isDoubleSided)]
        pub fn isDoubleSided(&self) -> bool;

        #[method(setDoubleSided:)]
        pub fn setDoubleSided(&self, double_sided: bool);

        #[method(isGeometryFlipped)]
        pub fn isGeometryFlipped(&self) -> bool;

        #[method(setGeometryFlipped:)]
        pub fn setGeometryFlipped(&self, geometry_flipped: bool);

        #[method(contentsAreFlipped)]
        pub fn contentsAreFlipped(&self) -> bool;

        #[method_id(@__retain_semantics Other superlayer)]
        pub fn superlayer(&self) -> Option<Id<CALayer>>;

        #[method(removeFromSuperlayer)]
        pub fn removeFromSuperlayer(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sublayers)]
        pub unsafe fn sublayers(&self) -> Option<Id<NSArray<CALayer>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSublayers:)]
        pub unsafe fn setSublayers(&self, sublayers: Option<&NSArray<CALayer>>);

        #[method(addSublayer:)]
        pub fn addSublayer(&self, layer: &CALayer);

        #[method(insertSublayer:atIndex:)]
        pub fn insertSublayer_atIndex(&self, layer: &CALayer, idx: c_uint);

        #[method(insertSublayer:below:)]
        pub fn insertSublayer_below(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(insertSublayer:above:)]
        pub fn insertSublayer_above(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(replaceSublayer:with:)]
        pub unsafe fn replaceSublayer_with(&self, old_layer: &CALayer, new_layer: &CALayer);

        #[cfg(all(
            feature = "Foundation_NSGeometry",
            feature = "QuartzCore_CATransform3D"
        ))]
        #[method(sublayerTransform)]
        pub fn sublayerTransform(&self) -> CATransform3D;

        #[cfg(all(
            feature = "Foundation_NSGeometry",
            feature = "QuartzCore_CATransform3D"
        ))]
        #[method(setSublayerTransform:)]
        pub fn setSublayerTransform(&self, sublayer_transform: CATransform3D);

        #[method_id(@__retain_semantics Other mask)]
        pub fn mask(&self) -> Option<Id<CALayer>>;

        #[method(setMask:)]
        pub unsafe fn setMask(&self, mask: Option<&CALayer>);

        #[method(masksToBounds)]
        pub fn masksToBounds(&self) -> bool;

        #[method(setMasksToBounds:)]
        pub fn setMasksToBounds(&self, masks_to_bounds: bool);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(convertPoint:fromLayer:)]
        pub fn convertPoint_fromLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(convertPoint:toLayer:)]
        pub fn convertPoint_toLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(convertRect:fromLayer:)]
        pub fn convertRect_fromLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(convertRect:toLayer:)]
        pub fn convertRect_toLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[method(convertTime:fromLayer:)]
        pub fn convertTime_fromLayer(
            &self,
            t: CFTimeInterval,
            l: Option<&CALayer>,
        ) -> CFTimeInterval;

        #[method(convertTime:toLayer:)]
        pub fn convertTime_toLayer(&self, t: CFTimeInterval, l: Option<&CALayer>)
            -> CFTimeInterval;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other hitTest:)]
        pub fn hitTest(&self, p: CGPoint) -> Option<Id<CALayer>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(containsPoint:)]
        pub fn containsPoint(&self, p: CGPoint) -> bool;

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<AnyObject>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&AnyObject>);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(contentsRect)]
        pub fn contentsRect(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setContentsRect:)]
        pub fn setContentsRect(&self, contents_rect: CGRect);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other contentsGravity)]
        pub fn contentsGravity(&self) -> Id<CALayerContentsGravity>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setContentsGravity:)]
        pub fn setContentsGravity(&self, contents_gravity: &CALayerContentsGravity);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(contentsScale)]
        pub fn contentsScale(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setContentsScale:)]
        pub fn setContentsScale(&self, contents_scale: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(contentsCenter)]
        pub fn contentsCenter(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setContentsCenter:)]
        pub fn setContentsCenter(&self, contents_center: CGRect);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other contentsFormat)]
        pub fn contentsFormat(&self) -> Id<CALayerContentsFormat>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setContentsFormat:)]
        pub fn setContentsFormat(&self, contents_format: &CALayerContentsFormat);

        #[method(wantsExtendedDynamicRangeContent)]
        pub unsafe fn wantsExtendedDynamicRangeContent(&self) -> bool;

        #[method(setWantsExtendedDynamicRangeContent:)]
        pub unsafe fn setWantsExtendedDynamicRangeContent(
            &self,
            wants_extended_dynamic_range_content: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other minificationFilter)]
        pub fn minificationFilter(&self) -> Id<CALayerContentsFilter>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMinificationFilter:)]
        pub fn setMinificationFilter(&self, minification_filter: &CALayerContentsFilter);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other magnificationFilter)]
        pub fn magnificationFilter(&self) -> Id<CALayerContentsFilter>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMagnificationFilter:)]
        pub fn setMagnificationFilter(&self, magnification_filter: &CALayerContentsFilter);

        #[method(minificationFilterBias)]
        pub fn minificationFilterBias(&self) -> c_float;

        #[method(setMinificationFilterBias:)]
        pub fn setMinificationFilterBias(&self, minification_filter_bias: c_float);

        #[method(isOpaque)]
        pub fn isOpaque(&self) -> bool;

        #[method(setOpaque:)]
        pub fn setOpaque(&self, opaque: bool);

        #[method(display)]
        pub fn display(&self);

        #[method(setNeedsDisplay)]
        pub fn setNeedsDisplay(&self);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setNeedsDisplayInRect:)]
        pub fn setNeedsDisplayInRect(&self, r: CGRect);

        #[method(needsDisplay)]
        pub fn needsDisplay(&self) -> bool;

        #[method(displayIfNeeded)]
        pub fn displayIfNeeded(&self);

        #[method(needsDisplayOnBoundsChange)]
        pub fn needsDisplayOnBoundsChange(&self) -> bool;

        #[method(setNeedsDisplayOnBoundsChange:)]
        pub fn setNeedsDisplayOnBoundsChange(&self, needs_display_on_bounds_change: bool);

        #[method(drawsAsynchronously)]
        pub fn drawsAsynchronously(&self) -> bool;

        #[method(setDrawsAsynchronously:)]
        pub fn setDrawsAsynchronously(&self, draws_asynchronously: bool);

        #[method(edgeAntialiasingMask)]
        pub fn edgeAntialiasingMask(&self) -> CAEdgeAntialiasingMask;

        #[method(setEdgeAntialiasingMask:)]
        pub fn setEdgeAntialiasingMask(&self, edge_antialiasing_mask: CAEdgeAntialiasingMask);

        #[method(allowsEdgeAntialiasing)]
        pub fn allowsEdgeAntialiasing(&self) -> bool;

        #[method(setAllowsEdgeAntialiasing:)]
        pub fn setAllowsEdgeAntialiasing(&self, allows_edge_antialiasing: bool);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(cornerRadius)]
        pub fn cornerRadius(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setCornerRadius:)]
        pub fn setCornerRadius(&self, corner_radius: CGFloat);

        #[method(maskedCorners)]
        pub fn maskedCorners(&self) -> CACornerMask;

        #[method(setMaskedCorners:)]
        pub fn setMaskedCorners(&self, masked_corners: CACornerMask);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cornerCurve)]
        pub fn cornerCurve(&self) -> Id<CALayerCornerCurve>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCornerCurve:)]
        pub fn setCornerCurve(&self, corner_curve: &CALayerCornerCurve);

        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSString"))]
        #[method(cornerCurveExpansionFactor:)]
        pub fn cornerCurveExpansionFactor(curve: &CALayerCornerCurve) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(borderWidth)]
        pub fn borderWidth(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setBorderWidth:)]
        pub fn setBorderWidth(&self, border_width: CGFloat);

        #[method(opacity)]
        pub fn opacity(&self) -> c_float;

        #[method(setOpacity:)]
        pub fn setOpacity(&self, opacity: c_float);

        #[method(allowsGroupOpacity)]
        pub fn allowsGroupOpacity(&self) -> bool;

        #[method(setAllowsGroupOpacity:)]
        pub fn setAllowsGroupOpacity(&self, allows_group_opacity: bool);

        #[method_id(@__retain_semantics Other compositingFilter)]
        pub unsafe fn compositingFilter(&self) -> Option<Id<AnyObject>>;

        #[method(setCompositingFilter:)]
        pub unsafe fn setCompositingFilter(&self, compositing_filter: Option<&AnyObject>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other filters)]
        pub unsafe fn filters(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setFilters:)]
        pub unsafe fn setFilters(&self, filters: Option<&NSArray>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other backgroundFilters)]
        pub unsafe fn backgroundFilters(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setBackgroundFilters:)]
        pub unsafe fn setBackgroundFilters(&self, background_filters: Option<&NSArray>);

        #[method(shouldRasterize)]
        pub fn shouldRasterize(&self) -> bool;

        #[method(setShouldRasterize:)]
        pub fn setShouldRasterize(&self, should_rasterize: bool);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(rasterizationScale)]
        pub fn rasterizationScale(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setRasterizationScale:)]
        pub fn setRasterizationScale(&self, rasterization_scale: CGFloat);

        #[method(shadowOpacity)]
        pub fn shadowOpacity(&self) -> c_float;

        #[method(setShadowOpacity:)]
        pub fn setShadowOpacity(&self, shadow_opacity: c_float);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(shadowOffset)]
        pub fn shadowOffset(&self) -> CGSize;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setShadowOffset:)]
        pub fn setShadowOffset(&self, shadow_offset: CGSize);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(shadowRadius)]
        pub fn shadowRadius(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setShadowRadius:)]
        pub fn setShadowRadius(&self, shadow_radius: CGFloat);

        #[method(autoresizingMask)]
        pub fn autoresizingMask(&self) -> CAAutoresizingMask;

        #[method(setAutoresizingMask:)]
        pub fn setAutoresizingMask(&self, autoresizing_mask: CAAutoresizingMask);

        #[method_id(@__retain_semantics Other layoutManager)]
        pub fn layoutManager(&self) -> Option<Id<ProtocolObject<dyn CALayoutManager>>>;

        #[method(setLayoutManager:)]
        pub fn setLayoutManager(
            &self,
            layout_manager: Option<&ProtocolObject<dyn CALayoutManager>>,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(preferredFrameSize)]
        pub fn preferredFrameSize(&self) -> CGSize;

        #[method(setNeedsLayout)]
        pub fn setNeedsLayout(&self);

        #[method(needsLayout)]
        pub fn needsLayout(&self) -> bool;

        #[method(layoutIfNeeded)]
        pub fn layoutIfNeeded(&self);

        #[method(layoutSublayers)]
        pub fn layoutSublayers(&self);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(resizeSublayersWithOldSize:)]
        pub fn resizeSublayersWithOldSize(&self, size: CGSize);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(resizeWithOldSuperlayerSize:)]
        pub fn resizeWithOldSuperlayerSize(&self, size: CGSize);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultActionForKey:)]
        pub fn defaultActionForKey(event: &NSString) -> Option<Id<ProtocolObject<dyn CAAction>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionForKey:)]
        pub fn actionForKey(&self, event: &NSString) -> Option<Id<ProtocolObject<dyn CAAction>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other actions)]
        pub fn actions(&self) -> Option<Id<NSDictionary<NSString, ProtocolObject<dyn CAAction>>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setActions:)]
        pub fn setActions(
            &self,
            actions: Option<&NSDictionary<NSString, ProtocolObject<dyn CAAction>>>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "QuartzCore_CAAnimation"))]
        #[method(addAnimation:forKey:)]
        pub fn addAnimation_forKey(&self, anim: &CAAnimation, key: Option<&NSString>);

        #[method(removeAllAnimations)]
        pub fn removeAllAnimations(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeAnimationForKey:)]
        pub fn removeAnimationForKey(&self, key: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other animationKeys)]
        pub fn animationKeys(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "QuartzCore_CAAnimation"))]
        #[method_id(@__retain_semantics Other animationForKey:)]
        pub unsafe fn animationForKey(&self, key: &NSString) -> Option<Id<CAAnimation>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other delegate)]
        pub fn delegate(&self) -> Option<Id<ProtocolObject<dyn CALayerDelegate>>>;

        #[method(setDelegate:)]
        pub fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn CALayerDelegate>>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: Option<&NSDictionary>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CALayer {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for CALayer {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_protocol!(
    pub unsafe trait CALayoutManager: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[optional]
        #[method(preferredSizeOfLayer:)]
        unsafe fn preferredSizeOfLayer(&self, layer: &CALayer) -> CGSize;

        #[optional]
        #[method(invalidateLayoutOfLayer:)]
        unsafe fn invalidateLayoutOfLayer(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);
    }

    unsafe impl ProtocolType for dyn CALayoutManager {}
);

extern_protocol!(
    pub unsafe trait CAAction {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(runActionForKey:object:arguments:)]
        unsafe fn runActionForKey_object_arguments(
            &self,
            event: &NSString,
            an_object: &AnyObject,
            dict: Option<&NSDictionary>,
        );
    }

    unsafe impl ProtocolType for dyn CAAction {}
);

#[cfg(feature = "Foundation_NSNull")]
unsafe impl CAAction for NSNull {}

extern_protocol!(
    pub unsafe trait CALayerDelegate: NSObjectProtocol {
        #[optional]
        #[method(displayLayer:)]
        unsafe fn displayLayer(&self, layer: &CALayer);

        #[optional]
        #[method(layerWillDraw:)]
        unsafe fn layerWillDraw(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other actionForLayer:forKey:)]
        unsafe fn actionForLayer_forKey(
            &self,
            layer: &CALayer,
            event: &NSString,
        ) -> Option<Id<ProtocolObject<dyn CAAction>>>;
    }

    unsafe impl ProtocolType for dyn CALayerDelegate {}
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityCenter: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityTop: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityBottom: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityLeft: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityRight: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityTopLeft: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityTopRight: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityBottomLeft: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityBottomRight: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityResize: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityResizeAspect: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAGravityResizeAspectFill: &'static CALayerContentsGravity;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAContentsFormatRGBA8Uint: &'static CALayerContentsFormat;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAContentsFormatRGBA16Float: &'static CALayerContentsFormat;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAContentsFormatGray8Uint: &'static CALayerContentsFormat;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAFilterNearest: &'static CALayerContentsFilter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAFilterLinear: &'static CALayerContentsFilter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAFilterTrilinear: &'static CALayerContentsFilter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCACornerCurveCircular: &'static CALayerCornerCurve;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCACornerCurveContinuous: &'static CALayerCornerCurve;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAOnOrderIn: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCAOnOrderOut: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static kCATransition: &'static NSString;
}
