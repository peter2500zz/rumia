pub mod loading;

use tracing::{debug, trace};

use crate::hook::pvz::lawn_app::{
    ORIGINAL_LAWNAPP_CONSTRUCTOR, 
    ORIGINAL_LAWNAPP_DESTRUCTOR, ORIGINAL_LAWNAPP_INIT
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

    ORIGINAL_LAWNAPP_DESTRUCTOR.wait()(this);
}

/// `LawnApp` 的初始化函数
/// 
/// 包括读取设定数据及存档、加载资源、创建标题界面及初始化游戏内的各个系统等
pub extern "thiscall" fn Init(this: *mut LawnApp) {
    trace!("初始化");

    ORIGINAL_LAWNAPP_INIT.wait()(this);
}
