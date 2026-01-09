pub mod this;
pub mod lua;
pub mod profile;

use std::ffi::c_int;
use tracing::*;

use crate::{
    add_callback,
    hook::pvz::board::{
        ADDR_KEYDOWN, ADDR_MOUSE_DOWN, ADDR_MOUSE_UP, ADDR_UPDATE, AddZombieInRowWrapper,
        GetPlantsOnLawnWrapper, ORIGINAL_ADDCOIN, ORIGINAL_CONSTRUCTOR, ORIGINAL_DESTRUCTOR,
        ORIGINAL_DRAW, ORIGINAL_INIT_LEVEL, ORIGINAL_KEYDOWN, ORIGINAL_KILL_ALL_ZOMBIES_IN_RADIUS,
        ORIGINAL_MOUSE_DOWN, ORIGINAL_MOUSE_UP, ORIGINAL_UPDATE, PixelToGridXKeepOnBoardWrapper,
        PixelToGridXWrapper, PixelToGridYKeepOnBoardWrapper, PixelToGridYWrapper,
    },
    mods::callback::{POST, PRE, callback},
    pvz::{
        board::this::{Board, PlantsOnLawn},
        coin::Coin,
        graphics::this::Graphics,
        lawn_app::this::LawnApp,
        zombie::this::Zombie,
    },
    save::PROFILE_MANAGER,
    utils::{
        Vec2,
        delta_mgr::get_delta_mgr,
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

    ORIGINAL_ADDCOIN.wait()(this, pos, theCoinType, theCoinMotion)
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

pub fn PixelToGrid(this: *mut Board, pos: Vec2<i32>) -> Vec2<i32> {
    let grid_x = PixelToGridXWrapper(this, pos.x, pos.y);
    let grid_y = PixelToGridYWrapper(this, pos.x, pos.y);
    Vec2::new(grid_x, grid_y)
}

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
