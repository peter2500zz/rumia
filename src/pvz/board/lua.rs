use std::ptr;

use mlua::prelude::*;

use crate::{
    mods::ToLua,
    pvz::board::{
        AddCoin, AddZombieInRow, GetPlantsOnLawn, PixelToGridKeepOnBoard,
        board::{Board, PlantsOnLawn, with_board},
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
            with_board(|board| Ok(board.sun_value = value))
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
                let zombie = AddZombieInRow(board, from_wave, zombie_type, row);

                unsafe { (*zombie).to_lua(lua) }
            })
        });

        methods.add_method("AddCoin", |_, _, (pos, theCoinType, theCoinMotion)| {
            with_board(|board| {
                let coin = AddCoin(board, pos, theCoinType, theCoinMotion);

                unsafe { Ok(ptr::read(coin)) }
            })
        });

        methods.add_method("PosToGridKeepOnBoard", |_, _, pos| {
            with_board(|board| Ok(PixelToGridKeepOnBoard(board, pos)))
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

        methods.add_method("GetPlantByGrid", |lua, _, grid: Vec2<_>| {
            with_board(|board| {
                let mut plants = PlantsOnLawn::default();

                GetPlantsOnLawn(board, &mut plants, grid.x, grid.y);

                unsafe {
                    Ok((
                        if plants.normal.is_null() {
                            LuaNil
                        } else {
                            (*plants.normal).to_lua(lua)?
                        },
                        if plants.buttom.is_null() {
                            LuaNil
                        } else {
                            (*plants.buttom).to_lua(lua)?
                        },
                        if plants.outer.is_null() {
                            LuaNil
                        } else {
                            (*plants.outer).to_lua(lua)?
                        },
                        if plants.flying.is_null() {
                            LuaNil
                        } else {
                            (*plants.flying).to_lua(lua)?
                        },
                    ))
                }
            })
        });
    }
}
