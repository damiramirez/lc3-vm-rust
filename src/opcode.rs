#![allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, PartialEq)]
pub enum Opcode {
    OP_BR {
        n: bool,
        z: bool,
        p: bool,
        offset: u16,
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
        imm5: u16,
    },
    OP_LD {
        dr: u16,
        offset: u16,
    },
    OP_ST {
        sr: u16,
        offset: u16,
    },
    OP_JSR {
        mode: bool,
        offset: u16,
    },
    OP_AND {
        dr: u16,
        sr1: u16,
        mode: bool,
        sr2: u16,
    },
    OP_LDR {
        dr: u16,
        base_r: u16,
        offset: u16,
    },
    OP_STR {
        sr: u16,
        base_r: u16,
        offset: u16,
    },
    OP_RTI,
    OP_NOT {
        dr: u16,
        sr: u16,
    },
    OP_LDI {
        dr: u16,
        offset: u16,
    },
    OP_STI {
        sr: u16,
        offset: u16,
    },
    OP_JMP {
        base_r: u16,
    },
    OP_RET,
    OP_RES,
    OP_LEA {
        dr: u16,
        offset: u16,
    },
    OP_TRAP {
        trapvec: u16,
    },
}

#[derive(Debug)]
pub enum OpcodeError {
    InvalidOpcode,
}

impl Opcode {
    pub fn from(instruction: u16) -> Result<Self, OpcodeError> {
        let opcode = (instruction >> 12) & 0xF;
        print!("Instruction: {:016b} - ", instruction);

        match opcode {
            0x0001 => {
                let dr = (instruction >> 9) & 0x7;
                let sr1 = (instruction >> 6) & 0x7;
                let mode = (instruction >> 5) & 0x1 == 1;
                match mode {
                    false => {
                        let sr2 = instruction & 0x7;
                        Ok(Opcode::OP_ADD_SR { dr, sr1, mode, sr2 })
                    }
                    true => {
                        let imm5 = sign_extend(instruction & 0x1F, 4);
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
                let dr = (instruction >> 9) & 0x7;
                let sr1 = (instruction >> 6) & 0x7;
                let mode = (instruction >> 5) & 0x1 == 1;
                let sr2 = match mode {
                    false => instruction & 0x3,
                    true => sign_extend(instruction & 0x1F, 4),
                };

                Ok(Opcode::OP_AND { dr, sr1, mode, sr2 })
            }
            0b0000 => {
                let n = ((instruction >> 11) & 0x1) != 0;
                let z = ((instruction >> 10) & 0x1) != 0;
                let p = ((instruction >> 9) & 0x1) != 0;
                let offset = sign_extend(instruction & 0x1FF, 9);
                Ok(Opcode::OP_BR { n, z, p, offset })
            }
            0b1100 => {
                let base_r = (instruction >> 6) & 0x7;
                match base_r {
                    0b111 => Ok(Opcode::OP_RET),
                    _ => Ok(Opcode::OP_JMP { base_r }),
                }
            }
            0b0100 => {
                let offset = sign_extend(instruction & 0x3FF, 10);
                let mode = (instruction >> 11) & 0x1 == 1;
                Ok(Opcode::OP_JSR { mode, offset })
            }
            0b0010 => {
                let dr = (instruction >> 9) & 0x7;
                let offset = sign_extend(instruction & 0x1FF, 9);
                Ok(Opcode::OP_LD { dr, offset })
            }
            0b1010 => {
                let dr = (instruction >> 9) & 0x7;
                let offset = sign_extend(instruction & 0x1FF, 9);
                Ok(Opcode::OP_LDI { dr, offset })
            }
            0b0110 => {
                let dr = (instruction >> 9) & 0x7;
                let base_r = (instruction >> 6) & 0x7;
                let offset = sign_extend(instruction & 0x1F, 6);
                Ok(Opcode::OP_LDR { dr, base_r, offset })
            }
            0b1110 => {
                let dr = (instruction >> 9) & 0x7;
                let offset = sign_extend(instruction & 0x1FF, 9);
                Ok(Opcode::OP_LEA { dr, offset })
            }
            0b1001 => {
                let dr = (instruction >> 9) & 0x7;
                let sr = (instruction >> 6) & 0x7;
                Ok(Opcode::OP_NOT { dr, sr })
            }
            0b1000 => Ok(Opcode::OP_RTI),
            0b0011 => {
                let sr = (instruction >> 9) & 0x7;
                let offset = sign_extend(instruction & 0x1FF, 9);
                Ok(Opcode::OP_ST { sr, offset })
            }
            0b1011 => {
                let sr = (instruction >> 9) & 0x7;
                let offset = sign_extend(instruction & 0x1FF, 9);
                Ok(Opcode::OP_STI { sr, offset })
            }
            0b0111 => {
                let sr = (instruction >> 9) & 0x7;
                let base_r = (instruction >> 6) & 0x7;
                let offset = sign_extend(instruction & 0x3F, 6);
                Ok(Opcode::OP_STR { sr, base_r, offset })
            }
            0b1111 => {
                let trapvec = instruction & 0xFF;
                Ok(Opcode::OP_TRAP { trapvec })
            }
            0b1101 => Ok(Opcode::OP_RES),
            _ => Err(OpcodeError::InvalidOpcode),
        }
    }
}

fn sign_extend(mut value: u16, bit_count: u8) -> u16 {
    let sub_bit_count = match bit_count.checked_sub(1) {
        Some(value) => value,
        None => todo!(),
    };

    if (value >> (sub_bit_count)) & 1 != 0 {
        value |= 0xFFFF << bit_count;
    }
    value
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
            Opcode::OP_AND {
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
            Opcode::OP_AND {
                dr: 1,
                sr1: 2,
                mode: true,
                sr2: 1
            }
        );

        Ok(())
    }

    #[test]
    fn parse_add_mode_1_negative_imm5() -> Result<(), OpcodeError> {
        let add = Opcode::OP_ADD_IMM {
            dr: 1,
            sr1: 2,
            mode: true,
            imm5: 0xFFFD,
        };
        let instruction: u16 = 0b0001_0010_1011_1101;
        assert_eq!(add, Opcode::from(instruction)?);
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
        let instruction = 0b1111_0000_0000_0011;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_TRAP { trapvec: 3 });
        Ok(())
    }

    #[test]
    fn test_op_res() -> Result<(), OpcodeError> {
        let instruction = 0b1101_0000_0000_0000;
        let opcode = Opcode::from(instruction)?;
        assert_eq!(opcode, Opcode::OP_RES);
        Ok(())
    }

    #[test]
    fn test_sign_extend_positive() {
        let value = 0b00011;
        let bit_count = 5;
        let result = sign_extend(value, bit_count);
        assert_eq!(result, 0b00011);
    }

    #[test]
    fn test_sign_extend_negative() {
        let value = 0b11111;
        let bit_count = 5;
        let result = sign_extend(value, bit_count);
        assert_eq!(result, 0xFFFF);
    }
}
