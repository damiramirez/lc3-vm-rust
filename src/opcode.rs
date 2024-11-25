#![allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, PartialEq)]
pub enum Opcode {
    OP_BR {
        n: bool,
        z: bool,
        p: bool,
        offset: i16,
    },
    OP_ADD_SR {
        dr: u16,
        sr1: u16,
        mode: bool,
        sr2: u16,
    },
    OP_ADD_IMM {
        dr: u16,
        sr1: u16,
        mode: bool,
        imm5: i16,
    },
    OP_LD {
        dr: u16,
        offset: i16,
    },
    OP_ST {
        sr: u16,
        offset: i16,
    },
    OP_JSR {
        mode: bool,
        offset: i16,
    },
    OP_JSRR {
        mode: bool,
        base_r: u16,
    },
    OP_AND_SR {
        dr: u16,
        sr1: u16,
        mode: bool,
        sr2: u16,
    },
    OP_AND_IMM {
        dr: u16,
        sr1: u16,
        mode: bool,
        imm5: i16,
    },
    OP_LDR {
        dr: u16,
        base_r: u16,
        offset: i16,
    },
    OP_STR {
        sr: u16,
        base_r: u16,
        offset: i16,
    },
    OP_RTI,
    OP_NOT {
        dr: u16,
        sr: u16,
    },
    OP_LDI {
        dr: u16,
        offset: i16,
    },
    OP_STI {
        sr: u16,
        offset: i16,
    },
    OP_JMP {
        base_r: u16,
    },
    OP_RET,
    OP_RES,
    OP_LEA {
        dr: u16,
        offset: i16,
    },
    OP_TRAP {
        trapvec: Trap,
    },
}

#[derive(Debug, PartialEq)]
pub enum OpcodeError {
    InvalidOpcode,
}

#[derive(Debug, PartialEq)]
pub enum Trap {
    GetC,
    Out,
    Puts,
    In,
    Putsp,
    Halt,
}

impl Opcode {
    pub fn from(instruction: u16) -> Result<Self, OpcodeError> {
        let opcode = (instruction >> 12) & 0b0000_0000_0000_1111;
        print!("Instruction: {:016b} - ", instruction);

        match opcode {
            0x0001 => {
                let dr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let sr1 = (instruction >> 6) & 0b0000_0000_0000_0111;
                let mode = (instruction >> 5) & 0b0000_0000_0000_0001 == 1;
                match mode {
                    false => {
                        let sr2 = instruction & 0b0000_0000_0000_0111;
                        Ok(Opcode::OP_ADD_SR { dr, sr1, mode, sr2 })
                    }
                    true => {
                        let imm5 = sign_ext_imm5(instruction & 0b_0000_0000_0001_1111);
                        Ok(Opcode::OP_ADD_IMM {
                            dr,
                            sr1,
                            mode,
                            imm5,
                        })
                    }
                }
            }
            0b0101 => {
                let dr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let sr1 = (instruction >> 6) & 0b0000_0000_0000_0111;
                let mode = (instruction >> 5) & 0b0000_0000_0000_0001 == 1;
                match mode {
                    false => {
                        let sr2 = instruction & 0b0000_0000_0000_0111;
                        Ok(Opcode::OP_AND_SR { dr, sr1, mode, sr2 })
                    }
                    true => {
                        let imm5 = sign_ext_imm5(instruction);
                        Ok(Opcode::OP_AND_IMM {
                            dr,
                            sr1,
                            mode,
                            imm5,
                        })
                    }
                }
            }
            0b0000 => {
                let n = ((instruction >> 11) & 0b0000_0000_0000_0001) != 0;
                let z = ((instruction >> 10) & 0b0000_0000_0000_0001) != 0;
                let p = ((instruction >> 9) & 0b0000_0000_0000_0001) != 0;
                let offset = sign_ext_imm9(instruction);
                Ok(Opcode::OP_BR { n, z, p, offset })
            }
            0b1100 => {
                let base_r = (instruction >> 6) & 0b0000_0000_0000_0111;
                match base_r {
                    0b111 => Ok(Opcode::OP_RET),
                    _ => Ok(Opcode::OP_JMP { base_r }),
                }
            }
            0b0100 => {
                let mode = (instruction >> 11) & 0b0000_0000_0000_0001 == 1;
                match mode {
                    false => {
                        let base_r = (instruction >> 6) & 0b0000_0000_0011_1111;
                        Ok(Opcode::OP_JSRR { mode, base_r })
                    }
                    true => {
                        let offset = sign_ext_imm11(instruction);
                        Ok(Opcode::OP_JSR { mode, offset })
                    }
                }
            }
            0b0010 => {
                let dr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let offset = sign_ext_imm9(instruction);
                Ok(Opcode::OP_LD { dr, offset })
            }
            0b1010 => {
                let dr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let offset = sign_ext_imm9(instruction);
                Ok(Opcode::OP_LDI { dr, offset })
            }
            0b0110 => {
                let dr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let base_r = (instruction >> 6) & 0b0000_0000_0000_0111;
                let offset = sign_ext_imm6(instruction);
                Ok(Opcode::OP_LDR { dr, base_r, offset })
            }
            0b1110 => {
                let dr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let offset = sign_ext_imm9(instruction);
                Ok(Opcode::OP_LEA { dr, offset })
            }
            0b1001 => {
                let dr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let sr = (instruction >> 6) & 0b0000_0000_0000_0111;
                Ok(Opcode::OP_NOT { dr, sr })
            }
            0b1000 => Ok(Opcode::OP_RTI),
            0b0011 => {
                let sr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let offset = sign_ext_imm9(instruction);
                Ok(Opcode::OP_ST { sr, offset })
            }
            0b1011 => {
                let sr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let offset = sign_ext_imm9(instruction);
                Ok(Opcode::OP_STI { sr, offset })
            }
            0b0111 => {
                let sr = (instruction >> 9) & 0b0000_0000_0000_0111;
                let base_r = (instruction >> 6) & 0b0000_0000_0000_0111;
                let offset = sign_ext_imm6(instruction);
                Ok(Opcode::OP_STR { sr, base_r, offset })
            }
            0b1111 => {
                let value = instruction & 0b000_0000_1111_1111;
                match value {
                    0x20 => Ok(Opcode::OP_TRAP {
                        trapvec: Trap::GetC,
                    }),
                    0x21 => Ok(Opcode::OP_TRAP { trapvec: Trap::Out }),
                    0x22 => Ok(Opcode::OP_TRAP {
                        trapvec: Trap::Puts,
                    }),
                    0x23 => Ok(Opcode::OP_TRAP { trapvec: Trap::In }),
                    0x24 => Ok(Opcode::OP_TRAP {
                        trapvec: Trap::Putsp,
                    }),
                    0x25 => Ok(Opcode::OP_TRAP {
                        trapvec: Trap::Halt,
                    }),
                    _ => Err(OpcodeError::InvalidOpcode),
                }
            }
            0b1101 => Ok(Opcode::OP_RES),
            _ => Err(OpcodeError::InvalidOpcode),
        }
    }
}

pub fn sign_ext_imm6(instruction: u16) -> i16 {
    let offset: i16 = (instruction & 0b11_1111).try_into().unwrap_or_default();

    if offset & 0b10_0000 != 0 {
        offset | !0b11_1111
    } else {
        offset & 0b11_1111
    }
}

pub fn sign_ext_imm9(instruction: u16) -> i16 {
    let offset: i16 = (instruction & 0b1_1111_1111).try_into().unwrap_or_default();

    if offset & 0b1_0000_0000 != 0 {
        offset | !0b1_1111_1111
    } else {
        offset & 0b1_1111_1111
    }
}

pub fn sign_ext_imm5(instruction: u16) -> i16 {
    let imm5: i16 = (instruction & 0b0000_0000_0001_1111)
        .try_into()
        .unwrap_or_default();

    if imm5 & 0b1_0000 != 0 {
        imm5 | !0b0001_1111
    } else {
        imm5 & 0b0001_1111
    }
}

pub fn sign_ext_imm11(instruction: u16) -> i16 {
    let offset: i16 = (instruction & 0b111_1111_1111)
        .try_into()
        .unwrap_or_default();

    if offset & 0b100_0000_0000 != 0 {
        offset | !0b111_1111_1111
    } else {
        offset & 0b111_1111_1111
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_add() -> Result<(), OpcodeError> {
        let instruction = 0b0001_0010_1000_0011;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_ADD_SR {
                dr: 1,
                sr1: 2,
                mode: false,
                sr2: 3
            }
        );

        let instruction = 0b0001_0010_1010_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_ADD_IMM {
                dr: 1,
                sr1: 2,
                mode: true,
                imm5: 1
            }
        );

        Ok(())
    }

    #[test]
    fn test_op_and() -> Result<(), OpcodeError> {
        let instruction = 0b0101_0010_1000_0011;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_AND_SR {
                dr: 1,
                sr1: 2,
                mode: false,
                sr2: 3
            }
        );

        let instruction = 0b0101_0010_1010_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_AND_IMM {
                dr: 1,
                sr1: 2,
                mode: true,
                imm5: 1
            }
        );

        Ok(())
    }

    #[test]
    fn test_op_br() -> Result<(), OpcodeError> {
        let instruction = 0b0000_1110_0000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_BR {
                n: true,
                z: true,
                p: true,
                offset: 1
            }
        );

        Ok(())
    }

    #[test]
    fn test_op_jmp() -> Result<(), OpcodeError> {
        let instruction = 0b1100_0000_1101_1011;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_JMP { base_r: 3 });
        Ok(())
    }

    #[test]
    fn test_op_jsr() -> Result<(), OpcodeError> {
        let instruction = 0b0100_1000_0000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_JSR {
                mode: true,
                offset: 1
            }
        );
        Ok(())
    }

    #[test]
    fn test_op_ld() -> Result<(), OpcodeError> {
        let instruction = 0b0010_0010_0000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_LD { dr: 1, offset: 1 });
        Ok(())
    }

    #[test]
    fn test_op_ldi() -> Result<(), OpcodeError> {
        let instruction = 0b1010_0010_0000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_LDI { dr: 1, offset: 1 });
        Ok(())
    }

    #[test]
    fn test_op_ldr() -> Result<(), OpcodeError> {
        let instruction = 0b0110_0010_1000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_LDR {
                dr: 1,
                base_r: 2,
                offset: 1
            }
        );
        Ok(())
    }

    #[test]
    fn test_op_lea() -> Result<(), OpcodeError> {
        let instruction = 0b1110_0010_0000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_LEA { dr: 1, offset: 1 });
        Ok(())
    }

    #[test]
    fn test_op_not() -> Result<(), OpcodeError> {
        let instruction = 0b1001_0010_1011_1111;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_NOT { dr: 1, sr: 2 });
        Ok(())
    }

    #[test]
    fn test_op_rti() -> Result<(), OpcodeError> {
        let instruction = 0b1000_0000_0000_0000;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_RTI);
        Ok(())
    }

    #[test]
    fn test_op_st() -> Result<(), OpcodeError> {
        let instruction = 0b0011_0010_0000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_ST { sr: 1, offset: 1 });
        Ok(())
    }

    #[test]
    fn test_op_sti() -> Result<(), OpcodeError> {
        let instruction = 0b1011_0010_0000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_STI { sr: 1, offset: 1 });
        Ok(())
    }

    #[test]
    fn test_op_str() -> Result<(), OpcodeError> {
        let instruction = 0b0111_0010_1000_0001;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_STR {
                sr: 1,
                base_r: 2,
                offset: 1
            }
        );
        Ok(())
    }

    #[test]
    fn test_op_trap() -> Result<(), OpcodeError> {
        let instruction = 0b1111_0000_0010_0000;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(
            opcode,
            Opcode::OP_TRAP {
                trapvec: Trap::GetC
            }
        );
        Ok(())
    }

    #[test]
    fn test_op_res() -> Result<(), OpcodeError> {
        let instruction = 0b1101_0000_0000_0000;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_RES);
        Ok(())
    }
}
