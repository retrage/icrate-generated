//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSKeyValueObservingOptions {
        NSKeyValueObservingOptionNew = 0x01,
        NSKeyValueObservingOptionOld = 0x02,
        NSKeyValueObservingOptionInitial = 0x04,
        NSKeyValueObservingOptionPrior = 0x08,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSKeyValueChange {
        NSKeyValueChangeSetting = 1,
        NSKeyValueChangeInsertion = 2,
        NSKeyValueChangeRemoval = 3,
        NSKeyValueChangeReplacement = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSKeyValueSetMutationKind {
        NSKeyValueUnionSetMutation = 1,
        NSKeyValueMinusSetMutation = 2,
        NSKeyValueIntersectSetMutation = 3,
        NSKeyValueSetSetMutation = 4,
    }
);

typed_enum!(
    pub type NSKeyValueChangeKey = NSString;
);

extern_static!(NSKeyValueChangeKindKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeNewKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeOldKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeIndexesKey: &'static NSKeyValueChangeKey);

extern_static!(NSKeyValueChangeNotificationIsPriorKey: &'static NSKeyValueChangeKey);

extern_category!(
    /// Category "NSKeyValueObserving" on [`NSObject`].
    #[doc(alias = "NSKeyValueObserving")]
    pub unsafe trait NSObjectNSKeyValueObserving {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(observeValueForKeyPath:ofObject:change:context:)]
        unsafe fn observeValueForKeyPath_ofObject_change_context(
            &self,
            key_path: Option<&NSString>,
            object: Option<&AnyObject>,
            change: Option<&NSDictionary<NSKeyValueChangeKey, AnyObject>>,
            context: *mut c_void,
        );
    }

    unsafe impl NSObjectNSKeyValueObserving for NSObject {}
);

extern_category!(
    /// Category "NSKeyValueObserverRegistration" on [`NSObject`].
    #[doc(alias = "NSKeyValueObserverRegistration")]
    pub unsafe trait NSObjectNSKeyValueObserverRegistration {
        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:forKeyPath:options:context:)]
        unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:context:)]
        unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:)]
        unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, key_path: &NSString);
    }

    unsafe impl NSObjectNSKeyValueObserverRegistration for NSObject {}
);

extern_methods!(
    /// NSKeyValueObserverRegistration
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(addObserver:toObjectsAtIndexes:forKeyPath:options:context:)]
        pub unsafe fn addObserver_toObjectsAtIndexes_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(removeObserver:fromObjectsAtIndexes:forKeyPath:context:)]
        pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath_context(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(removeObserver:fromObjectsAtIndexes:forKeyPath:)]
        pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            key_path: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, key_path: &NSString);
    }
);

extern_methods!(
    /// NSKeyValueObserverRegistration
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, key_path: &NSString);
    }
);

extern_methods!(
    /// NSKeyValueObserverRegistration
    #[cfg(feature = "Foundation_NSSet")]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[cfg(feature = "Foundation_NSString")]
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            key_path: &NSString,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, key_path: &NSString);
    }
);

extern_category!(
    /// Category "NSKeyValueObserverNotification" on [`NSObject`].
    #[doc(alias = "NSKeyValueObserverNotification")]
    pub unsafe trait NSObjectNSKeyValueObserverNotification {
        #[cfg(feature = "Foundation_NSString")]
        #[method(willChangeValueForKey:)]
        unsafe fn willChangeValueForKey(&self, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(didChangeValueForKey:)]
        unsafe fn didChangeValueForKey(&self, key: &NSString);

        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(willChange:valuesAtIndexes:forKey:)]
        unsafe fn willChange_valuesAtIndexes_forKey(
            &self,
            change_kind: NSKeyValueChange,
            indexes: &NSIndexSet,
            key: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSIndexSet", feature = "Foundation_NSString"))]
        #[method(didChange:valuesAtIndexes:forKey:)]
        unsafe fn didChange_valuesAtIndexes_forKey(
            &self,
            change_kind: NSKeyValueChange,
            indexes: &NSIndexSet,
            key: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(willChangeValueForKey:withSetMutation:usingObjects:)]
        unsafe fn willChangeValueForKey_withSetMutation_usingObjects(
            &self,
            key: &NSString,
            mutation_kind: NSKeyValueSetMutationKind,
            objects: &NSSet,
        );

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(didChangeValueForKey:withSetMutation:usingObjects:)]
        unsafe fn didChangeValueForKey_withSetMutation_usingObjects(
            &self,
            key: &NSString,
            mutation_kind: NSKeyValueSetMutationKind,
            objects: &NSSet,
        );
    }

    unsafe impl NSObjectNSKeyValueObserverNotification for NSObject {}
);

extern_category!(
    /// Category "NSKeyValueObservingCustomization" on [`NSObject`].
    #[doc(alias = "NSKeyValueObservingCustomization")]
    pub unsafe trait NSObjectNSKeyValueObservingCustomization {
        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other keyPathsForValuesAffectingValueForKey:)]
        unsafe fn keyPathsForValuesAffectingValueForKey(key: &NSString) -> Id<NSSet<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(automaticallyNotifiesObserversForKey:)]
        unsafe fn automaticallyNotifiesObserversForKey(key: &NSString) -> bool;

        #[method(observationInfo)]
        unsafe fn observationInfo(&self) -> *mut c_void;

        #[method(setObservationInfo:)]
        unsafe fn setObservationInfo(&self, observation_info: *mut c_void);
    }

    unsafe impl NSObjectNSKeyValueObservingCustomization for NSObject {}
);
