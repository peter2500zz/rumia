use mlua::prelude::*;
pub(super) use tracing::{debug, error, info, trace, warn};

use super::LuaRegistration;

macro_rules! create_log_func {
    ($lua:expr, $log_table:expr, $name:expr, $macro:ident) => {{
        let func = $lua.create_function(|lua, args: LuaMultiValue| {
            let mut message = String::new();
            let mut formatted = false;

            // 检查参数数量和第一个参数类型
            if let Some(first) = args.iter().next() {
                if let LuaValue::String(_) = first {
                    // 如果有多个参数，且第一个是字符串，尝试调用 string.format
                    if args.len() > 1 {
                        let string_format: LuaFunction = lua
                            .globals()
                            .get::<LuaTable>("string")?
                            .get("format")?;

                        message = string_format.call::<String>(&args)?;
                        formatted = true;
                    }
                }
            }

            if !formatted {
                message = args
                    .iter()
                    .map(|v| v.to_string().unwrap_or_else(|_| format!("{:?}", v)))
                    .collect::<Vec<_>>()
                    .join(" ");
            }

            $macro!("{}", message);
            Ok(())
        })?;
        $log_table.set($name, func)?;
    }};
}

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();
        let log_table = lua.create_table()?;

        create_log_func!(lua, log_table, "trace", trace);
        create_log_func!(lua, log_table, "debug", debug);
        create_log_func!(lua, log_table, "info", info);
        create_log_func!(lua, log_table, "warn", warn);
        create_log_func!(lua, log_table, "error", error);

        globals.set("Log", log_table)?;
        Ok(())
    })
}
