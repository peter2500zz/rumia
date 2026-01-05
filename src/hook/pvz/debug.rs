use std::sync::OnceLock;
use windows::core::PCSTR;

use super::HookRegistration;
use crate::{
    hook::{hook, hook_api},
    pvz::debug::{hijack_debug_output, hijack_fatal_error_output},
    utils::msvc_string::MsvcString,
};

/// `OutputDebugStringA` 的签名
type SignOutputDebugStringA = extern "system" fn(PCSTR);
/// `OutputDebugStringA` 的跳板
pub static ORIGINAL_OUTPUT_DEBUG_STRING_A: OnceLock<SignOutputDebugStringA> = OnceLock::new();

/// `Sexy::SEHCatcher::WriteToFile` 的地址
const ADDR_SEXY_SEHCATCHER_WRITE_TO_FILE: u32 = 0x005A5BD0;
/// `Sexy::SEHCatcher::WriteToFile` 的签名
type SignSexySehcatcherWriteToFile = extern "thiscall" fn(theErrorText: *const MsvcString);
/// `Sexy::SEHCatcher::WriteToFile` 的跳板
pub static ORIGINAL_SEXY_SEHCATCHER_WRITE_TO_FILE: OnceLock<SignSexySehcatcherWriteToFile> =
    OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_OUTPUT_DEBUG_STRING_A.set(hook_api(
            "kernel32.dll",
            "OutputDebugStringA",
            hijack_debug_output as _
        )?);

        let _ = ORIGINAL_SEXY_SEHCATCHER_WRITE_TO_FILE.set(
            hook(ADDR_SEXY_SEHCATCHER_WRITE_TO_FILE as _, hijack_fatal_error_output as _)?
        );

        Ok(())
    })
}
