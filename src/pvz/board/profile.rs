use anyhow::{Result, anyhow};
use std::fs::File;
use tracing::debug;

use crate::{
    hook::pvz::board::{LawnLoadGameWrapper, LawnSaveGameWrapper},
    pvz::{board::this::Board, lawn_app::this::get_lawn_app},
    save::{PROFILE_MANAGER, SAVES_DIR},
    utils::msvc_string::MsvcString,
};

/// 游戏读取存档
pub extern "stdcall" fn LawnLoadGame(this: *mut Board, theFilePath: *const MsvcString) -> bool {
    unsafe {
        debug!("load profile from {}", (*theFilePath).to_string());
    }

    let mut success = LawnLoadGameWrapper(this, theFilePath);

    let load_custom_profile = || -> Result<()> {
        if let Ok(the_app) = get_lawn_app() {
            unsafe {
                let json_path = format!(
                    "{}/user{}.json",
                    SAVES_DIR,
                    (*(*the_app).player_info).save_slot
                );
                let maybe_a_file = File::open(&json_path);

                if let Ok(file) = maybe_a_file {
                    debug!("load custom profile from {}", json_path);
                    let mut profile = PROFILE_MANAGER.lock().unwrap();
                    *profile = serde_json::from_reader(file)?;
                }
            }

            Ok(())
        } else {
            Err(anyhow!("can not get LawnApp"))
        }
    };

    if success {
        success = load_custom_profile().is_ok();
    }

    success
}

/// 游戏读取存档
pub extern "stdcall" fn LawnSaveGame(this: *mut Board, theFilePath: *const MsvcString) -> bool {
    unsafe {
        debug!("save profile to {}", (*theFilePath).to_string());
    }

    let mut success = LawnSaveGameWrapper(this, theFilePath);

    let save_custom_profile = || -> Result<()> {
        if let Ok(the_app) = get_lawn_app() {
            unsafe {
                let json_path = format!(
                    "{}/user{}.json",
                    SAVES_DIR,
                    (*(*the_app).player_info).save_slot
                );
                let file = File::create(&json_path)?;
                debug!("save custom profile to {}", json_path);

                let profile = PROFILE_MANAGER.lock().unwrap();
                serde_json::to_writer_pretty(file, &*profile)?;
            }

            Ok(())
        } else {
            Err(anyhow!("can not get LawnApp"))
        }
    };

    if success {
        success = save_custom_profile().is_ok();
    }

    success
}
