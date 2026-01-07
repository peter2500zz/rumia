pub mod plant;
pub mod lua;

use std::arch::naked_asm;
use tracing::trace;

use crate::{
    hook::pvz::plant::{ADDR_FIRE_WITHOUT_TARGET, ORIGINAL_FIRE, PlantInitializeWrapper},
    pvz::{plant::plant::Plant, zombie::zombie::Zombie},
};

pub extern "stdcall" fn PlantInitialize(
    theGridX: i32,
    theGridY: i32,
    this: *mut Plant,
    theSeedType: i32,
    theImitaterType: i32,
) {
    trace!(
        "plant {} initialized at ({}, {})",
        theSeedType, theGridX, theGridY
    );
    PlantInitializeWrapper(this, theGridX, theGridY, theSeedType, theImitaterType);

    unsafe {
        trace!("{}", (*this).plant_subtype);
    }
}

pub extern "stdcall" fn Fire(
    this: *mut Plant,
    theTargetZombie: *mut Zombie,
    theRow: i32,
    thePlantWeapon: i32,
) {
    ORIGINAL_FIRE.wait()(this, theTargetZombie, theRow, thePlantWeapon)
}

#[unsafe(naked)]
pub extern "thiscall" fn FireWithoutTarget(this: *mut Plant, theRow: i32, thePlantWeapon: i32) {
    naked_asm!(
        "mov eax, ecx",
        "push ecx",
        "push ebx",
        "push ebp",
        "push esi",
        "mov esi, eax",
        "push edi",
        "mov edx, {origin}",
        "jmp edx",

        origin = const ADDR_FIRE_WITHOUT_TARGET
    )
}
