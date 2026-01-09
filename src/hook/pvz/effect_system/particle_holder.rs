use std::{
    arch::{asm, naked_asm},
    ffi::{c_float, c_int},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    hook::{HookRegistration, hook},
    pvz::effect_system::particle_holder::{
        ParticleHolder,
        particle_system::{self, ParticleSystem},
    },
    utils::{Vec2, asm::stack_rotate},
};

pub const ADDR_ALLOC_PARTICLE_SYSTEM: u32 = 0x00518A70;
static ORIGINAL_ALLOC_PARTICLE_SYSTEM: AtomicUsize = AtomicUsize::new(0);

#[unsafe(naked)]
extern "stdcall" fn AllocParticleSystemHelper() {
    naked_asm!(
        "push 5",
        "call {stack_rotate}",

        "mov ecx, esi",

        "call {func}",

        "ret",

        stack_rotate = sym stack_rotate,
        func = sym particle_system::AllocParticleSystem
    )
}

pub fn AllocParticleSystemWrapper(
    this: *mut ParticleHolder,
    thePos: Vec2<c_float>,
    theRenderOrder: c_int,
    theParticleEffect: c_int,
) -> *mut ParticleSystem {
    unsafe {
        let result;

        asm!(
            "push {theParticleEffect}",
            "push {theRenderOrder}",
            "push {theY}",
            "push {theX}",
            "mov esi, {this}",

            "call [{func}]",

            theParticleEffect = in(reg) theParticleEffect,
            theRenderOrder = in(reg) theRenderOrder,
            theY = in(reg) thePos.y,
            theX = in(reg) thePos.x,
            this = in(reg) this,

            func = sym ORIGINAL_ALLOC_PARTICLE_SYSTEM,

            lateout("eax") result,
            clobber_abi("C"),
        );

        result
    }
}

inventory::submit! {
    HookRegistration(|| {
        ORIGINAL_ALLOC_PARTICLE_SYSTEM.store(
            hook(ADDR_ALLOC_PARTICLE_SYSTEM as _, AllocParticleSystemHelper as _)?, Ordering::Relaxed
        );

        Ok(())
    })
}
