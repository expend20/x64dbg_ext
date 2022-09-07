//
// The high level interface for x64dbg
//
pub mod colors;
pub mod common;
pub mod telescope;
pub mod unicorn;

use crate::common::primitive::c_char_to_string;
use std::ffi::c_void;
use std::os::raw::c_char;
use x64dbg_sdk_sys::{
    _plugin_registercallback, _plugin_registercommand, CBTYPE_CB_STEPPED,
    PLUG_INITSTRUCT, PLUG_SDKVERSION, PLUG_SETUPSTRUCT,
};

static mut PLUGIN_HANDLE: i32 = 0;
static NAME: &str = "Dbg expy";

#[no_mangle]
pub unsafe extern "C" fn pluginit(init: *mut PLUG_INITSTRUCT) -> bool {
    (*init).pluginVersion = 1;
    (*init).sdkVersion = PLUG_SDKVERSION as i32;
    let name = (*init).pluginName.as_ptr() as *mut u8;
    name.copy_from(NAME.as_ptr(), NAME.len());
    PLUGIN_HANDLE = (*init).pluginHandle;
    colors::print_test_colors();
    colors::init_colors();
    logprintln!("{} init ok", NAME);
    true
}

#[no_mangle]
pub unsafe extern "C" fn plugsetup(_setup_struct: *const PLUG_SETUPSTRUCT) {
    for v in vec![dbs!("telescope"), dbs!("tele")] {
        _plugin_registercommand(PLUGIN_HANDLE, v, Some(cb_telescope), true);
    }

    for v in vec![dbs!("unicorn"), dbs!("uni")] {
        _plugin_registercommand(PLUGIN_HANDLE, v, Some(cb_unicorn), true);
    }

    for v in vec![dbs!("context"), dbs!("ctx")] {
        _plugin_registercommand(PLUGIN_HANDLE, v, Some(cb_context), true);
    }

    _plugin_registercallback(
        PLUGIN_HANDLE,
        CBTYPE_CB_STEPPED,
        Some(cb_stepped),
    );
}

unsafe extern "C" fn cb_stepped(_cb_type: i32, _info: *mut c_void) {
    common::print_context();
}

unsafe extern "C" fn cb_telescope(argc: i32, argsv: *mut *mut c_char) -> bool {
    //
    // Main entrace for telescope, we parse the arguments
    //
    if argc < 2 {
        logprintln!(
            "Usage: {} {{address}}[,size in hex][,recursion count]",
            c_char_to_string(*argsv.offset(0))
        );
        return false;
    }

    //
    // Extract the address and size and recursion count
    //
    let addr = common::c_char_to_val(*argsv.offset(1));
    if addr == 0 {
        logprintln!("Invalid address");
        return false;
    }
    let mut sz = 0x10; // default size
    if argc > 2 {
        let r = common::c_char_to_val(*argsv.offset(2));
        if r != 0 {
            sz = r;
        }
    }
    let mut recursion_count = 8 as u32;
    if argc > 3 {
        let r = common::c_char_to_val(*argsv.offset(3));
        if r != 0 {
            recursion_count = r as u32;
        }
    }

    //
    // Call the telescope function
    //
    telescope::telescope(addr, sz, recursion_count);
    true
}

unsafe extern "C" fn cb_unicorn(argc: i32, argsv: *mut *mut c_char) -> bool {
    //
    // Main entrace for telescope, we parse the arguments
    //
    if argc < 2 {
        logprintln!(
            "Usage: {} {{address}},[size]",
            c_char_to_string(*argsv.offset(0))
        );
        return false;
    }

    //
    // Extract the address and size
    //
    let addr = common::c_char_to_val(*argsv.offset(1));
    if addr == 0 {
        logprintln!("Invalid address");
        return false;
    }
    let mut sz = 0x10; // default size
    if argc > 2 {
        let r = common::c_char_to_val(*argsv.offset(2));
        if r != 0 {
            sz = r;
        }
    }

    //
    // Call the unicorn function
    //
    unicorn::unicorn(addr, sz);
    true
}

unsafe extern "C" fn cb_context(_argc: i32, _argsv: *mut *mut c_char) -> bool {
    common::print_context();
    true
}
#[no_mangle]
pub extern "stdcall" fn plugstop() -> bool {
    true
}
