use std::{
    arch::{asm, naked_asm},
    ffi::c_int,
    sync::{
        OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

use super::{HookRegistration, hook};
use crate::{
    pvz::{
        board::{
            self,
            this::{Board, PlantsOnLawn},
        },
        coin::Coin,
        graphics::this::Graphics,
        lawn_app::this::LawnApp,
        plant::this::Plant,
        zombie::this::Zombie,
    },
    utils::{Vec2, asm::stack_rotate, msvc_string::MsvcString},
};

/// `Board` 构造函数的地址
const ADDR_CONSTRUCTOR: u32 = 0x00407B50;
/// `Board` 构造函数的签名
type SignConstructor = extern "thiscall" fn(uninit: *mut Board, theApp: *mut LawnApp) -> *mut Board;
/// `Board` 构造函数的跳板
pub static ORIGINAL_CONSTRUCTOR: OnceLock<SignConstructor> = OnceLock::new();

/// `Board` 析构函数的地址
const ADDR_DESTRUCTOR: u32 = 0x00408690;
/// `Board` 析构函数的签名
type SignDestructor = extern "thiscall" fn(this: *mut Board);
/// `Board` 析构函数的跳板
pub static ORIGINAL_DESTRUCTOR: OnceLock<SignDestructor> = OnceLock::new();

/// `Board::InitLevel` 的地址
const ADDR_INIT_LEVEL: u32 = 0x0040AF90;
/// `Board::InitLevel` 的签名
type SignInitLevel = extern "stdcall" fn(this: *mut Board);
/// `Board::InitLevel` 的跳板
pub static ORIGINAL_INIT_LEVEL: OnceLock<SignInitLevel> = OnceLock::new();

/// `Board::AddCoin` 的地址
pub const ADDR_ADDCOIN: u32 = 0x0040CB10;
/// `Board::AddCoin` 的签名
type SignAddCoin = extern "thiscall" fn(
    this: *mut Board,
    pos: Vec2<c_int>,
    theCoinType: c_int,
    theCoinMotion: c_int,
) -> *mut Coin;
/// `Board::AddCoin` 的跳板
pub static ORIGINAL_ADDCOIN: OnceLock<SignAddCoin> = OnceLock::new();

/// `Board::KeyDown` 的地址
pub const ADDR_KEYDOWN: u32 = 0x0041B820;
/// `Board::KeyDown` 的签名
type SignKeyDown = extern "thiscall" fn(this: *mut Board, keycode: c_int);
/// `Board::KeyDown` 的跳板
pub static ORIGINAL_KEYDOWN: OnceLock<SignKeyDown> = OnceLock::new();

/// `Board::AddZombieInRow` 的地址
pub const ADDR_ADD_ZOMBIE_IN_ROW: u32 = 0x0040DDC0;
/// `Board::AddZombieInRow` 的签名
type SignAddZombieInRow = extern "stdcall" fn(
    this: *mut Board,
    theZombieType: c_int,
    theRow: c_int,
    theFromWave: c_int,
) -> *mut Zombie;
/// `Board::AddZombieInRow` 的跳板
pub static ORIGINAL_ADD_ZOMBIE_IN_ROW: OnceLock<SignAddZombieInRow> = OnceLock::new();

/// 从 `usercall` 中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn AddZombieInRowHelper() {
    naked_asm!(
        // 写入 this
        "mov ecx, eax",
        "push 3",
        "call {stack_rotate}",
        "push ebx",

        // 调用 hook 函数
        "call {hook}",

        "ret",

        stack_rotate = sym stack_rotate,
        hook = sym board::AddZombieInRow,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn AddZombieInRowWrapper(
    this: *mut Board,
    theZombieType: c_int,
    theRow: c_int,
    theFromWave: c_int,
) -> *mut Zombie {
    // 获取原函数的指针
    let func = ORIGINAL_ADD_ZOMBIE_IN_ROW.wait();
    unsafe {
        asm!("push ebx");
        asm!(
            // 压参数
            "push {}",
            "push {}",
            in(reg) theRow,
            in(reg) theZombieType,
        );
        let zombie;
        asm!(
            // 调用原函数
            // 解指针获得真实地址
            "call dword ptr [{func}]",
            "mov {zombie}, eax",
            in("eax") this,
            in("ebx") theFromWave,
            func = in(reg) func,
            zombie = out(reg) zombie
        );
        asm!("pop ebx");
        zombie
    }
}

/// `Board::MouseDown` 的地址
pub const ADDR_MOUSE_DOWN: u32 = 0x00411F20;
/// `Board::KeyDown` 的签名
type SignMouseDown = extern "thiscall" fn(this: *mut Board, pos: Vec2<c_int>, theClickCount: c_int);
/// `Board::MouseDown` 的跳板
pub static ORIGINAL_MOUSE_DOWN: OnceLock<SignMouseDown> = OnceLock::new();

/// `Board::MouseUp` 的地址
pub const ADDR_MOUSE_UP: u32 = 0x00412540;
/// `Board::MouseUp` 的签名
type SignMouseUp = extern "thiscall" fn(this: *mut Board, pos: Vec2<c_int>, theClickCount: c_int);
/// `Board::MouseUp` 的跳板
pub static ORIGINAL_MOUSE_UP: OnceLock<SignMouseUp> = OnceLock::new();

/// `Board::Update` 的地址
pub const ADDR_UPDATE: u32 = 0x00415D40;
/// `Board::Update` 的签名
type SignUpdate = extern "thiscall" fn(this: *mut Board);
/// `Board::Update` 的跳板
pub static ORIGINAL_UPDATE: OnceLock<SignUpdate> = OnceLock::new();

/// `Board::PixelToGridX` 的地址
pub const ADDR_PIXEL_TO_GRID_X: u32 = 0x0041C4C0;

pub fn PixelToGridXWrapper(this: *mut Board, theX: c_int, theY: c_int) -> i32 {
    unsafe {
        let result;

        asm!(
            "mov edx, {func}",
            "call edx",

            in("edi") theY,
            in("eax") theX,
            in("ecx") this,

            func = const ADDR_PIXEL_TO_GRID_X,

            lateout("eax") result,
            clobber_abi("C")
        );

        result
    }
}

/// `Board::PixelToGridY` 的地址
pub const ADDR_PIXEL_TO_GRID_Y: u32 = 0x0041C550;

pub fn PixelToGridYWrapper(this: *mut Board, theX: c_int, theY: c_int) -> i32 {
    unsafe {
        let result;

        asm!(
            "call {func}",

            in("ecx") theY,
            in("eax") theX,
            in("edx") this,

            func = in(reg) ADDR_PIXEL_TO_GRID_Y,

            lateout("eax") result,
            clobber_abi("C")
        );

        result
    }
}

/// `Board::PixelToGridXKeepOnBoard` 的地址
pub const ADDR_PIXEL_TO_GRID_X_KEEP_ON_BOARD: u32 = 0x0041C530;

pub fn PixelToGridXKeepOnBoardWrapper(this: *mut Board, theX: c_int, theY: c_int) -> i32 {
    unsafe {
        let result;

        asm!(
            "push esi",
            "mov esi, {theX}",

            "mov edx, {func}",
            "call edx",

            "pop esi",

            in("eax") theY,
            theX = in(reg) theX,
            in("ebx") this,

            func = const ADDR_PIXEL_TO_GRID_X_KEEP_ON_BOARD,

            lateout("eax") result,
            clobber_abi("C")
        );

        result
    }
}

/// `Board::PixelToGridYKeepOnBoard` 的地址
pub const ADDR_PIXEL_TO_GRID_Y_KEEP_ON_BOARD: u32 = 0x0041C650;

pub fn PixelToGridYKeepOnBoardWrapper(this: *mut Board, theX: c_int, theY: c_int) -> i32 {
    unsafe {
        let result;

        asm!(
            "mov edx, {func}",
            "call edx",

            in("edi") theY,
            in("eax") theX,
            in("ebx") this,

            func = const ADDR_PIXEL_TO_GRID_Y_KEEP_ON_BOARD,

            lateout("eax") result,
            clobber_abi("C")
        );

        result
    }
}

/// `Board::Draw` 的地址
pub const ADDR_DRAW: u32 = 0x0041ACF0;
/// `Board::Draw` 的签名
type SignDraw = extern "thiscall" fn(this: *mut Board, g: *mut Graphics);
/// `Board::Draw` 的跳板
pub static ORIGINAL_DRAW: OnceLock<SignDraw> = OnceLock::new();

/// `LawnLoadGame` 的地址
const ADDR_LAWN_LOAD_GAME: u32 = 0x00481FE0 as _;
/// `LawnLoadGame` 的跳板
pub static ORIGINAL_LAWN_LOAD_GAME: AtomicUsize = AtomicUsize::new(0);

#[unsafe(naked)]
extern "stdcall" fn LawnLoadGameHelper() {
    naked_asm!(
        "push ebp",
        "mov ebp, esp",

        "push [ebp + 8]",
        "push ecx",
        "call {func}",

        "leave",
        "ret",

        func = sym board::profile::LawnLoadGame
    )
}

pub fn LawnLoadGameWrapper(this: *mut Board, theFilePath: *const MsvcString) -> bool {
    unsafe {
        let result: u32;
        asm!(
            "push {path}",
            "call [{func}]",
            "add esp, 4",
            path = in(reg) theFilePath,
            func = sym ORIGINAL_LAWN_LOAD_GAME,
            in("ecx") this,
            lateout("eax") result,
            clobber_abi("C"),
        );
        result != 0
    }
}

/// `LawnSaveGame` 的地址
const ADDR_LAWN_SAVE_GAME: u32 = 0x004820D0 as _;
/// `LawnSaveGame` 的跳板
pub static ORIGINAL_LAWN_SAVE_GAME: AtomicUsize = AtomicUsize::new(0);

#[unsafe(naked)]
extern "stdcall" fn LawnSaveGameHelper() {
    naked_asm!(
        "push ebp",
        "mov ebp, esp",

        "push [ebp + 8]",
        "push edi",
        "call {func}",

        "leave",
        "ret",

        func = sym board::profile::LawnSaveGame
    )
}

pub fn LawnSaveGameWrapper(this: *mut Board, theFilePath: *const MsvcString) -> bool {
    unsafe {
        let result: u32;
        asm!(
            "push {path}",
            "call [{func}]",
            "add esp, 4",
            path = in(reg) theFilePath,
            func = sym ORIGINAL_LAWN_SAVE_GAME,
            in("edi") this,
            lateout("eax") result,
            clobber_abi("C"),
        );
        result != 0
    }
}

/// `Board::GetPlantsOnLawn` 的函数地址
pub const ADDR_GET_PLANTS_ON_LAWN: u32 = 0x0040D2A0;
/// `Board::GetPlantsOnLawn` 的函数签名
type SignGetPlantsOnLawn =
    fn(this: *mut Board, thePlantOnLawn: *mut PlantsOnLawn, theGridPos: Vec2<c_int>);
static ORIGINAL_GET_PLANTS_ON_LAWN: OnceLock<SignGetPlantsOnLawn> = OnceLock::new();

#[unsafe(naked)]
extern "stdcall" fn GetPlantsOnLawnHelper() {
    naked_asm!(
        "mov ecx, edx",
        "push 3",
        "call {stack_rotate}",
        "push ebx",

        "call {hook}",

        "ret",

        stack_rotate = sym stack_rotate,
        hook = sym board::GetPlantsOnLawn
    )
}

pub fn GetPlantsOnLawnWrapper(
    this: *mut Board,
    thePlantOnLawn: *mut PlantsOnLawn,
    theGridPos: Vec2<c_int>,
) {
    unsafe {
        asm!(
            "push {theGridY}",
            "push {theGridX}",

            "call [{func}]",

            in("ebx") thePlantOnLawn,
            in("edx") this,
            theGridY = in(reg) theGridPos.y,
            theGridX = in(reg) theGridPos.x,
            func = in(reg) ORIGINAL_GET_PLANTS_ON_LAWN.wait(),
            clobber_abi("C")
        )
    }
}

/// `Board::KillAllZombiesInRadius` 的地址
pub const ADDR_KILL_ALL_ZOMBIES_IN_RADIUS: u32 = 0x0041D8A0;
/// `Board::KillAllZombiesInRadius` 的签名
type SignKillAllZombiesInRadius = extern "stdcall" fn(
    this: *mut Board,
    theRow: c_int,
    thePos: Vec2<c_int>,
    theRadius: c_int,
    theRowRange: c_int,
    theBurn: bool,
    theDamageRangeFlags: c_int,
);
/// `Board::KillAllZombiesInRadius` 的跳板
pub static ORIGINAL_KILL_ALL_ZOMBIES_IN_RADIUS: OnceLock<SignKillAllZombiesInRadius> =
    OnceLock::new();

/// `Board::AddPlant` 的地址
pub const ADDR_ADD_PLANT: u32 = 0x0040D120;
type SignAddPlant = extern "thiscall" fn(
    this: *mut Board,
    theGridPos: Vec2<c_int>,
    theSeedType: c_int,
    theImitaterType: c_int,
) -> *mut Plant;
pub static ORIGINAL_ADD_PLANT: OnceLock<SignAddPlant> = OnceLock::new();

#[unsafe(naked)]
extern "stdcall" fn AddPlantHelper() {
    naked_asm!(
        "push 5",
        "call {stack_rotate}",

        "pop ecx",
        "pop edx",
        "push eax",
        "push edx",

        "call {hook}",

        "ret",

        stack_rotate = sym stack_rotate,
        hook = sym board::AddPlant
    )
}

pub fn AddPlantWrapper(
    this: *mut Board,
    theGridPos: Vec2<c_int>,
    theSeedType: c_int,
    theImitaterType: c_int
) -> *mut Plant {
    unsafe {
        let result;

        asm!(
            "push {theImitaterType}",
            "push {theSeedType}",
            "push {theGridX}",
            "push {this}",

            "call [{func}]",

            in("eax") theGridPos.y,
            theImitaterType = in(reg) theImitaterType,
            theSeedType = in(reg) theSeedType,
            theGridX = in(reg) theGridPos.x,
            this = in(reg) this,
            func = in(reg) ORIGINAL_ADD_PLANT.wait(),
            lateout("eax") result,
            clobber_abi("C")
        );

        result
    }
}

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_CONSTRUCTOR.set(
            hook(ADDR_CONSTRUCTOR as _, board::Constructor as _)?
        );

        let _ = ORIGINAL_DESTRUCTOR.set(
            hook(ADDR_DESTRUCTOR as _, board::Destructor as _)?
        );

        let _ = ORIGINAL_INIT_LEVEL.set(
            hook(ADDR_INIT_LEVEL as _, board::InitLevel as _)?
        );

        let _ = ORIGINAL_ADDCOIN.set(
            hook(ADDR_ADDCOIN as _, board::AddCoin as _)?
        );

        let _ = ORIGINAL_KEYDOWN.set(
            hook(ADDR_KEYDOWN as _, board::KeyDown as _)?
        );

        let _ = ORIGINAL_ADD_ZOMBIE_IN_ROW.set(
            hook(ADDR_ADD_ZOMBIE_IN_ROW as _, AddZombieInRowHelper as _)?
        );

        let _ = ORIGINAL_MOUSE_DOWN.set(
            hook(ADDR_MOUSE_DOWN as _, board::MouseDown as _)?
        );

        let _ = ORIGINAL_MOUSE_UP.set(
            hook(ADDR_MOUSE_UP as _, board::MouseUp as _)?
        );

        let _ = ORIGINAL_UPDATE.set(
            hook(ADDR_UPDATE as _, board::Update as _)?
        );

        let _ = ORIGINAL_DRAW.set(
            hook(ADDR_DRAW as _, board::Draw as _)?
        );

        ORIGINAL_LAWN_LOAD_GAME.store(
            hook(ADDR_LAWN_LOAD_GAME as _, LawnLoadGameHelper as _)?, Ordering::SeqCst
        );

        ORIGINAL_LAWN_SAVE_GAME.store(
            hook(ADDR_LAWN_SAVE_GAME as _, LawnSaveGameHelper as _)?, Ordering::SeqCst
        );

        let _ = ORIGINAL_GET_PLANTS_ON_LAWN.set(
            hook(ADDR_GET_PLANTS_ON_LAWN as _, GetPlantsOnLawnHelper as _)?
        );

        let _ = ORIGINAL_KILL_ALL_ZOMBIES_IN_RADIUS.set(
            hook(ADDR_KILL_ALL_ZOMBIES_IN_RADIUS as _, board::KillAllZombiesInRadius as _)?
        );

        let _ = ORIGINAL_ADD_PLANT.set(
            hook(ADDR_ADD_PLANT as _, AddPlantHelper     as _)?
        );

        Ok(())
    })
}
