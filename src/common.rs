pub mod disasm;
pub mod primitive;
pub mod unsafe_wrapper;

use core::num::ParseIntError;
use std::ffi::c_char;

use crate::colors;
use crate::common::primitive::c_char_to_string;
use crate::common::unsafe_wrapper as uw;
use crate::telescope;
use crate::unicorn;

use x64dbg_sdk_sys::SEGMENTREG_SEG_DEFAULT;

#[macro_export]
macro_rules! logprintln {
    ($($arg:tt)*) => {{
        crate::common::unsafe_wrapper::plg_logputs(&format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! logprint {
    ($($arg:tt)*) => {{
        crate::common::unsafe_wrapper::plg_logprintf(&format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! lograw {
    ($($arg:tt)*) => {{
        crate::common::unsafe_wrapper::plg_lograw_html(&format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! dbs {
    ($str:expr) => {
        concat!($str, "\0").as_ptr() as *const c_char
    };
}

pub fn from_hex(s: &str) -> Result<usize, ParseIntError> {
    let s = s.trim_start_matches("0x").to_string();
    let r = usize::from_str_radix(s.as_str(), 16);
    r
}

pub fn from_hex_or_eval(s: &str) -> usize {
    //
    // Try hex string first
    //
    let r = from_hex(s);
    if r.is_ok() {
        r.unwrap()
    } else {
        //
        // use DbgEval to resolve address
        //
        let (_, v) = uw::dbg_eval(&s);
        v as usize
    }
}

pub fn c_char_to_val(s: *const c_char) -> usize {
    let s = c_char_to_string(s);
    from_hex_or_eval(&s)
}

pub fn print_regs() {
    let regs = uw::dbg_get_regs();
    let r = regs.regcontext;
    let vv = vec![
        (r.cax, "rax"),
        (r.ccx, "rcx"),
        (r.cdx, "rdx"),
        (r.cbx, "rbx"),
        (r.csp, "rsp"),
        (r.cbp, "rbp"),
        (r.csi, "rsi"),
        (r.cdi, "rdi"),
        (r.r8, "r8"),
        (r.r9, "r9"),
        (r.r10, "r10"),
        (r.r11, "r11"),
        (r.r12, "r12"),
        (r.r13, "r13"),
        (r.r14, "r14"),
        (r.r15, "r15"),
        (r.cip, "rip"),
    ];
    for (v, n) in vv {
        let mut t = String::new();
        telescope::telescope_line(v as usize, 8, &mut t);
        lograw!(
            "{} {} {}<br>",
            colors::a(&format!("{:3}", n), colors::get_color(1), ""),
            colors::a("=", colors::get_color_comment(), ""),
            t
        );
    }
}

pub fn print_stack() {
    let (r, rsp) = uw::dbg_eval("rsp");
    if !r {
        logprintln!("rsp not found");
        return;
    }
    telescope::telescope(rsp as usize, 10, 8);
}

pub fn print_emu() {
    let (r, rip) = uw::dbg_eval("rip");
    if !r {
        logprintln!("rip not found");
        return;
    }
    unicorn::unicorn(rip, 15);
}

pub fn print_context() {
    lograw!(
        "{}<br>",
        colors::a("--- Registers ---", colors::get_color(0), "")
    );
    print_regs();
    lograw!(
        "{}<br>",
        colors::a("--- Code [Unicorn] ---", colors::get_color(0), "")
    );
    print_emu();
    lograw!(
        "{}<br>",
        colors::a("--- Stack ---", colors::get_color(0), "")
    );
    print_stack();
}

pub fn get_label(addr: usize) -> String {
    let mut label = String::new();
    // TODO: performance impact?
    for i in 0..0x2000 {
        label = uw::dbg_get_label_at(addr-i, SEGMENTREG_SEG_DEFAULT);
        if label.len() != 0 {
            if i != 0 {
                label = label + format!(" +{:03x}", i).as_str();
            }
            break;
        }
    }
    label
}
