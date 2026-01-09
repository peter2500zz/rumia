pub mod lua;
pub mod this;

use std::{arch::naked_asm, ffi::c_int};
use tracing::trace;

use crate::{
    hook::pvz::plant::{ADDR_FIRE_WITHOUT_TARGET, ORIGINAL_FIRE, PlantInitializeWrapper},
    pvz::{plant::this::Plant, zombie::this::Zombie},
    utils::Vec2,
};

pub extern "stdcall" fn PlantInitialize(
    theGridPos: Vec2<c_int>,
    this: *mut Plant,
    theSeedType: c_int,
    theImitaterType: c_int,
) {
    trace!("plant {} initialized at {:?}", theSeedType, theGridPos);
    PlantInitializeWrapper(this, theGridPos, theSeedType, theImitaterType);

    unsafe {
        trace!("{}", (*this).plant_subtype);
    }
}

pub extern "stdcall" fn Fire(
    this: *mut Plant,
    theTargetZombie: *mut Zombie,
    theRow: c_int,
    thePlantWeapon: c_int,
) {
    ORIGINAL_FIRE.wait()(this, theTargetZombie, theRow, thePlantWeapon)
}

#[unsafe(naked)]
pub extern "thiscall" fn FireWithoutTarget(this: *mut Plant, theRow: c_int, thePlantWeapon: c_int) {
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
