//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AutomaticAssessmentConfiguration::*;
use crate::Foundation::*;

#[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
extern_static!(AEAssessmentErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum AEAssessmentErrorCode {
        AEAssessmentErrorUnknown = 1,
        AEAssessmentErrorUnsupportedPlatform = 2,
    }
);
