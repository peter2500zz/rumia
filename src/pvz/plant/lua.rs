use mlua::prelude::*;

use crate::{
    mods::ToLua,
    pvz::plant::{
        Fire, FireWithoutTarget,
        plant::{Plant, get_plant, with_plant},
    },
    save::PROFILE_MANAGER,
    utils::data_array::{DataArrayId, HasId},
};

/// 给 Lua 的安全包装
struct LuaPlant(DataArrayId);

impl ToLua for Plant {
    fn to_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        LuaPlant(self.id()).into_lua(lua)
    }
}

impl LuaUserData for LuaPlant {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 外部数据
        methods.add_method("SetAttr", |_, this, (key, value)| {
            with_plant(this.0, |plant| {
                PROFILE_MANAGER.lock().unwrap().set_attr(plant, key, value)
            })
        });
        methods.add_method("GetAttr", |lua, this, key| {
            with_plant(this.0, |plant| {
                Ok(PROFILE_MANAGER.lock().unwrap().get_attr(lua, plant, key))
            })
        });
        methods.add_method("RemoveAttr", |_, this, key| {
            with_plant(this.0, |plant| {
                Ok(PROFILE_MANAGER.lock().unwrap().remove_attr(plant, key))
            })
        });

        // 如果植物被从内存里清理掉了，就给 false
        methods.add_method("IsValid", |_, this, ()| Ok(get_plant(this.0).is_ok()));

        methods.add_method("GetHitbox", |_, this, ()| {
            with_plant(this.0, |plant| Ok(plant.hitbox))
        });

        methods.add_method("Shoot", |_, this, ()| {
            with_plant(this.0, |plant| {
                FireWithoutTarget(plant, plant.row, 0);

                Ok(())
            })
        });

        methods.add_method("ShootRaw", |_, this, ()| {
            with_plant(this.0, |plant| {
                Fire(plant, 0 as _, plant.row, 0);

                Ok(())
            })
        });
    }
}
