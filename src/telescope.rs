//
// Implementation of telescope feature
//
use crate::colors;
use crate::common::unsafe_wrapper as uw;
use crate::*; // logging and output
use x64dbg_sdk_sys::SEGMENTREG_SEG_DEFAULT;

pub fn telescope_line(addr: usize, rcount: u32, out: &mut String) {
    //
    // Recursion check
    //
    if rcount == 0 {
        *out = out.to_owned() + " ...";
        return;
    }

    //
    // Print the address
    //
    if out.len() > 0 {
        *out = out.to_owned()
            + &colors::a(" -> ", colors::get_color_comment(), "");
    }
    *out = out.to_owned()
        + &colors::a(
            &format!("{:016x}", addr),
            colors::get_color(8),
            &format!("x64dbg://localhost/address64#{:016x}", addr),
        );

    //
    // Resolve the symbol
    //
    let mut infos = vec![];
    let mut is_text = false;
    let mut is_string = false;
    let mut module = uw::dbg_get_module_at(addr);
    let mut label = String::new();
    if module.len() != 0 {
        let sect = uw::dbg_funcs_section_from_addr(addr);
        if sect.len() != 0 {
            // TODO: replace with memory attibutes test, if mem image
            if sect.contains(".text") {
                is_text = true;
            }
            label = common::get_label(addr);
            module = colors::a(
                &format!("{}:{} {}", module, sect, label),
                colors::get_color(3),
                "",
            );
        } else {
            module = colors::a(&module, colors::get_color(3), "");
        }
        infos.push(module);
    }
    let string = uw::dbg_get_string_at(addr);
    if string.len() != 0 {
        is_string = true;
        if !label.contains(string.as_str()) {
            let string2 =
                colors::a(&format!("{}", string), colors::get_color(4), "");
            infos.push(string2);
        }
    } else if label.len() != 0 {
        // last resort try to get symbol without moving backwards
        let label2 = uw::dbg_get_label_at(addr, SEGMENTREG_SEG_DEFAULT);
        if label2.len() != 0 {
            let label3 = colors::a(
                &format!("{}", label2),
                colors::get_color(5),
                "",
                );
            infos.push(label3);
        }
    }
    let joined_string =
        infos.join(&colors::a(" | ", colors::get_color_comment(), ""));
    let wrapped_string = format!(
        "{}{}{}",
        &colors::a(" (", colors::get_color_comment(), ""),
        joined_string,
        &colors::a(")", colors::get_color_comment(), ""),
    );
    if joined_string.len() > 0 {
        *out = out.to_owned() + wrapped_string.as_str();
    }

    //
    // Check if the pointer is valid, do a recursion if it is
    //
    let (r, ptr) = uw::dbg_mem_read_ptr(addr);
    if !r {
        return;
    }
    // stop rule: don't dereference a string
    if is_string {
        return;
    }
    // stop rule: .text segment, then print the disassembly
    if is_text {
        return;
    }
    telescope_line(ptr, rcount - 1, out);
}

pub fn telescope(addr: usize, size: usize, rcount: u32) {
    //
    // Here we dereference the pointers one by one and call telescope_line
    // for each of them
    //
    for i in 0..size {
        let mut line = String::new();
        telescope_line(addr + i * 8, rcount, &mut line);
        lograw!("{}<br>", line);
    }
}
