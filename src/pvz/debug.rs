use tracing::{error, trace};
use windows::core::PCSTR;

use crate::{hook::pvz::debug::ORIGINAL_OUTPUT_DEBUG_STRING_A, utils::msvc_string::MsvcString};

/// 劫持 Windows 的 `OutputDebugStringA` 来输出调试信息
pub extern "system" fn hijack_debug_output(dbg_str: PCSTR) {
    unsafe {
        if let Ok(dbg_string) = dbg_str.to_string() {
            trace!("{}", dbg_string.trim())
        }
    }

    ORIGINAL_OUTPUT_DEBUG_STRING_A.wait()(dbg_str)
}

pub extern "thiscall" fn hijack_fatal_error_output(theErrorText: *const MsvcString) {
    unsafe {
        error!("{}", (*theErrorText).to_string());
    }
}
