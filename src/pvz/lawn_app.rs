pub mod lawn_app;
pub mod loading;
pub mod save;
pub mod lua;

use tracing::{debug, error, info, trace};
use windows::{Win32::UI::WindowsAndMessaging::SetWindowTextW, core::w};

use crate::{
    add_callback,
    hook::pvz::lawn_app::{
        ADDR_INIT, ORIGINAL_CONSTRUCTOR, ORIGINAL_DESTRUCTOR, ORIGINAL_INIT, ORIGINAL_LOST_FOCUS,
    },
    mods::{
        callback::{POST, callback},
        load_mods,
    },
    pvz::lawn_app::lawn_app::LawnApp,
};

/// 这是 `LawnApp` 的构造函数
pub extern "stdcall" fn Constructor(uninit: *mut LawnApp) -> *mut LawnApp {
    trace!("constructing lawnapp");

    // unsafe {
    // info!("{:?}", (0x6A9EC0 as *mut usize).is_null());
    // info!("{:?}", *(0x6A9EC0 as *mut usize));
    // }

    let this = ORIGINAL_CONSTRUCTOR.wait()(uninit);

    trace!("address={:#x?}", this);

    this
}

/// 这是 `LawnApp` 的析构函数
pub extern "thiscall" fn Destructor(this: *mut LawnApp) {
    trace!("destructing lawnapp");

    ORIGINAL_DESTRUCTOR.wait()(this);
}

/// `LawnApp` 的初始化函数
///
/// 包括读取设定数据及存档、加载资源、创建标题界面及初始化游戏内的各个系统等
pub extern "thiscall" fn Init(this: *mut LawnApp) {
    trace!("initializing lawnapp");

    ORIGINAL_INIT.wait()(this);

    unsafe {
        let _ = SetWindowTextW((*this).hwnd, w!("Plants vs. Zombies with Rumia"));
    }

    match load_mods() {
        Ok(loaded) => {
            if loaded != 0 {
                info!("loaded {} mod(s)", loaded);
            }
        }
        Err(e) => {
            error!("error while loading mods: {}", e);
        }
    }
    callback(POST | ADDR_INIT, ());

    // unsafe {
    //     (*(*this).resource_manager).use_system_font = true;
    //     let f = LoadFont((*this).resource_manager, &MsvcString::from("FONT_CONSOLAS"));
    //     info!("{:?}", f);
    // }
}
add_callback!("AT_GAME_INIT", POST | ADDR_INIT);

/// 程序窗口失去焦点
///
/// 如果能暂停且没有启用作弊会暂停，除此之外没有别的作用
pub extern "thiscall" fn LostFocus(this: *mut LawnApp) {
    debug!("game lost focus");

    let _ = this;
    let _ = ORIGINAL_LOST_FOCUS;
    // 仙布暂停
    // ORIGINAL_LAWNAPP_LOST_FOCUS.wait()(
    //     this
    // );
}
