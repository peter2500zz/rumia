use std::ffi::c_int;

use tracing::trace;

use crate::{
    hook::pvz::lawn_app::sound::ORIGINAL_PLAY_SAMPLE, mods::LuaRegistration,
    pvz::lawn_app::this::LawnApp,
};

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();
        let sounds = lua.create_table()?;

        // 毁灭蘑爆炸
        sounds.set("DOOM", 100)?;

        globals.set("GameSounds", sounds)?;

        Ok(())
    })
}

pub extern "thiscall" fn PlaySample(this: *mut LawnApp, theSoundNum: c_int) {
    trace!("play sound {}", theSoundNum);
    ORIGINAL_PLAY_SAMPLE.wait()(this, theSoundNum)
}
