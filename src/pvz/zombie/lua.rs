use mlua::prelude::*;

use crate::{
    hook::pvz::board::PixelToGridYKeepOnBoardWrapper,
    mods::ToLua,
    pvz::{
        board::board::with_board,
        zombie::zombie::{Zombie, get_zombie, with_zombie},
    },
    save::PROFILE_MANAGER,
    utils::data_array::{DataArrayId, HasId},
};

struct LuaZombie(DataArrayId);

impl ToLua for Zombie {
    fn to_lua(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        LuaZombie(self.id()).into_lua(lua)
    }
}

impl LuaUserData for LuaZombie {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 外部数据
        methods.add_method("SetAttr", |_, this, (key, value)| {
            with_zombie(this.0, |zombie| {
                PROFILE_MANAGER.lock().unwrap().set_attr(zombie, key, value)
            })
        });
        methods.add_method("GetAttr", |lua, this, key| {
            with_zombie(this.0, |zombie| {
                Ok(PROFILE_MANAGER.lock().unwrap().get_attr(lua, zombie, key))
            })
        });
        methods.add_method("RemoveAttr", |_, this, key| {
            with_zombie(this.0, |zombie| {
                Ok(PROFILE_MANAGER.lock().unwrap().remove_attr(zombie, key))
            })
        });

        // 如果僵尸被从内存里清理掉了，就给 false
        methods.add_method("IsValid", |_, this, ()| Ok(get_zombie(this.0).is_ok()));

        methods.add_method("GetId", |_, this, ()| Ok(this.0));

        methods.add_method("GetPos", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.pos))
        });
        methods.add_method("SetPos", |_, this, pos| {
            with_board(|board| {
                with_zombie(this.0, |zombie| {
                    zombie.pos = pos;
                    zombie.row = PixelToGridYKeepOnBoardWrapper(
                        board,
                        pos.x as _,
                        pos.y as i32 + zombie.hitbox_rect.size.y,
                    );
                    Ok(())
                })
            })
        });
        methods.add_method("SetPosRaw", |_, this, pos| {
            with_zombie(this.0, |zombie| Ok(zombie.pos = pos))
        });

        methods.add_method("GetSpawnWave", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.spawn_wave))
        });

        methods.add_method("SetSpawnWave", |_, this, wave| {
            with_zombie(this.0, |zombie| Ok(zombie.spawn_wave = wave))
        });

        methods.add_method("GetRow", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.row))
        });

        methods.add_method("SetRow", |_, this, row| {
            with_zombie(this.0, |zombie| Ok(zombie.row = row))
        });

        methods.add_method("GetHitbox", |_, this, ()| {
            with_zombie(this.0, |zombie| {
                let mut hitbox = zombie.hitbox_rect;
                hitbox.position.x += zombie.pos.x as i32;
                hitbox.position.y += zombie.pos.y as i32;
                Ok(hitbox)
            })
        });
        methods.add_method("GetHitboxRelative", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.hitbox_rect))
        });

        methods.add_method("GetAtkbox", |_, this, ()| {
            with_zombie(this.0, |zombie| {
                let mut atkbox = zombie.atkbox_rect;
                atkbox.position.x += zombie.pos.x as i32;
                atkbox.position.y += zombie.pos.y as i32;
                Ok(atkbox)
            })
        });
        methods.add_method("GetAtkboxRelative", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.atkbox_rect))
        });

        methods.add_method("GetBodyHp", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.body_hp))
        });
        methods.add_method("SetBodyHp", |_, this, hp| {
            with_zombie(this.0, |zombie| Ok(zombie.body_hp = hp))
        });
        methods.add_method("GetBodyHpMax", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.body_hp_max))
        });
        methods.add_method("SetBodyHpMax", |_, this, hp_max| {
            with_zombie(this.0, |zombie| Ok(zombie.body_hp_max = hp_max))
        });

        methods.add_method("HasHelmet", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.helmet_type != 0))
        });
        methods.add_method("GetHelmetType", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.helmet_type))
        });
        methods.add_method("SetHelmetType", |_, this, helmet_type| {
            with_zombie(this.0, |zombie| Ok(zombie.helmet_type = helmet_type))
        });
        methods.add_method("GetHelmetHp", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.helmet_hp))
        });
        methods.add_method("SetHelmetHp", |_, this, hp| {
            with_zombie(this.0, |zombie| Ok(zombie.helmet_hp = hp))
        });
        methods.add_method("GetHelmetHpMax", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.helmet_hp_max))
        });
        methods.add_method("SetHelmetHpMax", |_, this, hp_max| {
            with_zombie(this.0, |zombie| Ok(zombie.helmet_hp_max = hp_max))
        });

        methods.add_method("HasShield", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.shield_type != 0))
        });
        methods.add_method("GetShieldType", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.shield_type))
        });
        methods.add_method("SetShieldType", |_, this, shield_type| {
            with_zombie(this.0, |zombie| Ok(zombie.shield_type = shield_type))
        });
        methods.add_method("GetShieldHp", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.shield_hp))
        });
        methods.add_method("SetShieldHp", |_, this, hp| {
            with_zombie(this.0, |zombie| Ok(zombie.shield_hp = hp))
        });
        methods.add_method("GetShieldHpMax", |_, this, ()| {
            with_zombie(this.0, |zombie| Ok(zombie.shield_hp_max))
        });
        methods.add_method("SetShieldHpMax", |_, this, hp_max| {
            with_zombie(this.0, |zombie| Ok(zombie.shield_hp_max = hp_max))
        });
    }
}
