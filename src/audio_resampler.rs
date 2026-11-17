use hbb_common::thiserror;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AudioResamplerConfig {
    pub input_rate: u32,
    pub output_rate: u32,
    pub channels: u16,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(crate) enum AudioResamplerError {
    #[error(
        "invalid audio resampler configuration: input_rate={}, output_rate={}, channels={}",
        .0.input_rate, .0.output_rate, .0.channels
    )]
    InvalidConfig(AudioResamplerConfig),
    #[error("invalid resampler output frame size: {output_frames}")]
    InvalidOutputFrameSize { output_frames: usize },
    #[error("audio resampler input length {samples} is not divisible by channel count {channels}")]
    IncompleteFrame { samples: usize, channels: usize },
    #[error("audio resampler output capacity overflow")]
    CapacityOverflow,
}

pub(crate) struct FixedFrameAudioResampler;

impl FixedFrameAudioResampler {
    pub(crate) fn new(
        _config: AudioResamplerConfig,
        _output_frames: usize,
    ) -> Result<Self, AudioResamplerError> {
        Ok(Self)
    }

    pub(crate) fn process_with(
        &mut self,
        _input: &[f32],
        _on_packet: impl FnMut(&[f32]),
    ) -> Result<(), AudioResamplerError> {
        Ok(())
    }
}

pub(crate) struct AudioResampler;

impl AudioResampler {
    pub(crate) fn new(_config: AudioResamplerConfig) -> Result<Self, AudioResamplerError> {
        Ok(Self)
    }

    pub(crate) fn process(&mut self, _input: &[f32]) -> Result<Vec<f32>, AudioResamplerError> {
        Ok(Vec::new())
    }
}
