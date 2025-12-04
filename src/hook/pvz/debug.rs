use std::{sync::OnceLock};
use windows::core::PCSTR;

use super::HookRegistration;
use crate::{hook::hook_api, pvz::debug::hijack_debug_output};

/// `OutputDebugStringA` 的签名
type SignOutputDebugStringA = extern "system" fn(
    PCSTR
);
/// `OutputDebugStringA` 的跳板
pub static ORIGINAL_OUTPUT_DEBUG_STRING_A: OnceLock<SignOutputDebugStringA> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_OUTPUT_DEBUG_STRING_A.set(hook_api(
            "kernel32.dll", 
            "OutputDebugStringA", 
            hijack_debug_output as _
        )?);

        Ok(())
    })
}
