//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineCapStyle {
        #[doc(alias = "NSLineCapStyleButt")]
        Butt = 0,
        #[doc(alias = "NSLineCapStyleRound")]
        Round = 1,
        #[doc(alias = "NSLineCapStyleSquare")]
        Square = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineJoinStyle {
        #[doc(alias = "NSLineJoinStyleMiter")]
        Miter = 0,
        #[doc(alias = "NSLineJoinStyleRound")]
        Round = 1,
        #[doc(alias = "NSLineJoinStyleBevel")]
        Bevel = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSWindingRule {
        #[doc(alias = "NSWindingRuleNonZero")]
        NonZero = 0,
        #[doc(alias = "NSWindingRuleEvenOdd")]
        EvenOdd = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBezierPathElement {
        #[doc(alias = "NSBezierPathElementMoveTo")]
        MoveTo = 0,
        #[doc(alias = "NSBezierPathElementLineTo")]
        LineTo = 1,
        #[doc(alias = "NSBezierPathElementCubicCurveTo")]
        CubicCurveTo = 2,
        #[doc(alias = "NSBezierPathElementClosePath")]
        ClosePath = 3,
        #[doc(alias = "NSBezierPathElementQuadraticCurveTo")]
        QuadraticCurveTo = 4,
        #[deprecated]
        #[doc(alias = "NSBezierPathElementCurveTo")]
        CurveTo = NSBezierPathElement::CubicCurveTo.0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBezierPath;

    unsafe impl ClassType for NSBezierPath {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSBezierPath {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSBezierPath {}

unsafe impl NSObjectProtocol for NSBezierPath {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSBezierPath {}

extern_methods!(
    unsafe impl NSBezierPath {
        #[method_id(@__retain_semantics Other bezierPath)]
        pub unsafe fn bezierPath() -> Id<NSBezierPath>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other bezierPathWithRect:)]
        pub unsafe fn bezierPathWithRect(rect: NSRect) -> Id<NSBezierPath>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other bezierPathWithOvalInRect:)]
        pub unsafe fn bezierPathWithOvalInRect(rect: NSRect) -> Id<NSBezierPath>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other bezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn bezierPathWithRoundedRect_xRadius_yRadius(
            rect: NSRect,
            x_radius: CGFloat,
            y_radius: CGFloat,
        ) -> Id<NSBezierPath>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(fillRect:)]
        pub unsafe fn fillRect(rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(strokeRect:)]
        pub unsafe fn strokeRect(rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(clipRect:)]
        pub unsafe fn clipRect(rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(strokeLineFromPoint:toPoint:)]
        pub unsafe fn strokeLineFromPoint_toPoint(point1: NSPoint, point2: NSPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(drawPackedGlyphs:atPoint:)]
        pub unsafe fn drawPackedGlyphs_atPoint(packed_glyphs: NonNull<c_char>, point: NSPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(defaultMiterLimit)]
        pub unsafe fn defaultMiterLimit() -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setDefaultMiterLimit:)]
        pub unsafe fn setDefaultMiterLimit(default_miter_limit: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(defaultFlatness)]
        pub unsafe fn defaultFlatness() -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setDefaultFlatness:)]
        pub unsafe fn setDefaultFlatness(default_flatness: CGFloat);

        #[method(defaultWindingRule)]
        pub unsafe fn defaultWindingRule() -> NSWindingRule;

        #[method(setDefaultWindingRule:)]
        pub unsafe fn setDefaultWindingRule(default_winding_rule: NSWindingRule);

        #[method(defaultLineCapStyle)]
        pub unsafe fn defaultLineCapStyle() -> NSLineCapStyle;

        #[method(setDefaultLineCapStyle:)]
        pub unsafe fn setDefaultLineCapStyle(default_line_cap_style: NSLineCapStyle);

        #[method(defaultLineJoinStyle)]
        pub unsafe fn defaultLineJoinStyle() -> NSLineJoinStyle;

        #[method(setDefaultLineJoinStyle:)]
        pub unsafe fn setDefaultLineJoinStyle(default_line_join_style: NSLineJoinStyle);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(defaultLineWidth)]
        pub unsafe fn defaultLineWidth() -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setDefaultLineWidth:)]
        pub unsafe fn setDefaultLineWidth(default_line_width: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(moveToPoint:)]
        pub unsafe fn moveToPoint(&self, point: NSPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(lineToPoint:)]
        pub unsafe fn lineToPoint(&self, point: NSPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(curveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn curveToPoint_controlPoint1_controlPoint2(
            &self,
            end_point: NSPoint,
            control_point1: NSPoint,
            control_point2: NSPoint,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(curveToPoint:controlPoint:)]
        pub unsafe fn curveToPoint_controlPoint(&self, end_point: NSPoint, control_point: NSPoint);

        #[method(closePath)]
        pub unsafe fn closePath(&self);

        #[method(removeAllPoints)]
        pub unsafe fn removeAllPoints(&self);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(relativeMoveToPoint:)]
        pub unsafe fn relativeMoveToPoint(&self, point: NSPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(relativeLineToPoint:)]
        pub unsafe fn relativeLineToPoint(&self, point: NSPoint);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(relativeCurveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn relativeCurveToPoint_controlPoint1_controlPoint2(
            &self,
            end_point: NSPoint,
            control_point1: NSPoint,
            control_point2: NSPoint,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(relativeCurveToPoint:controlPoint:)]
        pub unsafe fn relativeCurveToPoint_controlPoint(
            &self,
            end_point: NSPoint,
            control_point: NSPoint,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, line_width: CGFloat);

        #[method(lineCapStyle)]
        pub unsafe fn lineCapStyle(&self) -> NSLineCapStyle;

        #[method(setLineCapStyle:)]
        pub unsafe fn setLineCapStyle(&self, line_cap_style: NSLineCapStyle);

        #[method(lineJoinStyle)]
        pub unsafe fn lineJoinStyle(&self) -> NSLineJoinStyle;

        #[method(setLineJoinStyle:)]
        pub unsafe fn setLineJoinStyle(&self, line_join_style: NSLineJoinStyle);

        #[method(windingRule)]
        pub unsafe fn windingRule(&self) -> NSWindingRule;

        #[method(setWindingRule:)]
        pub unsafe fn setWindingRule(&self, winding_rule: NSWindingRule);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miter_limit: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(flatness)]
        pub unsafe fn flatness(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setFlatness:)]
        pub unsafe fn setFlatness(&self, flatness: CGFloat);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(getLineDash:count:phase:)]
        pub unsafe fn getLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: *mut NSInteger,
            phase: *mut CGFloat,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setLineDash:count:phase:)]
        pub unsafe fn setLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: NSInteger,
            phase: CGFloat,
        );

        #[method(stroke)]
        pub unsafe fn stroke(&self);

        #[method(fill)]
        pub unsafe fn fill(&self);

        #[method(addClip)]
        pub unsafe fn addClip(&self);

        #[method(setClip)]
        pub unsafe fn setClip(&self);

        #[method_id(@__retain_semantics Other bezierPathByFlatteningPath)]
        pub unsafe fn bezierPathByFlatteningPath(&self) -> Id<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathByReversingPath)]
        pub unsafe fn bezierPathByReversingPath(&self) -> Id<NSBezierPath>;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method(transformUsingAffineTransform:)]
        pub unsafe fn transformUsingAffineTransform(&self, transform: &NSAffineTransform);

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(currentPoint)]
        pub unsafe fn currentPoint(&self) -> NSPoint;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(controlPointBounds)]
        pub unsafe fn controlPointBounds(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;

        #[method(elementCount)]
        pub unsafe fn elementCount(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(elementAtIndex:associatedPoints:)]
        pub unsafe fn elementAtIndex_associatedPoints(
            &self,
            index: NSInteger,
            points: NSPointArray,
        ) -> NSBezierPathElement;

        #[method(elementAtIndex:)]
        pub unsafe fn elementAtIndex(&self, index: NSInteger) -> NSBezierPathElement;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setAssociatedPoints:atIndex:)]
        pub unsafe fn setAssociatedPoints_atIndex(&self, points: NSPointArray, index: NSInteger);

        #[method(appendBezierPath:)]
        pub unsafe fn appendBezierPath(&self, path: &NSBezierPath);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(appendBezierPathWithRect:)]
        pub unsafe fn appendBezierPathWithRect(&self, rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(appendBezierPathWithPoints:count:)]
        pub unsafe fn appendBezierPathWithPoints_count(
            &self,
            points: NSPointArray,
            count: NSInteger,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(appendBezierPathWithOvalInRect:)]
        pub unsafe fn appendBezierPathWithOvalInRect(&self, rect: NSRect);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:clockwise:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle_clockwise(
            &self,
            center: NSPoint,
            radius: CGFloat,
            start_angle: CGFloat,
            end_angle: CGFloat,
            clockwise: bool,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle(
            &self,
            center: NSPoint,
            radius: CGFloat,
            start_angle: CGFloat,
            end_angle: CGFloat,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(appendBezierPathWithArcFromPoint:toPoint:radius:)]
        pub unsafe fn appendBezierPathWithArcFromPoint_toPoint_radius(
            &self,
            point1: NSPoint,
            point2: NSPoint,
            radius: CGFloat,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(appendBezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn appendBezierPathWithRoundedRect_xRadius_yRadius(
            &self,
            rect: NSRect,
            x_radius: CGFloat,
            y_radius: CGFloat,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: NSPoint) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBezierPath {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSBezierPathDeprecated
    unsafe impl NSBezierPath {
        #[deprecated]
        #[method(cachesBezierPath)]
        pub unsafe fn cachesBezierPath(&self) -> bool;

        #[deprecated]
        #[method(setCachesBezierPath:)]
        pub unsafe fn setCachesBezierPath(&self, flag: bool);

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "Use -appendBezierPathWithCGGlyph:inFont: instead"]
        #[method(appendBezierPathWithGlyph:inFont:)]
        pub unsafe fn appendBezierPathWithGlyph_inFont(&self, glyph: NSGlyph, font: &NSFont);

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "Use -appendBezierPathWithCGGlyphs:count:inFont: instead"]
        #[method(appendBezierPathWithGlyphs:count:inFont:)]
        pub unsafe fn appendBezierPathWithGlyphs_count_inFont(
            &self,
            glyphs: NonNull<NSGlyph>,
            count: NSInteger,
            font: &NSFont,
        );

        #[deprecated = "Use -appendBezierPathWithCGGlyphs:count:inFont: instead"]
        #[method(appendBezierPathWithPackedGlyphs:)]
        pub unsafe fn appendBezierPathWithPackedGlyphs(&self, packed_glyphs: NonNull<c_char>);
    }
);

extern_static!(NSButtLineCapStyle: NSLineCapStyle = NSLineCapStyle(NSLineCapStyle::Butt.0));

extern_static!(NSRoundLineCapStyle: NSLineCapStyle = NSLineCapStyle(NSLineCapStyle::Round.0));

extern_static!(NSSquareLineCapStyle: NSLineCapStyle = NSLineCapStyle(NSLineCapStyle::Square.0));

extern_static!(NSMiterLineJoinStyle: NSLineJoinStyle = NSLineJoinStyle(NSLineJoinStyle::Miter.0));

extern_static!(NSRoundLineJoinStyle: NSLineJoinStyle = NSLineJoinStyle(NSLineJoinStyle::Round.0));

extern_static!(NSBevelLineJoinStyle: NSLineJoinStyle = NSLineJoinStyle(NSLineJoinStyle::Bevel.0));

extern_static!(NSNonZeroWindingRule: NSWindingRule = NSWindingRule(NSWindingRule::NonZero.0));

extern_static!(NSEvenOddWindingRule: NSWindingRule = NSWindingRule(NSWindingRule::EvenOdd.0));

extern_static!(NSMoveToBezierPathElement: NSBezierPathElement = NSBezierPathElement(NSBezierPathElement::MoveTo.0));

extern_static!(NSLineToBezierPathElement: NSBezierPathElement = NSBezierPathElement(NSBezierPathElement::LineTo.0));

extern_static!(NSCurveToBezierPathElement: NSBezierPathElement = NSBezierPathElement(NSBezierPathElement::CurveTo.0));

extern_static!(NSClosePathBezierPathElement: NSBezierPathElement = NSBezierPathElement(NSBezierPathElement::ClosePath.0));
