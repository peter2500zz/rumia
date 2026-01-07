use std::{
    arch::{asm, naked_asm},
    sync::OnceLock,
};
use windows::core::BOOL;

use super::{HookRegistration, hook};
use crate::{
    pvz::{graphics::graphics::Graphics, zombie::{self, zombie::Zombie}},
    utils::{asm::stack_rotate, data_array::DataArray},
};

/// `DataArray<Zombie>::DataArrayAlloc` 函数的地址
const ADDR_DATA_ARRAY_ALLOC: u32 = 0x0041DDA0;
/// `DataArray<Zombie>::DataArrayAlloc` 函数的签名
type SignDataArrayAlloc = fn(this: *mut DataArray<Zombie>) -> *mut Zombie;
/// `DataArray<Zombie>::DataArrayAlloc` 函数的跳板
static ORIGINAL_DATA_ARRAY_ALLOC: OnceLock<SignDataArrayAlloc> = OnceLock::new();

/// 从非标准调用中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn DataArrayAllocHelper() {
    naked_asm!(
        // 压栈 esi 作为参数
        "push esi",
        // 调用 hook 函数
        "call {hook}",
        // 返回
        "ret",

        // 传入 hook 函数符号地址
        hook = sym zombie::DataArrayAlloc,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn DataArrayAllocWrapper(this: *mut DataArray<Zombie>) -> *mut Zombie {
    unsafe {
        let result;
        asm!(
            // 保存 esi
            "push esi",
            // 把参数放入原函数期望的寄存器中
            "mov esi, {this}",
            // 调用原函数
            "call [{func}]",
            // 恢复 esi
            "pop esi",
            // 提取返回值
            "mov {result}, eax",

            this = in(reg) this,
            func = in(reg) ORIGINAL_DATA_ARRAY_ALLOC.wait(),
            result = out(reg) result,
        );
        result
    }
}

/// `Zombie::ZombieInitialize` 的地址
pub const ADDR_ZOMBIE_INITIALIZE: u32 = 0x00522580;
/// `Zombie::ZombieInitialize` 的签名
///
/// 仅标注用
type SignZombieInitialize = fn(
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
);
/// `Zombie::ZombieInitialize` 的跳板
static ORIGINAL_ZOMBIE_INITIALIZE: OnceLock<SignZombieInitialize> = OnceLock::new();

/// 从 `usercall` 中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn ZombieInitializeHelper() {
    naked_asm!(
        "push 6",
        "call {stack_rotate}",
        "pop ecx",
        "push eax",

        // 调用 hook 函数
        "call {hook}",

        "ret",

        stack_rotate = sym stack_rotate,
        hook = sym zombie::ZombieInitialize,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn ZombieInitializeWrapper(
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
) {
    // 获取原函数的指针
    let func = ORIGINAL_ZOMBIE_INITIALIZE.wait();
    unsafe {
        asm!(
            // 压参数
            "push {}",
            "push {}",
            "push {}",
            "push {}",
            "push {}",
            in(reg) theFromWave,
            in(reg) theParentZombie,
            in(reg) theVariant.0,
            in(reg) theZombieType,
            in(reg) this,
        );
        asm!(
            // 调用原函数
            // 解指针获得真实地址
            "call dword ptr [{func}]",
            in("eax") theRow,
            func = in(reg) func,
        );
    }
}

/// `Zombie::Update` 构造函数的地址
pub const ADDR_UPDATE: u32 = 0x0052AE60;
/// `Zombie::Update` 构造函数的签名
type SignUpdate = fn(this: *mut Zombie);
/// `Zombie::Update` 构造函数的跳板
static ORIGINAL_UPDATE: OnceLock<SignUpdate> = OnceLock::new();

/// 从非标准调用中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn UpdateHelper() {
    naked_asm!(
        // 压栈 eax 作为参数
        "push eax",
        // 调用 hook 函数
        "call {hook}",
        // 返回
        "ret",

        // 传入 hook 函数符号地址
        hook = sym zombie::Update,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn UpdateWrapper(this: *mut Zombie) {
    unsafe {
        asm!(
            // 把参数放入原函数期望的寄存器中
            "mov eax, {this}",
            // 调用原函数
            "call [{func}]",

            this = in(reg) this,
            func = in(reg) ORIGINAL_UPDATE.wait(),
        );
    }
}

/// `Zombie::Draw` 构造函数的地址
pub const ADDR_DRAW: u32 = 0x0052E2E0;
/// `Zombie::Draw` 构造函数的签名
type SignDraw = fn(
    this: *mut Zombie,
    g: *mut Graphics,
);
/// `Zombie::Draw` 构造函数的跳板
static ORIGINAL_DRAW: OnceLock<SignDraw> = OnceLock::new();

/// 从非标准调用中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn DrawHelper(
    g: *mut Graphics
) {
    naked_asm!(
        "push [esp + 4]",
        "push ebx",
        // 调用 hook 函数
        "call {hook}",
        // 返回
        "ret 4",

        // 传入 hook 函数符号地址
        hook = sym zombie::Draw,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn DrawWrapper(
    this: *mut Zombie,
    g: *mut Graphics
) {
    unsafe {
        asm!(
            // 把参数放入原函数期望的寄存器中
            "push {g}",
            "mov ebx, {this}",
            // 调用原函数
            "call [{func}]",

            this = in(reg) this,
            g = in(reg) g,
            func = in(reg) ORIGINAL_DRAW.wait(),
        );
    }
}

/// `Zombie::DieNoLoot` 的地址
pub const ADDR_DIE_NO_LOOT: u32 = 0x00530510;
/// `Zombie::DieNoLoot` 的签名
type SignDieNoLoot = extern "thiscall" fn(this: *mut Zombie);
/// `Zombie::DieNoLoot` 的跳板
pub static ORIGINAL_DIE_NO_LOOT: OnceLock<SignDieNoLoot> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_DATA_ARRAY_ALLOC.set(
            hook(ADDR_DATA_ARRAY_ALLOC as _, DataArrayAllocHelper as _)?
        );

        let _ = ORIGINAL_ZOMBIE_INITIALIZE.set(
            hook(ADDR_ZOMBIE_INITIALIZE as _, ZombieInitializeHelper as _)?
        );

        let _ = ORIGINAL_UPDATE.set(
            hook(ADDR_UPDATE as _, UpdateHelper as _)?
        );

        let _ = ORIGINAL_DRAW.set(
            hook(ADDR_DRAW as _, DrawHelper as _)?
        );

        let _ = ORIGINAL_DIE_NO_LOOT.set(
            hook(ADDR_DIE_NO_LOOT as _, zombie::DieNoLoot as _)?
        );

        Ok(())
    })
}
