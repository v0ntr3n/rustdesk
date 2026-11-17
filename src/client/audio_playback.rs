use hbb_common::thiserror;
use ringbuf::HeapRb;
use std::sync::{
    atomic::AtomicBool,
    Arc, Mutex,
};

pub(super) const UNDERRUN_DECLICK_MS: usize = 5;
pub(super) const AUDIO_PLAYBACK_LOG_INTERVAL: std::time::Duration =
    std::time::Duration::from_secs(5);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct AudioPlaybackConfig {
    pub sample_rate: u32,
    pub channels: usize,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(super) enum AudioPlaybackError {
    #[error(
        "invalid audio playback configuration: sample_rate={}, channels={}",
        .0.sample_rate, .0.channels
    )]
    InvalidConfig(AudioPlaybackConfig),
    #[error("audio playback frame has {samples} samples for {channels} channels")]
    IncompleteFrame { samples: usize, channels: usize },
    #[error("audio playback transition frame count overflow")]
    FrameCountOverflow,
}

#[derive(Default)]
pub(super) struct AudioPlaybackStatus {
    pub(super) ready: AtomicBool,
}

impl AudioPlaybackStatus {
    #[inline]
    pub(super) fn report_errors(&self) {}
}

pub(super) struct AudioPlaybackWriter {
    pub(super) status: Arc<AudioPlaybackStatus>,
}

impl AudioPlaybackWriter {
    pub(super) fn new(
        _config: AudioPlaybackConfig,
        _audio_buffer: Arc<Mutex<HeapRb<f32>>>,
        _discontinuity_generation: Arc<std::sync::atomic::AtomicUsize>,
    ) -> Result<Self, AudioPlaybackError> {
        Ok(Self {
            status: Default::default(),
        })
    }

    #[inline]
    pub(super) fn write_output<T>(&mut self, output: &mut [T])
    where
        T: cpal::Sample + cpal::FromSample<f32>,
    {
        output.fill(T::from_sample(0.0));
    }
}
