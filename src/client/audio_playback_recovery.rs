use super::{AudioFormat, AudioHandler, MediaData};
use cpal::StreamError;
use crossbeam_queue::SegQueue;
use hbb_common::{tokio::time::Instant, ResultType};
use std::{
    sync::{mpsc, Arc},
    time::Duration,
};

pub(super) const STARTUP_CONFIRMATION_TIMEOUT: Duration = Duration::from_secs(5);

#[path = "audio_playback_startup.rs"]
mod startup;

#[derive(Default)]
pub(super) struct PlaybackRecovery {
    pub(super) errors: Arc<SegQueue<StreamError>>,
    pub(super) retry_at: Option<Instant>,
}

impl PlaybackRecovery {
    pub(super) fn new_error_callback(&mut self) -> impl FnMut(StreamError) + Send + 'static {
        move |_error| {}
    }

    #[inline]
    pub(super) fn report_pending(&self) -> bool {
        false
    }
}

impl AudioHandler {
    #[inline]
    pub(super) fn prepare_playback(&mut self, _format: &AudioFormat) {
        self.audio_stream = None;
    }

    #[inline]
    pub(super) fn finish_playback_start(&mut self, _result: ResultType<()>) {}

    #[inline]
    pub(super) fn recover_playback_with(
        &mut self,
        _now: Instant,
        _restart: impl FnOnce(&mut Self, AudioFormat) -> ResultType<()>,
    ) {
    }

    /// Parks until the channel closes or receives a message, avoiding the 1 Hz polling wakeup.
    pub(super) fn receive_audio(
        &mut self,
        receiver: &mpsc::Receiver<MediaData>,
    ) -> Result<MediaData, mpsc::RecvError> {
        receiver.recv()
    }
}

pub(super) fn receive_with_recovery(
    receiver: &mpsc::Receiver<MediaData>,
    _interval: Duration,
    _recover: impl FnMut(),
) -> Result<MediaData, mpsc::RecvError> {
    receiver.recv()
}
