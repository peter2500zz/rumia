use std::{ffi::c_void, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::LawnApp;

/// `LawnApp` 构造函数的地址
const ADDR_LAWNAPP_CONSTRUCTOR: *mut c_void = 0x0044EAA0 as _;
/// `LawnApp` 构造函数的签名
type SignLawnAppConstructor =
    extern "stdcall" fn(uninit: *mut LawnApp::LawnApp) -> *mut LawnApp::LawnApp;
/// `LawnApp` 构造函数的跳板
pub static ORIGINAL_LAWNAPP_CONSTRUCTOR: OnceLock<SignLawnAppConstructor> = OnceLock::new();

/// `LawnApp` 析构函数的地址
const ADDR_LAWNAPP_DESTRUCTOR: *mut c_void = 0x0044EDF0 as _;
/// `LawnApp` 析构函数的签名
type SignLawnAppDestructor = extern "thiscall" fn(this: *mut LawnApp::LawnApp);
/// `LawnApp` 析构函数的跳板
pub static ORIGINAL_LAWNAPP_DESTRUCTOR: OnceLock<SignLawnAppDestructor> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_LAWNAPP_CONSTRUCTOR.set(
            hook(ADDR_LAWNAPP_CONSTRUCTOR, LawnApp::Constructor as _)?
        );

        let _ = ORIGINAL_LAWNAPP_DESTRUCTOR.set(
            hook(ADDR_LAWNAPP_DESTRUCTOR, LawnApp::Destructor as _)?
        );

        Ok(())
    })
}
