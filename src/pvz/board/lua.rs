use std::ptr;

use mlua::prelude::*;

use crate::{
    mods::ToLua,
    pvz::{
        board::{
            self,
            this::{Board, PlantsOnLawn, with_board},
        },
        effect_system::particle_holder::particle_system::AllocParticleSystem,
        lawn_app::{sound::PlaySample, this::with_lawn_app},
    },
    utils::{Vec2, data_array::HasId, delta_mgr::get_delta_mgr},
};

struct LuaBoard;

impl ToLua for Board {
    fn to_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        LuaBoard.into_lua(lua)
    }
}

impl LuaUserData for LuaBoard {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetUpdateDelta", |_, _, ()| {
            Ok(get_delta_mgr()
                .get_delta("Board::Update")
                .unwrap_or_default())
        });

        methods.add_method("MousePressing", |_, _, ()| {
            with_board(|board| Ok(board.mouse_pressing))
        });

        methods.add_method("SetSun", |_, _, value: i32| {
            with_board(|board| {
                let _: () = board.sun_value = value;
                Ok(())
            })
        });

        methods.add_method("GetZombies", |lua, _, ()| {
            with_board(|board| {
                let zombies = lua.create_table()?;

                for zombie in board.zombies.iter() {
                    zombies.set(zombie.id(), zombie.to_lua(lua)?)?;
                }

                Ok(zombies)
            })
        });

        methods.add_method("GetZombieById", |lua, _, id| {
            with_board(|board| {
                if let Some(zombie) = board.zombies.get(id) {
                    zombie.to_lua(lua)
                } else {
                    Ok(LuaNil)
                }
            })
        });

        methods.add_method("AddZombie", |lua, _, (zombie_type, row, from_wave)| {
            with_board(|board| {
                let zombie = board::AddZombieInRow(board, from_wave, zombie_type, row);

                unsafe { (*zombie).to_lua(lua) }
            })
        });

        methods.add_method("AddCoin", |_, _, (pos, theCoinType, theCoinMotion)| {
            with_board(|board| {
                let coin = board::AddCoin(board, pos, theCoinType, theCoinMotion);

                unsafe { Ok(ptr::read(coin)) }
            })
        });

        methods.add_method("PosToGridKeepOnBoard", |_, _, pos| {
            with_board(|board| Ok(board::PixelToGridKeepOnBoard(board, pos)))
        });

        methods.add_method("GetPlants", |lua, _, ()| {
            with_board(|board| {
                let plants = lua.create_table()?;

                for plant in board.plants.iter() {
                    plants.set(plant.id(), plant.to_lua(lua)?)?;
                }

                Ok(plants)
            })
        });

        methods.add_method("GetPlantById", |lua, _, id| {
            with_board(|board| {
                if let Some(plant) = board.plants.get(id) {
                    plant.to_lua(lua)
                } else {
                    Ok(LuaNil)
                }
            })
        });

        methods.add_method("GetPlantByGrid", |lua, _, grid_pos: Vec2<i32>| {
            with_board(|board| {
                let plants = lua.create_table()?;

                for plant in board.plants.iter() {
                    if plant.col == grid_pos.x && plant.row == grid_pos.y {
                        plants.set(plant.id(), plant.to_lua(lua)?)?;
                    }
                }

                Ok(plants)
            })
        });

        methods.add_method("Explode", |_, _, (pos, radius, flag, sound, particle)| {
            with_lawn_app(|lawn_app| {
                if let Some(sound) = sound {
                    PlaySample(lawn_app, sound);
                }

                with_board(|board| {
                    let grid = board::PixelToGrid(board, pos);

                    let theRadius = if radius > 80 { radius / 80 } else { 0 };

                    board::KillAllZombiesInRadius(board, grid.y, pos, radius, theRadius, true, flag);

                    if let Some(particle) = particle {
                        unsafe {
                            AllocParticleSystem(
                                (*lawn_app.effect_system).particle,
                                Vec2::new(pos.x as _, pos.y as _),
                                400000,
                                particle,
                            );
                        }
                    }

                    Ok(())
                })
            })
        });

        methods.add_method("AddPlant", |lua, _, (grid_pos, seed_type)| {
            with_board(|board| {
                let plant = board::AddPlant(board, grid_pos, seed_type, -1);

                unsafe { (*plant).to_lua(lua) }
            })
        });
    }
}
