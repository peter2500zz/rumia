use tracing::trace;

use crate::{hook::pvz::plant::PlantInitializeWrapper, pvz::plant::plant::Plant, utils::data_array::HasId};

pub mod plant;

pub extern "stdcall" fn PlantInitialize(
    theGridX: i32,
    theGridY: i32,
    this: *mut Plant,
    theSeedType: i32,
    theImitaterType: i32,
) {
    trace!(
        "植物 {} 初始化 at ({}, {})",
        theSeedType, theGridX, theGridY
    );
    PlantInitializeWrapper(this, theGridX, theGridY, theSeedType, theImitaterType);

    unsafe {
        trace!("{}", (*this).id());
    }
}
