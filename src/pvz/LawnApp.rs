use tracing::{debug, trace};

use crate::hook::pvz::LawnApp::{
    ORIGINAL_LAWNAPP_CONSTRUCTOR, 
    ORIGINAL_LAWNAPP_DESTRUCTOR
};

#[derive(Debug)]
#[repr(C)]
/// 这是 `LawnApp`
/// 
/// 手动管理生命周期并不好玩，孩子们
pub struct LawnApp {
    _pad: [u8; 0x8C8],  
}

/// 这是 `LawnApp` 的构造函数
pub extern "stdcall" fn Constructor(uninit: *mut LawnApp) -> *mut LawnApp {
    trace!("构造");

    let this = ORIGINAL_LAWNAPP_CONSTRUCTOR.wait()(uninit);

    debug!("地址 {:#x?}", this);

    this
}

/// 这是 `LawnApp` 的析构函数
pub extern "thiscall" fn Destructor(this: *mut LawnApp) {
    trace!("析构");

    ORIGINAL_LAWNAPP_DESTRUCTOR.wait()(this)
}
