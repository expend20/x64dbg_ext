use crate::common::primitive::str_to_vec;
use crate::*; // needed for logprintln macro
use std::ffi::c_void;
use std::ffi::{CStr, CString};

use x64dbg_sdk_sys::{
    duint, BridgeSettingGet, DbgEval, DbgFunctions, DbgGetLabelAt,
    DbgGetModuleAt, DbgGetRegDumpEx, DbgGetStringAt, DbgGetTebAddress,
    DbgGetThreadId, DbgMemRead, GuiAddInfoLine, _plugin_logprintf,
    _plugin_logputs, _plugin_lograw_html, MAX_MODULE_SIZE, MAX_SETTING_SIZE,
    MAX_STRING_SIZE, REGDUMP, SEGMENTREG,
};

pub fn plg_logputs(s: &str) {
    let s = format!("{}\0", s);
    unsafe { _plugin_logputs(s.as_ptr() as *const _) };
}

pub fn plg_logprintf(s: &str) {
    let s = format!("{}\0", s);
    unsafe { _plugin_logprintf(s.as_ptr() as *const _) };
}

pub fn plg_lograw_html(s: &str) {
    let s = format!("{}\0", s);
    unsafe { _plugin_lograw_html(s.as_ptr() as *const _) };
}

pub fn brdg_setting_get(section: &str, name: &str) -> String {
    let mut val = vec![0 as u8; MAX_SETTING_SIZE as usize];
    let s_section = str_to_vec(section);
    let s_name = str_to_vec(name);
    let r = unsafe {
        BridgeSettingGet(
            s_section.as_ptr() as *const _,
            s_name.as_ptr() as *const _,
            val.as_mut_ptr() as *mut _,
        )
    };
    if !r {
        return String::new();
    }
    let s = unsafe { CStr::from_ptr(val.as_ptr() as *mut _) };
    s.to_string_lossy().to_string()
}

pub fn dbg_eval(s: &str) -> (bool, usize) {
    let mut r = false;
    let s = CString::new(s);
    if s.is_err() {
        return (r, 0);
    }
    let s = s.unwrap();
    let val = unsafe { DbgEval(s.as_ptr(), &mut r) };
    (r, val as usize)
}

pub fn dbg_get_string_at(addr: usize) -> String {
    let mut val = vec![0 as u8; MAX_STRING_SIZE as usize];
    let r =
        unsafe { DbgGetStringAt(addr as duint, val.as_mut_ptr() as *mut _) };
    if !r {
        return String::new();
    }
    let s = unsafe { CStr::from_ptr(val.as_ptr() as *mut _) };
    s.to_string_lossy().to_string()
}

pub fn dbg_get_module_at(addr: usize) -> String {
    let mut val = vec![0 as u8; MAX_MODULE_SIZE as usize];
    let r =
        unsafe { DbgGetModuleAt(addr as duint, val.as_mut_ptr() as *mut _) };
    if !r {
        return String::new();
    }
    let s = unsafe { CStr::from_ptr(val.as_ptr() as *mut _) };
    s.to_string_lossy().to_string()
}

pub fn dbg_get_label_at(addr: usize, reg: SEGMENTREG) -> String {
    let mut val = vec![0 as u8; MAX_MODULE_SIZE as usize];
    let r = unsafe {
        DbgGetLabelAt(addr as duint, reg, val.as_mut_ptr() as *mut _)
    };
    if !r {
        return String::new();
    }
    let s = unsafe { CStr::from_ptr(val.as_ptr() as *mut _) };
    s.to_string_lossy().to_string()
}

pub fn dbg_funcs_section_from_addr(addr: usize) -> String {
    let mut val = vec![0 as u8; MAX_MODULE_SIZE as usize];
    let r = unsafe {
        DbgFunctions().as_ref().unwrap().SectionFromAddr.unwrap()(
            addr as duint,
            val.as_mut_ptr() as *mut _,
        )
    };
    if !r {
        return String::new();
    }
    let s = unsafe { CStr::from_ptr(val.as_ptr() as *mut _) };
    s.to_string_lossy().to_string()
}

pub fn gui_add_info_line(s: &str) {
    let s = format!("{}\0", s);
    unsafe { GuiAddInfoLine(s.as_ptr() as *const _) };
}

pub fn dbg_mem_read_ptr(addr: usize) -> (bool, usize) {
    let mut val = 0 as duint;
    let r = unsafe {
        DbgMemRead(
            addr as duint,
            &mut val as *mut _ as *mut c_void,
            std::mem::size_of::<usize>() as duint,
        )
    };
    (r, val as usize)
}

pub fn dbg_mem_read(addr: usize, sz: usize) -> (bool, Vec<u8>) {
    let mut v = vec![0 as u8; sz];
    let r = unsafe {
        DbgMemRead(
            addr as duint,
            v.as_mut_ptr() as *mut _ as *mut c_void,
            sz as duint,
        )
    };
    (r, v)
}

pub fn dbg_get_regs() -> REGDUMP {
    // get size of regdump struct
    let mut regs: REGDUMP = unsafe { std::mem::zeroed() };

    let r = unsafe {
        DbgGetRegDumpEx(
            &mut regs as *mut _,
            std::mem::size_of::<REGDUMP>() as duint,
        )
    };
    if !r {
        logprintln!("Can't get registers context");
    }
    regs
}

pub fn dbg_get_thread_id() -> u32 {
    unsafe { DbgGetThreadId() as u32 }
}
pub fn dbg_get_teb_address(id: u32) -> usize {
    unsafe { DbgGetTebAddress(id) as usize }
}
