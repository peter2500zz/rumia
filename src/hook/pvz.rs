use std::{ffi::c_void, sync::OnceLock};
use anyhow::Result;
use windows::{
    Win32::{Foundation::HINSTANCE, UI::WindowsAndMessaging::SHOW_WINDOW_CMD}, 
    core::PSTR,
};

use super::hook;
use crate::pvz;


const ADDR_WINMAIN: *mut c_void = 0x0044E8F0 as _;
type SignWinMain = extern "stdcall" fn(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    lpCmdLine: PSTR,
    nCmdShow: SHOW_WINDOW_CMD
) -> i32;
pub static ORIGINAL_WINMAIN: OnceLock<SignWinMain> = OnceLock::new();


pub(super) fn init_hook() -> Result<()> {
    let _ = ORIGINAL_WINMAIN.set(hook(ADDR_WINMAIN, pvz::WinMain as _)?);

    Ok(())
}

