use iced_x86::{Decoder, DecoderOptions, 
    Formatter, 
    Instruction, 
    IntelFormatter
    };

pub fn decode_instr(bytes: &[u8], addr: u64) -> Instruction {
    let mut instr = Instruction::default();
    // Setup decoder
    let mut decoder = Decoder::with_ip(64, bytes, addr, DecoderOptions::NONE);
    if !decoder.can_decode() {
        return instr;
    }
    // Decode
    decoder.decode_out(&mut instr);
    instr
}

pub fn format_instr(instr: &Instruction) -> String {
    let mut o = String::new();
    // Format
    let mut formatter = IntelFormatter::new();
    formatter.format(&instr, &mut o);
    o
}
