use crate::utils::data_array::DataArray;

#[repr(C)]
#[derive(Debug)]
pub struct ParticleHolder {
    /// 0x00 粒子系统 (+0x2C next)
    pub systems: DataArray<()>,

    /// 0x1C 发射源 (+0xB0 next)
    pub emitters: DataArray<()>,

    /// 0x38 粒子 (+0xA0 next)
    pub particles: DataArray<()>,

    pub _pad_0x54_0x7C: [u8; 0x7C - 0x54],
}
const _: () = assert!(size_of::<ParticleHolder>() == 0x7C);
