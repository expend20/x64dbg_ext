//
// Coloring for x64dbg
//

use crate::common::from_hex;
use crate::common::unsafe_wrapper::brdg_setting_get;
use crate::*; // needed for logprintln macro

pub fn a(text: &str, color: &str, href: &str) -> String {
    let text = text.replace(" ", "&nbsp;");
    format!("<a style=\"color:{color};\" href=\"{href}\">{text}</a>")
}

pub static mut COLORS_CUSTOM: Vec<String> = Vec::new();
pub static mut COLORS_COMMENT: String = String::new();

pub fn print_test_colors() {
    let r = from_hex(&brdg_setting_get("Colors", "CustomColorCount"));
    if r.is_err() {
        logprintln!("Can't get CustomColorCount from settings");
        return;
    }
    let col_count = r.unwrap();

    let mut s = String::new();
    for i in 0..col_count {
        let col_name = format!("CustomColor{}", i);
        let col_val = brdg_setting_get("Colors", &col_name);
        s = s + &format!("{} ", &a(&col_name, &col_val, "123"));
    }
    lograw!("CustomColors list: {}<br>", s);
}

pub fn init_colors() {
    unsafe {
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor2"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor4"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor5"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor6"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor8"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor9"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor10"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor11"));
        COLORS_CUSTOM.push(brdg_setting_get("Colors", "CustomColor12"));
        COLORS_COMMENT = brdg_setting_get("Colors", "CustomColor14");
    }
}

pub fn get_color(idx: usize) -> &'static str {
    unsafe {
        if idx >= COLORS_CUSTOM.len() {
            COLORS_CUSTOM[COLORS_CUSTOM.len() % idx].as_str()
        } else {
            COLORS_CUSTOM[idx].as_str()
        }
    }
}

pub fn get_color_comment() -> &'static str {
    unsafe { COLORS_COMMENT.as_str() }
}
