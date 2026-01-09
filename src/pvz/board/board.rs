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

        // 键码
        let mouse_codes = lua.create_table()?;

        mouse_codes.set("L_CLICK", 1)?; // 鼠标左键
        mouse_codes.set("L_DOUBLE_CLICK", 2)?; // 鼠标左键双击
        mouse_codes.set("R_CLICK", -1)?; // 鼠标右键
        mouse_codes.set("R_DOUBLE_CLICK", -2)?; // 鼠标右键双击
        mouse_codes.set("M_CLICK", 3)?; // 鼠标中键

        globals.set("MouseCodes", mouse_codes)?;

        // 伤害标志位
        let damage_flag = lua.create_table()?;

        damage_flag.set("GROUND", 1 << 0)?;      // 0x01 - 地面攻击
        damage_flag.set("AIR", 1 << 1)?;         // 0x02 - 空中攻击
        damage_flag.set("WATER", 1 << 2)?;       // 0x04 - 水中攻击
        damage_flag.set("SPECIAL", 1 << 4)?;     // 0x10 - 特殊状态攻击
        damage_flag.set("STEALTH", 1 << 5)?;     // 0x20 - 可攻击隐身/特殊僵尸
        damage_flag.set("JUMPING", 1 << 6)?;     // 0x40 - 可攻击跳跃状态
        damage_flag.set("CHARMED", 1 << 7)?;     // 0x80 - 可攻击被魅惑的僵尸

        globals.set("DamageFlagsw", damage_flag)?;

        Ok(())
    })
}

pub fn get_board() -> LuaResult<*mut Board> {
    unsafe {
        get_lawn_app().and_then(|lawn_app| {
            if (*lawn_app).board.is_null() {
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
