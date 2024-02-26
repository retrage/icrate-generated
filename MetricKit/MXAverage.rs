//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXAverage<UnitType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut UnitType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<UnitType: ?Sized + Message> ClassType for MXAverage<UnitType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl<UnitType: ?Sized + NSCoding> NSCoding for MXAverage<UnitType> {}

unsafe impl<UnitType: ?Sized> NSObjectProtocol for MXAverage<UnitType> {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl<UnitType: ?Sized + NSSecureCoding> NSSecureCoding for MXAverage<UnitType> {}

extern_methods!(
    unsafe impl<UnitType: Message> MXAverage<UnitType> {
        #[cfg(feature = "Foundation_NSMeasurement")]
        #[method_id(@__retain_semantics Other averageMeasurement)]
        pub unsafe fn averageMeasurement(&self) -> Id<NSMeasurement<UnitType>>;

        #[method(sampleCount)]
        pub unsafe fn sampleCount(&self) -> NSInteger;

        #[method(standardDeviation)]
        pub unsafe fn standardDeviation(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<UnitType: Message> MXAverage<UnitType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
