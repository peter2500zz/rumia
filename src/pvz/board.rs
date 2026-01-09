pub mod board;
pub mod lua;

use anyhow::{Result, anyhow};
use std::{ffi::c_int, fs::File};
use tracing::*;

use crate::{
    add_callback,
    hook::pvz::board::{
        ADDR_KEYDOWN, ADDR_MOUSE_DOWN, ADDR_MOUSE_UP, ADDR_UPDATE, AddZombieInRowWrapper,
        GetPlantsOnLawnWrapper, LawnLoadGameWrapper, LawnSaveGameWrapper, ORIGINAL_ADDCOIN,
        ORIGINAL_CONSTRUCTOR, ORIGINAL_DESTRUCTOR, ORIGINAL_DRAW, ORIGINAL_INIT_LEVEL,
        ORIGINAL_KEYDOWN, ORIGINAL_KILL_ALL_ZOMBIES_IN_RADIUS, ORIGINAL_MOUSE_DOWN,
        ORIGINAL_MOUSE_UP, ORIGINAL_UPDATE, PixelToGridXKeepOnBoardWrapper,
        PixelToGridYKeepOnBoardWrapper,
    },
    mods::callback::{POST, PRE, callback},
    pvz::{
        board::board::{Board, PlantsOnLawn},
        coin::Coin,
        graphics::graphics::Graphics,
        lawn_app::lawn_app::{LawnApp, get_lawn_app},
        zombie::zombie::Zombie,
    },
    save::{PROFILE_MANAGER, SAVES_DIR},
    utils::{
        Vec2,
        delta_mgr::get_delta_mgr,
        msvc_string::MsvcString,
        render_manager::{RenderLayer, execute_layer_render},
    },
};

/// 这是 `Board` 的构造函数
pub extern "thiscall" fn Constructor(uninit: *mut Board, theApp: *mut LawnApp) -> *mut Board {
    trace!("constructing board");

    let this = ORIGINAL_CONSTRUCTOR.wait()(uninit, theApp);

    if let Ok(mut pm) = PROFILE_MANAGER.lock() {
        pm.clear();
    }

    trace!("address={:#x?}", this);

    this
}

/// 这是 `Board` 的析构函数
pub extern "thiscall" fn Destructor(this: *mut Board) {
    trace!("destructing board");

    ORIGINAL_DESTRUCTOR.wait()(this);

    if let Ok(mut pm) = PROFILE_MANAGER.lock() {
        pm.clear();
    }
}

/// `Board` 的初始化函数
///
/// 初始化关卡信息，设定关卡背景、出怪、初始阳光、浓雾坐标等基础数据及卡槽和部分关卡的固定选卡
pub extern "stdcall" fn InitLevel(this: *mut Board) {
    unsafe {
        trace!("initializing board, size={}", size_of_val(&*this));
    }

    ORIGINAL_INIT_LEVEL.wait()(this);
}

/// 在游戏中生成掉落物的函数
pub extern "thiscall" fn AddCoin(
    this: *mut Board,
    pos: Vec2<c_int>,
    theCoinType: c_int,
    theCoinMotion: c_int,
) -> *mut Coin {
    trace!(
        "spawning coin {} at {:?} with motion {}",
        theCoinType, pos, theCoinMotion
    );

    let coin = ORIGINAL_ADDCOIN.wait()(this, pos, theCoinType, theCoinMotion);

    coin
}

/// `Board::KeyDown` 的 hook 函数
pub extern "thiscall" fn KeyDown(this: *mut Board, keycode: c_int) {
    // trace!("Board({:#x?}) 按下 {:#x}", this, keycode);
    if !callback(PRE | ADDR_KEYDOWN, keycode) {
        // 回调
        ORIGINAL_KEYDOWN.wait()(this, keycode);
    }
}
add_callback!("AT_BOARD_KEY_DOWN", PRE | ADDR_KEYDOWN);

pub extern "thiscall" fn AddZombieInRow(
    this: *mut Board,
    theFromWave: c_int,
    theZombieType: c_int,
    theRow: c_int,
) -> *mut Zombie {
    trace!(
        "spawning zombie type {} at wave {} row {}",
        theZombieType, theFromWave, theRow
    );

    AddZombieInRowWrapper(this, theZombieType, theRow, theFromWave)
}

/// 关卡内鼠标点击
pub extern "thiscall" fn MouseDown(this: *mut Board, pos: Vec2<c_int>, theClickCount: c_int) {
    // trace!("Board({:#x?}) 在 {:?} 点击 {}", this, pos, theClickCount);
    if !callback(PRE | ADDR_MOUSE_DOWN, (theClickCount, pos)) {
        ORIGINAL_MOUSE_DOWN.wait()(this, pos, theClickCount)
    }
}
add_callback!("AT_BOARD_MOUSE_DOWN", PRE | ADDR_MOUSE_DOWN);

/// 关卡内鼠标松开
pub extern "thiscall" fn MouseUp(this: *mut Board, pos: Vec2<c_int>, theClickCount: c_int) {
    // trace!("Board({:#x?}) 在 {:?} 松开 {}", this, pos, theClickCount);
    if !callback(PRE | ADDR_MOUSE_UP, (theClickCount, pos)) {
        ORIGINAL_MOUSE_UP.wait()(this, pos, theClickCount)
    }
}
add_callback!("AT_BOARD_MOUSE_UP", PRE | ADDR_MOUSE_UP);

/// 关卡更新
pub extern "thiscall" fn Update(this: *mut Board) {
    // info!("<u<");
    let delta = get_delta_mgr().update_delta("Board::Update");
    ORIGINAL_UPDATE.wait()(this);
    callback(POST | ADDR_UPDATE, delta);
    // info!(">u>");
}
add_callback!("AT_BOARD_UPDATE", POST | ADDR_UPDATE);

pub fn PixelToGridKeepOnBoard(this: *mut Board, pos: Vec2<i32>) -> Vec2<i32> {
    let grid_x = PixelToGridXKeepOnBoardWrapper(this, pos.x, pos.y);
    let grid_y = PixelToGridYKeepOnBoardWrapper(this, pos.x, pos.y);
    Vec2::new(grid_x, grid_y)
}

/// 两次绘制中会有多次更新
pub extern "thiscall" fn Draw(this: *mut Board, g: *mut Graphics) {
    // info!("<d<");
    // pause!();
    ORIGINAL_DRAW.wait()(this, g);

    execute_layer_render(RenderLayer::Board, g);

    // info!(">d>");
}

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

/// 获取特定格内的植物
pub extern "thiscall" fn GetPlantsOnLawn(
    this: *mut Board,
    thePlantOnLawn: *mut PlantsOnLawn,
    theGridPos: Vec2<c_int>,
) {
    // unsafe {
    //     debug!("{:?} {:?} {:?} {:?}", this, (*thePlantOnLawn), theGridX, theGridY);
    // }

    GetPlantsOnLawnWrapper(this, thePlantOnLawn, theGridPos);
}

pub extern "stdcall" fn KillAllZombiesInRadius(
    this: *mut Board,
    theRow: c_int,
    thePos: Vec2<c_int>,
    theRadius: c_int,
    theRowRange: c_int,
    theBurn: bool,
    theDamageRangeFlags: c_int,
) {
    trace!("boooooooom!!!");
    ORIGINAL_KILL_ALL_ZOMBIES_IN_RADIUS.wait()(
        this,
        theRow,
        thePos,
        theRadius,
        theRowRange,
        theBurn,
        theDamageRangeFlags,
    )
}
