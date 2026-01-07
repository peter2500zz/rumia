use std::ffi::{CStr, c_char, c_int};
use tracing::info;

use super::lawn_app::LawnApp;
use crate::hook::pvz::lawn_app::loading::{ORIGINAL_LOAD_GROUP, ORIGINAL_LOADING_THREAD_PROC};

pub extern "thiscall" fn LoadingThreadProc(this: *mut LawnApp) {
    ORIGINAL_LOADING_THREAD_PROC.wait()(this);

    // info!("游戏资源加载完毕");
}

/// 加载资源组的函数
///
/// 会分组加载游戏不同类型的资源
pub extern "thiscall" fn LoadGroup(
    this: *mut LawnApp,
    theGroupName: *const c_char,
    theGroupAveMsToLoad: c_int,
) {
    unsafe {
        if let Ok(group_name) = CStr::from_ptr(theGroupName).to_str() {
            info!("loading resource group: {}", group_name);
        }
    }

    ORIGINAL_LOAD_GROUP.wait()(this, theGroupName, theGroupAveMsToLoad);
}
