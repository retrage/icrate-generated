//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum _SFTranscriptionOptions {
        _SFTranscriptionOptionsNormalizedTranscription = 1 << 0,
        _SFTranscriptionOptionsContextualizedTranscription = 1 << 1,
        _SFTranscriptionOptionsPunctuation = 1 << 2,
        _SFTranscriptionOptionsEmoji = 1 << 3,
        _SFTranscriptionOptionsEtiquetteReplacements = 1 << 4,
    }
);