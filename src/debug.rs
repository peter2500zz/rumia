use anyhow::Result;
use config::load_config;
use serde::Deserialize;
use tracing::debug;
use windows::Win32::System::Console::AllocConsole;

use crate::{
    CONFIG,
    pvz::{
        board::board::with_board, lawn_app::lawn_app::with_lawn_app,
        widget_manager::widget_manager::with_widget_manager,
    },
    save::PROFILE_MANAGER, utils::Vec2,
};

#[derive(Deserialize)]
struct Config {
    show_terminal: Option<bool>,
}

pub fn alloc_console() -> Result<()> {
    unsafe {
        if let Some(show_terminal) = load_config::<Config>(CONFIG).show_terminal
            && show_terminal
        {
            AllocConsole()?;
        }

        Ok(())
    }
}

pub fn tigger_handler(flag: String) {
    unsafe {
        match flag.as_str() {
            "save slot" => {
                let _ = with_lawn_app(|lawn_app| {
                    debug!("{}", (*lawn_app.player_info).save_slot);

                    Ok(())
                });
            }

            "sun" => {
                let _ = with_board(|board| {
                    board.sun_value = 9999;

                    Ok(())
                });
            }

            "boom" => {
                let _ = with_lawn_app(|the_app| {
                    with_widget_manager(|wm| {
                        with_board(|board| {
                            let mouse_pos = wm.mouse_pos;
                            let grid_pos = crate::pvz::board::PixelToGridKeepOnBoard(board, mouse_pos);

                            crate::pvz::board::KillAllZombiesInRadius(
                                board,
                                grid_pos.y,
                                mouse_pos,
                                250,
                                3,
                                true,
                                127,
                            );

                            crate::pvz::effect_system::particle_holder::particle_system::AllocParticleSystem(
                                (*the_app.effect_system).particle, 
                                Vec2::new(mouse_pos.x as _, mouse_pos.y as _), 
                                400000, 
                                30
                            );

                            Ok(())
                        })
                    })
                });
            }

            "zombie" => {
                let _ = with_widget_manager(|wm| {
                    with_board(|board| {
                        let mouse_pos = wm.mouse_pos;
                        let grid_pos = crate::pvz::board::PixelToGridKeepOnBoard(board, mouse_pos);

                        let zombie = crate::pvz::board::AddZombieInRow(board, 0, 0, grid_pos.y);

                        (*zombie).pos = crate::utils::Vec2::new(mouse_pos.x as _, mouse_pos.y as _);

                        Ok(())
                    })
                });
            }

            "win" => {
                let _ = with_lawn_app(|lawn_app| {
                    debug!("pre: {}", (*(*lawn_app).board).is_winning);
                    (*(*lawn_app).board).is_winning = true;
                    debug!("tiggerred win: {}", (*(*lawn_app).board).is_winning);
                    debug!("pre: {}", (*(*lawn_app).board).is_win);
                    (*(*lawn_app).board).is_win = true;
                    debug!("tiggerred win: {}", (*(*lawn_app).board).is_win);

                    Ok(())
                });
            }

            "data" => {
                if let Ok(pm) = PROFILE_MANAGER.lock() {
                    debug!("{:#?}", pm);
                }
            }

            "shoot" => {
                debug!("cool!");
                let _ = with_board(|board| {
                    for plant in board.plants.iter_mut() {
                        if plant.plant_subtype == 1 {
                            debug!("good plant shoot shoot shoot");
                            crate::pvz::plant::FireWithoutTarget(plant, plant.row, 0);
                        }
                    }

                    Ok(())
                });
            }

            "effsys" => {
                let _ = with_lawn_app(|lawn_app| {
                    let effsys = &*lawn_app.effect_system;
                    let psys = &*effsys.particle;
                    debug!("ps: {}", psys.systems.debug_name_to_string());
                    debug!("pe: {}", psys.emitters.debug_name_to_string());
                    debug!("pp: {}", psys.particles.debug_name_to_string());
                    debug!("t: {}", (*effsys.trails).debug_name_to_string());
                    debug!("r: {}", (*effsys.reanims).debug_name_to_string());
                    debug!("a: {}", (*effsys.attach).debug_name_to_string());

                    Ok(())
                });
            }

            _ => {
                debug!("Invalid debug tigger");
            }
        }
    }
}
