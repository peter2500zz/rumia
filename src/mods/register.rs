use mlua::prelude::*;
use tracing::trace;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) static MOD_CALLBACK_COUNT: AtomicUsize = AtomicUsize::new(0);

use super::LuaRegistration;


inventory::submit! {
    LuaRegistration(|lua| {
        // 取得全局变量表
        let globals = lua.globals();

        let register_mod = lua.create_function(|lua, name: String| {
            let the_mod = lua.create_table()?;

            the_mod.set("name", name)?;

            let add_callback_func = lua.create_function(|lua, (this, callback, function): (LuaTable, u32, LuaFunction)| {
                // 获取 mod 信息
                let name: String = this.get("name")?;

                // 取得全局变量表
                let globals = lua.globals();

                // 获取回调函数表
                globals.set("Callbacks", globals.get("Callbacks").unwrap_or(lua.create_table()?))?;
                let callbacks: LuaTable = globals.get("Callbacks")?;

                // 获取回调点表
                callbacks.set(callback, callbacks.get(callback).unwrap_or(lua.create_table()?))?;
                let callback_point: LuaTable = callbacks.get(callback)?;

                trace!("mod ({}) registered a callback at {}", &name, format!("{} 0x{:08x}", if (callback >> 31) == 0 { "Pre" } else { "Post" }, (callback & (u32::MAX >> 1))));

                MOD_CALLBACK_COUNT.fetch_add(1, Ordering::Relaxed);

                callback_point.raw_push(function)?;

                Ok(())
            })?;

            the_mod.set("AddCallback", add_callback_func)?;

            Ok(the_mod)
        })?;

        globals.set("RegisterMod", register_mod)?;

        Ok(())
    })
}
