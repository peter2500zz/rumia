use std::{ffi::c_int, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::lawn_app::{self, this::LawnApp};

/// `LawnApp::PlaySample` 的地址
const ADDR_PLAY_SAMPLE: u32 = 0x004560C0 as _;
/// `LawnApp::PlaySample` 的签名
type SignPlaySample = extern "thiscall" fn(this: *mut LawnApp, theSoundNum: c_int);
/// `LawnApp::PlaySample` 的跳板
pub static ORIGINAL_PLAY_SAMPLE: OnceLock<SignPlaySample> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_PLAY_SAMPLE.set(
            hook(ADDR_PLAY_SAMPLE as _, lawn_app::sound::PlaySample as _)?
        );

        Ok(())
    })
}
