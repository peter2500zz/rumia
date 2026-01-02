use std::ptr;

use mlua::prelude::*;

use crate::{
    mods::LuaRegistration,
    pvz::{lawn_app::lawn_app::get_lawn_app, plant::plant::Plant, zombie::zombie::Zombie},
    utils::data_array::DataArray,
};

#[derive(Debug, Default)]
#[repr(C)]
pub struct PlantsOnLawn {
    pub buttom: *mut Plant,
    pub outer: *mut Plant,
    pub flying: *mut Plant,
    pub normal: *mut Plant,
}

#[derive(Debug)]
#[repr(C)]
/// 这是 `Board`
pub struct Board {
    _pad_0x0_0x58: [u8; 0x58 - 0x0],
    /// 0x58 鼠标是否按下（暂停不再记录）
    pub mouse_pressing: bool,
    _pad_0x59_0x90: [u8; 0x90 - 0x59],
    /// 0x90 僵尸数据
    pub zombies: DataArray<Zombie>,
    /// 0xAC 植物数据
    pub plants: DataArray<Plant>,
    _pad_0xC8_0x5560: [u8; 0x5560 - 0xC8],
    /// 0x5560 阳光值
    pub sun_value: i32,
    _pad_0x5564_0x5600: [u8; 0x5600 - 0x5564],
    /// 0x5600 true时为过关过程
    pub is_winning: bool,
    _pad_0x5601_0x560C: [u8; 0x560C - 0x5601],
    /// 0x560C true时为过关状态
    pub is_win: bool,
    _pad_0x560D_0x57B0: [u8; 0x57B0 - 0x560D],
}
const _: () = assert!(size_of::<Board>() == 0x57B0);

impl Clone for Board {
    fn clone(&self) -> Self {
        unsafe { ptr::read(self as *const Self as *mut Self) }
    }
}

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();
        let mouse_codes = lua.create_table()?;

        mouse_codes.set("L_CLICK", 1)?; // 鼠标左键
        mouse_codes.set("L_DOUBLE_CLICK", 2)?; // 鼠标左键双击
        mouse_codes.set("R_CLICK", -1)?; // 鼠标右键
        mouse_codes.set("R_DOUBLE_CLICK", -2)?; // 鼠标右键双击
        mouse_codes.set("M_CLICK", 3)?; // 鼠标中键

        globals.set("MouseCodes", mouse_codes)?;

        Ok(())
    })
}

pub fn get_board() -> LuaResult<*mut Board> {
    unsafe {
        get_lawn_app().and_then(|lawn_app| {
            if ((*lawn_app).board as u32) == 0 {
                Err(LuaError::MemoryError("Board 不可访问".to_string()))
            } else {
                Ok((*lawn_app).board)
            }
        })
    }
}

pub fn with_board<T>(f: impl FnOnce(&mut Board) -> LuaResult<T>) -> LuaResult<T> {
    get_board().and_then(|board| unsafe { f(&mut *board) })
}
