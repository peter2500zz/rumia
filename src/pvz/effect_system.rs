pub mod particle_holder;

use crate::{pvz::effect_system::particle_holder::ParticleHolder, utils::data_array::DataArray};

#[repr(C)]
#[derive(Debug)]
pub struct EffectSystem {
    /// 0x0 粒子系统
    pub particle: *mut ParticleHolder,
    /// 0x4 轨迹信息
    pub trails: *mut DataArray<()>,
    /// 0x8 动画对象
    pub reanims: *mut DataArray<()>,
    /// 0xC 动画附件
    pub attach: *mut DataArray<()>,

    _pad_0x10_0x7C: [u8; 0x7C - 0x10],
}
const _: () = assert!(size_of::<EffectSystem>() == 0x7C);
