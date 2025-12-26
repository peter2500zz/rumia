use std::{
    arch::{asm, naked_asm},
    sync::OnceLock,
};

use crate::{
    hook::{HookRegistration, hook},
    pvz::plant::{self, plant::Plant},
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
        "mov edx, [esp+12]",
        "xchg edx, [esp+8]",
        "xchg edx, [esp+4]",
        "xchg edx, [esp]",
        "mov [esp+12], edx",
        "push eax",
        "push ecx",

        "call {hook}",

        "ret",

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

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_PLANT_INITIALIZE.set(
            hook(ADDR_PLANT_INITIALIZE as _, PlantInitializeHelper as _)?
        );

        Ok(())
    })
}
