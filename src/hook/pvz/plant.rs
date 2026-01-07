use std::{
    arch::{asm, naked_asm},
    sync::OnceLock,
};

use crate::{
    hook::{HookRegistration, hook},
    pvz::{
        plant::{self, plant::Plant},
        zombie::zombie::Zombie,
    },
    utils::asm::stack_rotate,
};

/// `Plant::PlantInitialize` 的函数地址
pub const ADDR_PLANT_INITIALIZE: u32 = 0x0045DB60;
/// `Plant::PlantInitialize` 的函数签名
type SignPlantInitialize =
    fn(this: *mut Plant, theGridX: i32, theGridY: i32, theSeedType: i32, theImitaterType: i32);
static ORIGINAL_PLANT_INITIALIZE: OnceLock<SignPlantInitialize> = OnceLock::new();

/// 一次过 哦耶
#[unsafe(naked)]
extern "stdcall" fn PlantInitializeHelper() {
    naked_asm!(
        "push 4",
        "call {stack_rotate}",

        "push eax",
        "push ecx",

        "call {hook}",

        "ret",

        stack_rotate = sym stack_rotate,
        hook = sym plant::PlantInitialize
    )
}

pub fn PlantInitializeWrapper(
    this: *mut Plant,
    theGridX: i32,
    theGridY: i32,
    theSeedType: i32,
    theImitaterType: i32,
) {
    unsafe {
        asm!(
            "push {theImitaterType}",
            "push {theSeedType}",
            "push {this}",

            "call [{func}]",

            in("eax") theGridY,
            in("ecx") theGridX,
            theImitaterType = in(reg) theImitaterType,
            theSeedType = in(reg) theSeedType,
            this = in(reg) this,
            func = in(reg) ORIGINAL_PLANT_INITIALIZE.wait(),
            clobber_abi("C")
        )
    }
}

/// `Plant::Fire` 的函数地址
pub const ADDR_FIRE: u32 = 0x00466E00;
/// `Plant::Fire` 的函数签名
type SignFire = extern "stdcall" fn(
    this: *mut Plant,
    theTargetZombie: *mut Zombie,
    theRow: i32,
    thePlantWeapon: i32,
);
/// `Plant::Fire` 的函数跳板
pub static ORIGINAL_FIRE: OnceLock<SignFire> = OnceLock::new();

/// 跳过了目标检测的 `Plant::FindTargetAndFire` 地址
pub const ADDR_FIRE_WITHOUT_TARGET: u32 = 0x0045EF38;

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_PLANT_INITIALIZE.set(
            hook(ADDR_PLANT_INITIALIZE as _, PlantInitializeHelper as _)?
        );

        let _ = ORIGINAL_FIRE.set(
            hook(ADDR_FIRE as _, plant::Fire as _)?
        );

        Ok(())
    })
}
