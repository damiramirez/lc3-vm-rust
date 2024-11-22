#![allow(non_camel_case_types)]
#[repr(u16)]
pub enum Opcode {
    OP_BR = 0,
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

                println!(
                    "DR: {:03b} - SR1: {:03b} - M: {:01b} - SR2: {:05b}",
                    dr, sr1, mode, sr2
                );

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

                println!(
                    "DR: {:03b} - SR1: {:03b} - M: {:01b} - SR2: {:05b}",
                    dr, sr1, mode, sr2
                );

                Opcode::OP_AND { dr, sr1, mode, sr2 }
            }
            0b0000 => {}
            // 0b1100 => println!("JMP"),
            // 0b0100 => println!("JSR"),
            // 0b0010 => println!("LD"),
            // 0b1010 => println!("LDI"),
            // 0b0110 => println!("LDR"),
            // 0b1110 => println!("LEA"),
            // 0b1001 => println!("NOT"),
            // 0b1000 => println!("RTI"),
            // 0b0011 => println!("ST"),
            // 0b1011 => println!("STI"),
            // 0b0111 => println!("STR"),
            // 0b1111 => println!("TRAP"),
            _ => todo!(),
        }
    }
}
