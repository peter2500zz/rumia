use mlua::prelude::*;

use crate::{mods::LuaRegistration, pvz::{board::board::Board, widget_manager::WidgetManager}};

const ADDR_LAWN_APP_BASE: u32 = 0x006A9EC0;

pub fn get_lawn_app() -> Option<*mut LawnApp> {
    unsafe {
        if (*(ADDR_LAWN_APP_BASE as *const u32)) == 0 {
            None
        } else {
            Some(*(ADDR_LAWN_APP_BASE as *const *mut LawnApp))
        }
    }
}

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();

        let lua_get_lawn_app = lua.create_function(move |lua, ()| {
            if let Some(p_lawn_app) = get_lawn_app() {
                unsafe {
                    // // 1. 获取引用
                    // let app_ref = &*p_lawn_app;

                    // // 2. 强转为 'static 引用 (为了欺骗 mlua 的生命周期检查)
                    // // mlua 的 create_userdata 通常要求数据拥有所有权或是 'static 引用
                    // let static_ref: &'static LawnApp = std::mem::transmute(app_ref);

                    // 3. 【关键修正】显式创建 UserData
                    // create_userdata 会返回一个 AnyUserData 对象，这个对象实现了 IntoLua
                    let userdata = lua.create_userdata(std::ptr::read(p_lawn_app))?;

                    Ok(mlua::Value::UserData(userdata))
                }
            } else {
                Ok(mlua::Value::Nil)
            }
        })?;

        globals.set("LawnApp", lua_get_lawn_app)?;

        Ok(())
    })
}

#[derive(Debug)]
#[repr(C)]
/// 这是 `LawnApp`
/// 
/// 手动管理生命周期并不好玩，孩子们
pub struct LawnApp {
    _pad_0x0_0x768: [u8; 0x768 - 0x0],
    /// 0x768 游戏关卡
    pub board: *mut Board,
    _pad_0x76C_0x8C8: [u8; 0x8C8 - 0x76C],
}
const _: () = assert!(size_of::<LawnApp>() == 0x8C8);

impl LuaUserData for LawnApp {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_function_get("aaa", |lua, this| {
            

            Ok(LuaNil)
        });
    }
}
