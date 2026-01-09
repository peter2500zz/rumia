use std::ffi::{c_float, c_int};

use crate::{
    hook::pvz::effect_system::particle_holder::AllocParticleSystemWrapper, mods::LuaRegistration, pvz::effect_system::particle_holder::ParticleHolder, utils::Vec2
};

#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystem;

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();

        // 粒子效果
        let particles = lua.create_table()?;

        particles.set("DOOM", 30)?; // 蘑菇云

        globals.set("Particles", particles)?;

        Ok(())
    })
}

pub extern "thiscall" fn AllocParticleSystem(
    this: *mut ParticleHolder,
    thePos: Vec2<c_float>,
    theRenderOrder: c_int,
    theParticleEffect: c_int,
) -> *mut ParticleSystem {
    tracing::trace!("alloc new particle at {:?}", thePos);
    AllocParticleSystemWrapper(this, thePos, theRenderOrder, theParticleEffect)
}
