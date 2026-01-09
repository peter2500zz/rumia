use mlua::prelude::*;
use windows::Win32::Foundation::HWND;

use crate::{
    mods::{LuaRegistration, ToLua},
    pvz::{
        board::this::Board, effect_system::EffectSystem, player_info::PlayerInfo,
        resource_manager::ResourceManager, widget_manager::this::WidgetManager,
    },
    utils::Vec2,
};

const ADDR_LAWN_APP_BASE: u32 = 0x006A9EC0;

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();

        let lua_get_lawn_app = lua.create_function(move |lua, ()| {
            unsafe {
                (*get_lawn_app()?).to_lua(lua)
            }
        })?;

        globals.set("GetLawnApp", lua_get_lawn_app)?;

        Ok(())
    })
}

#[derive(Debug)]
#[repr(C)]
/// 这是 `LawnApp`
///
/// 手动管理生命周期并不好玩，孩子们
pub struct LawnApp {
    _pad_0x0_0xC0: [u8; 0xC0],
    /// 0xC0 窗口尺寸
    pub window_size: Vec2<u32>,
    _pad_0xC8_0x320: [u8; 0x320 - 0xC8],
    /// 0x320 控件管理器
    pub widget_manager: *mut WidgetManager,
    _pad_0x324_0x350: [u8; 0x350 - 0x324],
    /// 0x350 窗口句柄
    pub hwnd: HWND,
    _pad_0x354_0x634: [u8; 0x634 - 0x354],
    /// 0x638 资源管理器
    pub resource_manager: *mut ResourceManager,
    _pad_0x638_0x768: [u8; 0x768 - 0x638],
    /// 0x768 游戏关卡
    pub board: *mut Board,
    _pad_0x76C_0x820: [u8; 0x820 - 0x76C],
    /// 0x820 动画系统
    pub effect_system: *mut EffectSystem,
    _pad_0x824_0x82C: [u8; 0x82C - 0x824],
    /// 0x82C 用户档案与存档
    pub player_info: *mut PlayerInfo,
    _pad_0x830_0x8C8: [u8; 0x8C8 - 0x830],
}
const _: () = assert!(size_of::<LawnApp>() == 0x8C8);

pub fn get_lawn_app() -> LuaResult<*mut LawnApp> {
    unsafe {
        if (*(ADDR_LAWN_APP_BASE as *const u32)) == 0 {
            Err(LuaError::MemoryError("LawnApp 不可访问".to_string()))
        } else {
            Ok(*(ADDR_LAWN_APP_BASE as *const *mut LawnApp))
        }
    }
}

pub fn with_lawn_app<T>(f: impl FnOnce(&mut LawnApp) -> LuaResult<T>) -> LuaResult<T> {
    get_lawn_app().and_then(|lawn_app| unsafe { f(&mut *lawn_app) })
}
