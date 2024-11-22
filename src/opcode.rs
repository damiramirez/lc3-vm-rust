#![allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug)]
pub enum Opcode {
    OP_BR {
        n: bool,
        z: bool,
        p: bool,
        offset: u16,
    },
    OP_ADD {
        dr: u16,
        sr1: u16,
        mode: u16,
        sr2: u16,
    },
    OP_LD,
    OP_ST,
    OP_JSR,
    OP_AND {
        dr: u16,
        sr1: u16,
        mode: u16,
        sr2: u16,
    },
    OP_LDR,
    OP_STR,
    OP_RTI,
    OP_NOT,
    OP_LDI,
    OP_STI,
    OP_JMP,
    OP_RES,
    OP_LEA,
    OP_TRAP,
}

impl Opcode {
    pub fn from(instruction: u16) -> Self {
        let opcode = (instruction >> 12) & 0xF;
        print!("Instruction: {:016b} - ", instruction);

        match opcode {
            0x0001 => {
                let dr = (instruction >> 9) & 0x7;
                let sr1 = (instruction >> 6) & 0x7;
                let mode = (instruction >> 5) & 0x1;
                let sr2 = match mode {
                    0 => instruction & 0x3,
                    1 => instruction & 0x1F,
                    _ => todo!(),
                };

                Opcode::OP_ADD { dr, sr1, mode, sr2 }
            }
            0b0101 => {
                let dr = (instruction >> 9) & 0x7;
                let sr1 = (instruction >> 6) & 0x7;
                let mode = (instruction >> 5) & 0x1;
                let sr2 = match mode {
                    0 => instruction & 0x3,
                    1 => instruction & 0x1F,
                    _ => todo!(),
                };

                Opcode::OP_AND { dr, sr1, mode, sr2 }
            }
            0b0000 => {
                let n = ((instruction >> 11) & 0x1) != 0;
                let z = ((instruction >> 10) & 0x1) != 0;
                let p = ((instruction >> 9) & 0x1) != 0;
                let offset = instruction & 0xFF;

                Opcode::OP_BR { n, z, p, offset }
            }
            0b1100 => todo!("JMP"),
            0b0100 => todo!("JSR"),
            0b0010 => todo!("LD"),
            0b1010 => todo!("LDI"),
            0b0110 => todo!("LDR"),
            0b1110 => todo!("LEA"),
            0b1001 => todo!("NOT"),
            0b1000 => todo!("RTI"),
            0b0011 => todo!("ST"),
            0b1011 => todo!("STI"),
            0b0111 => todo!("STR"),
            0b1111 => todo!("TRAP"),
            _ => todo!(),
        }
    }
}

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    let bit_count = match bit_count.checked_sub(1) {
        Some(value) => value,
        None => todo!(),
    };

    if (x >> bit_count) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}
