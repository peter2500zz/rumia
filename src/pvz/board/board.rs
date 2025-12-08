use std::{cell::UnsafeCell, ffi::c_void};
use windows::core::BOOL;
use mlua::prelude::*;

use crate::{mods::LuaRegistration, pvz::{data_array::DataArray, lawn_app::lawn_app::get_lawn_app, zombie::zombie::Zombie}};


// inventory::submit! {
//     LuaRegistration(|lua| {

//         let globals = lua.globals();

//         let log_table = lua.create_table()?;

//         globals.set("Log", log_table)?;

//         Ok(())
//     })
// }


pub fn get_board() -> Option<*mut Board> {
    unsafe {
        get_lawn_app().and_then(|lawn_app| {
            if ((*lawn_app).board as u32) == 0 {
                None
            } else {
                Some((*lawn_app).board)
            }
        })
    }
}


#[derive(Debug)]
#[repr(C)]
/// 这是 `Board`
pub struct Board {
    _pad_0x0_0x5560: [u8; 0x5560 - 0x0],
    /// 0x5560 阳光值
    pub sun_value: i32,
    _pad_0x5564_0x57B0: [u8; 0x57B0 - 0x5564],
}
const _: () = assert!(size_of::<Board>() == 0x57B0);

impl LuaUserData for Board {
    
}
