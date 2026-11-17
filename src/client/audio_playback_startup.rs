use super::{AudioHandler, Instant, ResultType};

impl AudioHandler {
    #[inline]
    pub(in crate::client) fn cancel_pending_playback(&mut self) -> bool {
        false
    }

    #[inline]
    pub(in crate::client) fn finish_playback_replacement(
        &mut self,
        _result: ResultType<()>,
        _previous: Option<Self>,
    ) {
    }

    #[inline]
    pub(super) fn resolve_pending_playback(&mut self, _now: Instant) -> Option<bool> {
        None
    }
}
