use std::ffi::c_int;

use tracing::trace;

use crate::{hook::pvz::lawn_app::sound::ORIGINAL_PLAY_SAMPLE, pvz::lawn_app::lawn_app::LawnApp};

pub extern "thiscall" fn PlaySample(this: *mut LawnApp, theSoundNum: c_int) {
    trace!("play sound {}", theSoundNum);
    ORIGINAL_PLAY_SAMPLE.wait()(this, theSoundNum)
}
