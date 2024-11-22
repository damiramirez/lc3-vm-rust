#![allow(non_camel_case_types)]
pub enum Opcode {
    OP_BR = 0,
    OP_ADD,
    OP_LD,
    OP_ST,
    OP_JSR,
    OP_AND,
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
    pub fn from(instruction: u16) {
        let opcode = (instruction >> 12) & 0xF;
        println!("Instruction: {:016b} - code: {:04b}", instruction, opcode);

        match opcode {
            0x0001 => println!("ADD"),
            0b0101 => println!("AND"),
            0b0000 => println!("BR"),
            0b1100 => println!("JMP"),
            0b0100 => println!("JSR"),
            0b0010 => println!("LD"),
            0b1010 => println!("LDI"),
            0b0110 => println!("LDR"),
            0b1110 => println!("LEA"),
            0b1001 => println!("NOT"),
            0b1000 => println!("RTI"),
            0b0011 => println!("ST"),
            0b1011 => println!("STI"),
            0b0111 => println!("STR"),
            0b1111 => println!("TRAP"),
            _ => println!("NO OPCODE"),
        }
    }
}
