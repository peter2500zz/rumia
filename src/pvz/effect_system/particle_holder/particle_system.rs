use std::ffi::{c_float, c_int};

use crate::{
    hook::pvz::effect_system::particle_holder::AllocParticleSystemWrapper,
    pvz::effect_system::particle_holder::ParticleHolder, utils::Vec2,
};

#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystem;

pub extern "thiscall" fn AllocParticleSystem(
    this: *mut ParticleHolder,
    thePos: Vec2<c_float>,
    theRenderOrder: c_int,
    theParticleEffect: c_int,
) -> *mut ParticleSystem {
    tracing::trace!("alloc new particle at {:?}", thePos);
    AllocParticleSystemWrapper(this, thePos, theRenderOrder, theParticleEffect)
}
