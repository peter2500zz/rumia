pub mod board;
pub mod coin;
pub mod debug;
pub mod effect_system;
pub mod graphics;
pub mod lawn_app;
pub mod plant;
pub mod player_info;
pub mod resource_manager;
pub mod widget_container;
pub mod widget_manager;
pub mod zombie;

use tracing::{debug, info};
use windows::{
    Win32::{Foundation::HINSTANCE, UI::WindowsAndMessaging::SHOW_WINDOW_CMD},
    core::PSTR,
};

use crate::{hook::pvz::ORIGINAL_WINMAIN, pause};

pub extern "stdcall" fn WinMain(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    lpCmdLine: PSTR,
    nCmdShow: SHOW_WINDOW_CMD,
) -> i32 {
    let args = unsafe { lpCmdLine.to_string().unwrap_or_default() };

    debug!("args={}", if args.is_empty() { "<none>" } else { &args });

    info!("main process started");
    debug!("hinstance={:#x?}", hInstance.0);

    let result = ORIGINAL_WINMAIN.wait()(hInstance, hPrevInstance, lpCmdLine, nCmdShow);

    info!("main process exited");
    debug!("exit_code={:#x}", result);

    pause!("Press any key to continue...");

    result
}
