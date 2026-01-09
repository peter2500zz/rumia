use mlua::prelude::*;

use crate::{
    debug::tigger_handler,
    mods::ToLua,
    pvz::{
        lawn_app::this::{LawnApp, with_lawn_app},
        widget_manager::this::with_widget_manager,
    },
};

struct LuaLawnApp;

impl ToLua for LawnApp {
    fn to_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        LuaLawnApp.into_lua(lua)
    }
}

impl LuaUserData for LuaLawnApp {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("tigger", |_, flag| {
            tigger_handler(flag);

            Ok(())
        });

        // 获取窗口尺寸
        methods.add_method("GetWindowSize", |_, _, ()| {
            with_lawn_app(|lawn_app| Ok(lawn_app.window_size))
        });

        // 获取关卡
        methods.add_method("GetBoard", |lua, _, ()| {
            with_lawn_app(|lawn_app| {
                if lawn_app.board.is_null() {
                    Ok(LuaNil)
                } else {
                    unsafe { (*lawn_app.board).to_lua(lua) }
                }
            })
        });

        // 获取鼠标坐标
        methods.add_method("GetMousePos", |_, _, ()| {
            with_widget_manager(|wm| Ok(wm.mouse_pos))
        });
    }
}
