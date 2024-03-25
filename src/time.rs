use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    number::CFNumber,
    string::{CFString, CFStringRef},
};

pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub type CMTimeFlags = u32;

pub const kCMTimeFlags_Valid: CMTimeFlags = 1 << 0;
pub const kCMTimeFlags_HasBeenRounded: CMTimeFlags = 1 << 1;
pub const kCMTimeFlags_PositiveInfinity: CMTimeFlags = 1 << 2;
pub const kCMTimeFlags_NegativeInfinity: CMTimeFlags = 1 << 3;
pub const kCMTimeFlags_Indefinite: CMTimeFlags = 1 << 4;
pub const kCMTimeFlags_ImpliedValueFlagsMask: CMTimeFlags = kCMTimeFlags_PositiveInfinity | kCMTimeFlags_NegativeInfinity | kCMTimeFlags_Indefinite;

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}

extern "C" {
    pub static kCMTimeInvalid: CMTime;
    pub static kCMTimeIndefinite: CMTime;
    pub static kCMTimePositiveInfinity: CMTime;
    pub static kCMTimeNegativeInfinity: CMTime;
    pub static kCMTimeZero: CMTime;

    pub fn CMTimeMake(value: CMTimeValue, timescale: CMTimeScale) -> CMTime;
    pub fn CMTimeMakeWithEpoch(value: CMTimeValue, timescale: CMTimeScale, epoch: CMTimeEpoch) -> CMTime;
    pub fn CMTimeMakeWithSeconds(seconds: f64, preferredTimeScale: i32) -> CMTime;
    pub fn CMTimeGetSeconds(time: CMTime) -> f64;
}

pub type CMTimeRoundingMethod = u32;

pub const kCMTimeRoundingMethod_RoundHalfAwayFromZero: CMTimeRoundingMethod = 1;
pub const kCMTimeRoundingMethod_RoundTowardZero: CMTimeRoundingMethod = 2;
pub const kCMTimeRoundingMethod_RoundAwayFromZero: CMTimeRoundingMethod = 3;
pub const kCMTimeRoundingMethod_QuickTime: CMTimeRoundingMethod = 4;
pub const kCMTimeRoundingMethod_RoundTowardPositiveInfinity: CMTimeRoundingMethod = 5;
pub const kCMTimeRoundingMethod_RoundTowardNegativeInfinity: CMTimeRoundingMethod = 6;
pub const kCMTimeRoundingMethod_Default: CMTimeRoundingMethod = kCMTimeRoundingMethod_RoundHalfAwayFromZero;

extern "C" {
    pub fn CMTimeConvertScale(time: CMTime, newTimescale: CMTimeScale, method: CMTimeRoundingMethod) -> CMTime;
    pub fn CMTimeAdd(lhs: CMTime, rhs: CMTime) -> CMTime;
    pub fn CMTimeSubtract(lhs: CMTime, rhs: CMTime) -> CMTime;
    pub fn CMTimeMultiply(time: CMTime, multiplier: i32) -> CMTime;
    pub fn CMTimeMultiplyByFloat64(time: CMTime, multiplier: f64) -> CMTime;
    pub fn CMTimeMultiplyByRatio(time: CMTime, multiplier: i32, divisor: i32) -> CMTime;
    pub fn CMTimeCompare(time1: CMTime, time2: CMTime) -> i32;
    pub fn CMTimeMinimum(time1: CMTime, time2: CMTime) -> CMTime;
    pub fn CMTimeMaximum(time1: CMTime, time2: CMTime) -> CMTime;
    pub fn CMTimeAbsoluteValue(time: CMTime) -> CMTime;
    pub fn CMTimeCopyAsDictionary(time: CMTime, allocator: CFAllocatorRef) -> CFDictionaryRef;
    pub fn CMTimeMakeFromDictionary(dictionaryRepresentation: CFDictionaryRef) -> CMTime;

    pub static kCMTimeValueKey: CFStringRef;
    pub static kCMTimeScaleKey: CFStringRef;
    pub static kCMTimeEpochKey: CFStringRef;
    pub static kCMTimeFlagsKey: CFStringRef;

    pub fn CMTimeCopyDescription(allocator: CFAllocatorRef, time: CMTime) -> CFStringRef;
    pub fn CMTimeShow(time: CMTime);
}

impl CMTime {
    pub fn make(value: CMTimeValue, timescale: CMTimeScale) -> Self {
        unsafe { CMTimeMake(value, timescale) }
    }

    pub fn make_with_epoch(value: CMTimeValue, timescale: CMTimeScale, epoch: CMTimeEpoch) -> Self {
        unsafe { CMTimeMakeWithEpoch(value, timescale, epoch) }
    }

    pub fn make_with_seconds(seconds: f64, preferred_time_scale: i32) -> Self {
        unsafe { CMTimeMakeWithSeconds(seconds, preferred_time_scale) }
    }

    pub fn get_seconds(&self) -> f64 {
        unsafe { CMTimeGetSeconds(*self) }
    }

    pub fn convert_scale(&self, new_timescale: CMTimeScale, method: CMTimeRoundingMethod) -> Self {
        unsafe { CMTimeConvertScale(*self, new_timescale, method) }
    }

    pub fn add(&self, time: CMTime) -> Self {
        unsafe { CMTimeAdd(*self, time) }
    }

    pub fn subtract(&self, time: CMTime) -> Self {
        unsafe { CMTimeSubtract(*self, time) }
    }

    pub fn multiply(&self, multiplier: i32) -> Self {
        unsafe { CMTimeMultiply(*self, multiplier) }
    }

    pub fn multiply_by_float64(&self, multiplier: f64) -> Self {
        unsafe { CMTimeMultiplyByFloat64(*self, multiplier) }
    }

    pub fn multiply_by_ratio(&self, multiplier: i32, divisor: i32) -> Self {
        unsafe { CMTimeMultiplyByRatio(*self, multiplier, divisor) }
    }

    pub fn compare(&self, time: CMTime) -> i32 {
        unsafe { CMTimeCompare(*self, time) }
    }

    pub fn minimum(&self, time: CMTime) -> Self {
        unsafe { CMTimeMinimum(*self, time) }
    }

    pub fn maximum(&self, time: CMTime) -> Self {
        unsafe { CMTimeMaximum(*self, time) }
    }

    pub fn absolute_value(&self) -> Self {
        unsafe { CMTimeAbsoluteValue(*self) }
    }

    pub fn copy_as_dictionary(&self) -> Option<CFDictionary<CFString, CFNumber>> {
        unsafe {
            let dict = CMTimeCopyAsDictionary(*self, kCFAllocatorDefault);
            if dict.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(dict))
            }
        }
    }

    pub fn make_from_dictionary(dictionary_representation: &CFDictionary<CFString, CFNumber>) -> Self {
        unsafe { CMTimeMakeFromDictionary(dictionary_representation.as_concrete_TypeRef()) }
    }

    pub fn copy_description(&self) -> Option<CFString> {
        unsafe {
            let description = CMTimeCopyDescription(kCFAllocatorDefault, *self);
            if description.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(description))
            }
        }
    }

    pub fn show(&self) {
        unsafe { CMTimeShow(*self) }
    }
}