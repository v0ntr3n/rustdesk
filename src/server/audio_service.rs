use super::*;

pub const NAME: &'static str = "audio";
pub const AUDIO_DATA_SIZE_U8: usize = 960 * 4;

/// Returns an inert service instance without spawning any background capture or encoding threads.
pub fn new() -> GenericService {
    EmptyExtraFieldService::new(NAME.to_owned(), true).sp
}

#[inline]
pub fn get_voice_call_input_device() -> Option<String> {
    None
}

#[inline]
pub fn set_voice_call_input_device(_device: Option<String>, _set_if_present: bool) {}

pub fn restart() {}

#[inline]
#[cfg(feature = "screencapturekit")]
pub fn is_screen_capture_kit_available() -> bool {
    false
}
