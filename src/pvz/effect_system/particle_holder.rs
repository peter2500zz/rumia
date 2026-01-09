pub mod particle_system;

use crate::{
    pvz::effect_system::particle_holder::particle_system::ParticleSystem,
    utils::data_array::DataArray,
};

#[repr(C)]
#[derive(Debug)]
pub struct ParticleHolder {
    /// 0x00 粒子系统 (+0x2C next)
    pub systems: DataArray<ParticleSystem>,

    /// 0x1C 发射源 (+0xB0 next)
    pub emitters: DataArray<()>,

    /// 0x38 粒子 (+0xA0 next)
    pub particles: DataArray<()>,

    pub _pad_0x54_0x7C: [u8; 0x7C - 0x54],
}
const _: () = assert!(size_of::<ParticleHolder>() == 0x7C);

// pub extern "thiscall" fn AllocParticleSystem(
//     this: *mut ParticleHolder,
//     theX: c_float,
//     theY: c_float,
//     theRenderOrder: c_int,
//     theParticleEffect: c_void,
// ) -> *mut ParticleSystem {

// }
