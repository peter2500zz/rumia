use mlua::prelude::*;

use super::log::*;
use crate::mods::{ToLua, with_lua};

pub const PRE: u32 = 0 << 31;
pub const POST: u32 = 1 << 31;

#[macro_export]
macro_rules! add_callback {
    ($name:literal, $addr:expr) => {
        inventory::submit! {
            $crate::mods::LuaRegistration(|lua| {
                let globals = lua.globals();

                globals.set("ModCallbacks", globals.get("ModCallbacks").unwrap_or(lua.create_table()?))?;
                let mod_callbacks: mlua::Table = globals.get("ModCallbacks")?;

                mod_callbacks.set(format!("{}", $name.to_uppercase()), $addr)?;

                Ok(())
            })
        }
    };
}

/// 通用回调函数 - 支持传递任意实现 IntoLuaMulti 的参数
/// 
/// 如果返回了 `true` 将不会执行原函数
pub fn callback<A>(at: u32, args: A) -> bool
where 
    A: IntoLuaMulti + Clone,
{
    let result = with_lua(|lua| {
        let globals = lua.globals();

        // 获取或创建 Callbacks 表
        let callbacks: LuaTable = match globals.get("Callbacks") {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                globals.set("Callbacks", &t)?;
                t
            }
        };

        // 获取或创建对应 Hook 点的回调列表
        let callback_point: LuaTable = match callbacks.get(at) {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                callbacks.set(at, &t)?;
                t
            }
        };

        // 收集所有回调函数
        let mut funcs = Vec::new();
        for pair in callback_point.pairs::<i32, LuaFunction>() {
            if let Ok((_, func)) = pair {
                funcs.push(func);
            }
        }

        // 执行所有回调
        for func in funcs {
            // 每次调用都 clone args (因为 IntoLuaMulti 会消耗参数)
            match func.call::<bool>(args.clone()) {
                Ok(result) => if result { return Ok(result); },
                Err(e) => error!("Lua callback execution failed: {}", e),
            }
        }

        Ok(false)
    });
    if let Ok(result) = result {
        result
    } else {
        false
    }
}


pub fn callback_data<T>(at: u32, data: &mut T)
where 
    T: ToLua,
{
    // 假设这是你获取全局 Lua 实例的方式
    let _ = with_lua(|lua| {
        let globals = lua.globals();

        // --- 步骤 1: 获取回调函数列表 (这部分逻辑基本不变) ---
        // 查找 Callbacks 表
        let callbacks: LuaTable = match globals.get("Callbacks") {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                globals.set("Callbacks", &t)?;
                t
            }
        };

        // 查找对应的 Hook 点 (at)
        let callback_point: LuaTable = match callbacks.get(at) {
            Ok(t) => t,
            Err(_) => {
                let t = lua.create_table()?;
                callbacks.set(at, &t)?;
                t
            }
        };

        // 收集所有回调函数
        let pairs = callback_point.pairs::<i32, LuaFunction>();
        let mut funcs = Vec::new();
        for pair in pairs {
            if let Ok((_, func)) = pair {
                funcs.push(func);
            }
        }

        for func in funcs {
            match func.call::<bool>(data.to_lua(lua)) {
                Ok(result) => if result { return Ok(result); },
                Err(e) => error!("Lua callback execution failed: {}", e),
            }
        }

        Ok(false)
    });
}