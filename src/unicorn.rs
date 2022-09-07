use iced_x86;
use unicorn_engine::unicorn_const as ucc;
use unicorn_engine::{RegisterX86, Unicorn};

use crate::colors;
use crate::*; // needed for logprintln macro
use common::unsafe_wrapper as uw;

pub fn page_align(addr: usize) -> usize {
    addr & !(0x1000 - 1)
}

pub fn get_aligned_addr_and_size(addr: usize, size: usize) -> (usize, usize) {
    let addr = page_align(addr);
    let size = page_align(size) + 0x1000;
    (addr, size)
}

/// Converts iced_x86::Register to unicorn_engine::RegisterX86
pub fn iced_reg_to_unicorn(
    r: iced_x86::Register,
) -> unicorn_engine::RegisterX86 {
    match r {
        iced_x86::Register::AH => RegisterX86::AH,
        iced_x86::Register::AL => RegisterX86::AL,
        iced_x86::Register::AX => RegisterX86::AX,
        iced_x86::Register::BH => RegisterX86::BH,
        iced_x86::Register::BL => RegisterX86::BL,
        iced_x86::Register::BP => RegisterX86::BP,
        iced_x86::Register::BPL => RegisterX86::BPL,
        iced_x86::Register::BX => RegisterX86::BX,
        iced_x86::Register::CH => RegisterX86::CH,
        iced_x86::Register::CL => RegisterX86::CL,
        iced_x86::Register::CS => RegisterX86::CS,
        iced_x86::Register::CX => RegisterX86::CX,
        iced_x86::Register::DH => RegisterX86::DH,
        iced_x86::Register::DI => RegisterX86::DI,
        iced_x86::Register::DIL => RegisterX86::DIL,
        iced_x86::Register::DL => RegisterX86::DL,
        iced_x86::Register::DS => RegisterX86::DS,
        iced_x86::Register::DX => RegisterX86::DX,
        iced_x86::Register::EAX => RegisterX86::EAX,
        iced_x86::Register::EBP => RegisterX86::EBP,
        iced_x86::Register::EBX => RegisterX86::EBX,
        iced_x86::Register::ECX => RegisterX86::ECX,
        iced_x86::Register::EDI => RegisterX86::EDI,
        iced_x86::Register::EDX => RegisterX86::EDX,
        iced_x86::Register::EIP => RegisterX86::EIP,
        iced_x86::Register::ES => RegisterX86::ES,
        iced_x86::Register::ESI => RegisterX86::ESI,
        iced_x86::Register::ESP => RegisterX86::ESP,
        iced_x86::Register::FS => RegisterX86::FS,
        iced_x86::Register::GS => RegisterX86::GS,
        iced_x86::Register::RAX => RegisterX86::RAX,
        iced_x86::Register::RBP => RegisterX86::RBP,
        iced_x86::Register::RBX => RegisterX86::RBX,
        iced_x86::Register::RCX => RegisterX86::RCX,
        iced_x86::Register::RDI => RegisterX86::RDI,
        iced_x86::Register::RDX => RegisterX86::RDX,
        iced_x86::Register::RIP => RegisterX86::RIP,
        iced_x86::Register::RSI => RegisterX86::RSI,
        iced_x86::Register::RSP => RegisterX86::RSP,
        iced_x86::Register::SI => RegisterX86::SI,
        iced_x86::Register::SIL => RegisterX86::SIL,
        iced_x86::Register::SP => RegisterX86::SP,
        iced_x86::Register::SPL => RegisterX86::SPL,
        iced_x86::Register::SS => RegisterX86::SS,
        iced_x86::Register::R8 => unicorn_engine::RegisterX86::R8,
        iced_x86::Register::R9 => unicorn_engine::RegisterX86::R9,
        iced_x86::Register::R10 => unicorn_engine::RegisterX86::R10,
        iced_x86::Register::R11 => unicorn_engine::RegisterX86::R11,
        iced_x86::Register::R12 => unicorn_engine::RegisterX86::R12,
        iced_x86::Register::R13 => unicorn_engine::RegisterX86::R13,
        iced_x86::Register::R14 => unicorn_engine::RegisterX86::R14,
        iced_x86::Register::R15 => unicorn_engine::RegisterX86::R15,
        iced_x86::Register::R8D => unicorn_engine::RegisterX86::R8D,
        iced_x86::Register::R9D => unicorn_engine::RegisterX86::R9D,
        iced_x86::Register::R10D => unicorn_engine::RegisterX86::R10D,
        iced_x86::Register::R11D => unicorn_engine::RegisterX86::R11D,
        iced_x86::Register::R12D => unicorn_engine::RegisterX86::R12D,
        iced_x86::Register::R13D => unicorn_engine::RegisterX86::R13D,
        iced_x86::Register::R14D => unicorn_engine::RegisterX86::R14D,
        iced_x86::Register::R15D => unicorn_engine::RegisterX86::R15D,
        iced_x86::Register::R8W => unicorn_engine::RegisterX86::R8W,
        iced_x86::Register::R9W => unicorn_engine::RegisterX86::R9W,
        iced_x86::Register::R10W => unicorn_engine::RegisterX86::R10W,
        iced_x86::Register::R11W => unicorn_engine::RegisterX86::R11W,
        iced_x86::Register::R12W => unicorn_engine::RegisterX86::R12W,
        iced_x86::Register::R13W => unicorn_engine::RegisterX86::R13W,
        iced_x86::Register::R14W => unicorn_engine::RegisterX86::R14W,
        iced_x86::Register::R15W => unicorn_engine::RegisterX86::R15W,
        iced_x86::Register::R8L => unicorn_engine::RegisterX86::R8B,
        iced_x86::Register::R9L => unicorn_engine::RegisterX86::R9B,
        iced_x86::Register::R10L => unicorn_engine::RegisterX86::R10B,
        iced_x86::Register::R11L => unicorn_engine::RegisterX86::R11B,
        iced_x86::Register::R12L => unicorn_engine::RegisterX86::R12B,
        iced_x86::Register::R13L => unicorn_engine::RegisterX86::R13B,
        iced_x86::Register::R14L => unicorn_engine::RegisterX86::R14B,
        iced_x86::Register::R15L => unicorn_engine::RegisterX86::R15B,
        _ => {
            logprintln!("Unsupported register: {:?}", r);
            return unicorn_engine::RegisterX86::RAX;
        }
    }
}

fn fill_regs(emu: &mut Unicorn<()>) -> bool {
    let regs = uw::dbg_get_regs();
    let r = regs.regcontext;
    let vv = vec![
        (r.cax, RegisterX86::RAX),
        (r.ccx, RegisterX86::RCX),
        (r.cdx, RegisterX86::RDX),
        (r.cbx, RegisterX86::RBX),
        (r.csp, RegisterX86::RSP),
        (r.cbp, RegisterX86::RBP),
        (r.csi, RegisterX86::RSI),
        (r.cdi, RegisterX86::RDI),
        (r.r8, RegisterX86::R8),
        (r.r9, RegisterX86::R9),
        (r.r10, RegisterX86::R10),
        (r.r11, RegisterX86::R11),
        (r.r12, RegisterX86::R12),
        (r.r13, RegisterX86::R13),
        (r.r14, RegisterX86::R14),
        (r.r15, RegisterX86::R15),
        (r.cip, RegisterX86::R15),
        (r.eflags, RegisterX86::EFLAGS),
    ];
    for (v, u) in vv {
        let r = emu.reg_write(u, v);
        if r.is_err() {
            logprintln!("Unable to map register {:?}", u);
            return false;
        }
    }
    true
}

pub fn unicorn(addr: usize, sz: usize) {
    //
    // Create Unicorn instance
    //

    // Perf note: each time we load a Unicorn instance it eats ~1GB of RAM,
    // and makeing the instance global variable doens't really solve the
    // problem
    let mut emu = Unicorn::new(ucc::Arch::X86, ucc::Mode::MODE_64).unwrap();
    let hook = emu.add_mem_hook(
        ucc::HookType::MEM_UNMAPPED,
        0,
        u64::MAX,
        |uc, _access, addr, size, _value| {
            //logprintln!(
            //    "Unmapped memory access {:?} at {:x}, size {:x}, val {:x}",
            //    access,
            //    addr,
            //    size,
            //    value
            //);
            let (addr_a, sz_a) = get_aligned_addr_and_size(addr as usize, size);
            let (r, mem_rip) = uw::dbg_mem_read(addr_a, sz_a);
            if !r {
                logprintln!("Can't read memory {:x}:{:x}", addr_a, sz_a);
                return false;
            }
            // TODO: DbgGetProcessHandle & VittualQueryEx
            let r = uc.mem_map(addr_a as u64, sz_a, ucc::Permission::ALL);
            if r.is_err() {
                logprintln!("Unable to map memory {:x}:{:x}", addr_a, sz_a);
                return false;
            }
            let r = uc.mem_write(addr_a as u64, &mem_rip);
            if r.is_err() {
                logprintln!("Unable to write memory {:x}:{:x}", addr_a, sz_a);
                return false;
            }
            true
        },
    );
    if hook.is_err() {
        logprintln!("Unable to add memory hook");
    }
    let hook = hook.unwrap();

    //
    // Map registers general registers
    //
    if !fill_regs(&mut emu) {
        logprintln!("Unable to fill registers");
        return;
    }
    // Map teb too
    let teb = uw::dbg_get_teb_address(uw::dbg_get_thread_id());
    let r = emu.reg_write(RegisterX86::GS_BASE, teb as u64);
    if r.is_err() {
        logprintln!("Unable to map register {:?}", RegisterX86::GS_BASE);
        return;
    }

    //
    // Disassemble one instruction, map missing memory if needed,
    // execute it and repeat
    //
    static ONE_INSTRUCTION_MAX_SIZE: usize = 15;
    let mut ip = addr;
    let mut affected_regs = vec![];
    for _i in 0..sz {
        //
        // disassemble one instruction
        //
        let (r, mem_rip) = uw::dbg_mem_read(ip, ONE_INSTRUCTION_MAX_SIZE);
        if !r {
            logprintln!(
                "Unable to read memory {:x}:{:x}",
                ip,
                ONE_INSTRUCTION_MAX_SIZE
            );
            return;
        }
        let instr = common::disasm::decode_instr(&mem_rip, ip as u64);
        if instr.len() == 0 {
            logprintln!(
                "Unable to decode memory {:x}:{:x}",
                ip,
                ONE_INSTRUCTION_MAX_SIZE
            );
            return;
        }
        //
        // Symbolize registers and data
        //
        print_instruction(&instr, &emu, &mut affected_regs);
        //
        // Emulate one instruction
        //
        let r = emu.emu_start(
            ip as u64,
            (ip + instr.len()) as u64,
            5 * ucc::SECOND_SCALE,
            1,
        );
        if r.is_err() {
            logprintln!("Emulation error: {:?}", r);
            return;
        }
        //
        // Update the next address
        //
        let r = emu.reg_read(RegisterX86::RIP);
        if r.is_err() {
            logprintln!("Unable to read RIP");
            return;
        }
        ip = r.unwrap() as usize;
    }

    //
    // Cleanup
    //
    // we must remove hook, see https://github.com/unicorn-engine/unicorn/issues/1619
    let r = emu.remove_hook(hook);
    if r.is_err() {
        logprintln!("Unable to remove memory hook");
        return;
    }
}

pub fn print_instruction(
    instr: &iced_x86::Instruction,
    emu: &Unicorn<()>,
    affected_regs: &mut Vec<iced_x86::Register>,
) {
    // Find references if any
    let mut ptr = 0;
    let mut regs = affected_regs.to_owned();
    for op in 0..instr.op_count() {
        let kind = instr.op_kind(op);
        match kind {
            iced_x86::OpKind::Memory => {
                // note: there is instr.virtual_address() but it here we
                // do it in a bit different way
                let base = instr.memory_base();
                if base != iced_x86::Register::None && !regs.contains(&base) {
                    regs.push(base);
                }
                let index = instr.memory_index();
                if index != iced_x86::Register::None && !regs.contains(&index) {
                    regs.push(index);
                }
                let scale = instr.memory_index_scale();
                let disp = instr.memory_displacement64();
                let is_rel = instr.is_ip_rel_memory_operand();
                // if is_rel, disp contains already target address
                if is_rel {
                    ptr = disp as usize;
                } else {
                    if instr.segment_prefix() == iced_x86::Register::GS {
                        ptr = uw::dbg_get_teb_address(uw::dbg_get_thread_id());
                    }
                    if base != iced_x86::Register::None {
                        let r = emu.reg_read(iced_reg_to_unicorn(base));
                        if r.is_err() {
                            logprintln!("Unable to read register {:?}", base);
                            return;
                        }
                        ptr += r.unwrap() as usize;
                    }
                    if index != iced_x86::Register::None {
                        let r = emu.reg_read(iced_reg_to_unicorn(index));
                        if r.is_err() {
                            logprintln!("Unable to read register {:?}", index);
                            return;
                        }
                        logprintln!("index {:?} = {:016X}", index, r.unwrap());
                        ptr += (r.unwrap() as usize * scale as usize) as usize;
                    }
                    ptr += disp as usize;
                }
            }
            iced_x86::OpKind::Register => {
                let reg = instr.op_register(op);
                if reg != iced_x86::Register::None && !regs.contains(&reg) {
                    regs.push(reg);
                }
            }
            _ => {}
        }
    }
    //if ptr == 0 && instr.memory_displacement64() != 0 {
    //    ptr = instr.memory_displacement64() as usize;
    //}
    let mut o = String::new();
    if ptr != 0 {
        telescope::telescope_line(ptr, 3, &mut o);
    }
    let regs_str = regs
        .iter()
        .map(|r| {
            format!(
                "{:?}: {:x}",
                r,
                emu.reg_read(iced_reg_to_unicorn(*r)).unwrap()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    lograw!(
        "{} {} {} {} {}<br>",
        &colors::a(
            &format!("{:016X}", instr.ip()),
            colors::get_color(1),
            &format!("x64dbg://localhost/address64#{:016x}", instr.ip())
        ),
        &colors::a(
            &format!("{:>40}", common::get_label(instr.ip() as usize)),
            colors::get_color(1),
            ""
        ),
        &colors::a(
            &format!("{:30}", common::disasm::format_instr(instr)),
            colors::get_color(2),
            ""
        ),
        o,
        &colors::a(&format!("{}", regs_str), colors::get_color(4), ""),
    );

    // Update affected registers
    affected_regs.clear();
    if instr.op0_register() != iced_x86::Register::None {
        affected_regs.push(instr.op0_register());
    }
}
