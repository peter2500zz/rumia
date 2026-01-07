mod debug;
mod hook;
mod logger;
mod mods;
#[allow(non_snake_case)]
mod pvz;
mod save;
mod utils;

use ::serde::{Deserialize, Serialize};
use anyhow::Result;
use config::{load_config, save_config};
use std::ffi::c_void;
use tracing::{debug, info};
use windows::{
    Win32::{
        Foundation::HINSTANCE,
        System::SystemServices::{
            DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
        },
    },
    core::BOOL,
};
use windows_wrapper::mb;

use crate::{debug::alloc_console, hook::init_hook, logger::setup_logger};

const CONFIG: &str = "conf.yml";

#[derive(Serialize, Deserialize, Debug)]
struct LoaderConfig {
    force_launch: Option<bool>,
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: u32,
    lpReserved: *mut c_void,
) -> BOOL {
    // just satisfy clippy
    let _ = lpReserved;

    let result = match fdwReason {
        DLL_PROCESS_ATTACH => match on_pocess_attach(hinstDLL) {
            Ok(_) => {
                info!("initialization successful");

                true
            }
            Err(e) => {
                let mut cfg: LoaderConfig = load_config(CONFIG);
                let extra_msg = if cfg.force_launch.unwrap_or(false) {
                    cfg.force_launch = Some(false);
                    save_config(CONFIG, &cfg);

                    "\n\n由于发生错误，已禁用强制启动。你可以在下次启动时选择其他可执行文件。\nForce launch has been disabled due to an error. You can select a different executable on the next launch."
                } else {
                    ""
                };

                mb!(
                    "初始化时遇到问题。\nAn issue occurred during initialization.\n{}{}",
                    e,
                    extra_msg
                );

                false
            }
        },
        DLL_PROCESS_DETACH => true,
        DLL_THREAD_ATTACH => true,
        DLL_THREAD_DETACH => true,

        _ => unreachable!(),
    };

    BOOL::from(result)
}

fn on_pocess_attach(handle: HINSTANCE) -> Result<()> {
    alloc_console()?;
    setup_logger()?;

    info!("DLL injection successful");
    debug!("hinstDLL={:#x?}", handle.0);

    init_hook()?;

    info!("hook installed successfully");

    Ok(())
}
