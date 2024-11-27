use crate::flags::ConditionFlags;
use crate::memory::Memory;
use crate::opcode::{Opcode, Trap};
use std::{
    fmt::Debug,
    fs::File,
    io::{self, Read, Write},
    os::fd::AsFd,
};

pub enum CPUErrors {
    Register,
    Flag,
    Execute,
}

#[warn(clippy::upper_case_acronyms)]
pub struct CPU {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
    pub memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0x3000,
            cond: 0,
            memory: Memory::new(),
        }
    }

    pub fn fetch_instruction(&mut self) -> Option<u16> {
        let instruction = self.memory.read(self.pc.into())?;
        self.pc = self.pc.checked_add(1)?;

        let op = Opcode::from(instruction);

        Some(instruction)
    }

    pub fn execute(&mut self, opcode: Opcode) -> Result<(), CPUErrors> {
        match opcode {
            Opcode::OP_ADD_REG { dr, sr1, sr2 } => {
                let src_register = self.get_register_value(sr1)?;
                let rhs_register = self.get_register_value(sr2)?;
                let sum = src_register.wrapping_add(rhs_register);
                self.update_register(dr, sum)
                    .map_err(|_| CPUErrors::Execute)?;

                self.update_flag(dr)?;
            }
            Opcode::OP_ADD_IMM { dr, sr1, imm5 } => {
                let src_register = self.get_register_value(sr1)?;
                let sum = src_register.wrapping_add(imm5);
                self.update_register(dr, sum)
                    .map_err(|_| CPUErrors::Execute)?;

                self.update_flag(dr)?;
            }
            Opcode::OP_AND_REG { dr, sr1, sr2 } => {
                let src_register = self.get_register_value(sr1)?;
                let rhs_register = self.get_register_value(sr2)?;

                let result = src_register & rhs_register;
                self.update_register(dr, result)
                    .map_err(|_| CPUErrors::Execute)?;

                self.update_flag(dr)?;
            }
            Opcode::OP_AND_IMM { dr, sr1, imm5 } => {
                let src_register = self.get_register_value(sr1)?;

                let imm5: u16 = imm5;
                let result = src_register & imm5;
                self.update_register(dr, result)
                    .map_err(|_| CPUErrors::Execute)?;

                self.update_flag(dr)?;
            }
            Opcode::OP_BR { n, z, p, offset } => {
                // If any of the condition codes tested is set, the program branches to the location
                // specified by adding the sign-extended PCoffset9 field to the incremented PC.
                if (n && self.cond == ConditionFlags::NEG.into())
                    || (z && self.cond == ConditionFlags::ZRO.into())
                    || (p && self.cond == ConditionFlags::POS.into())
                {
                    self.pc = self.pc.wrapping_add(offset);
                }
            }
            Opcode::OP_JMP { base_r } => {
                self.pc = self.get_register_value(base_r)?;
            }
            Opcode::OP_JSR { offset } => {
                self.r7 = self.pc;
                self.pc = self.pc.wrapping_add(offset);
            }
            Opcode::OP_JSRR { base_r } => {
                self.r7 = self.pc;
                self.pc = self.get_register_value(base_r)?;
            }
            Opcode::OP_LD { dr, offset } => {
                let address = self.pc.wrapping_add(offset);
                if let Some(read_value) = self.memory.read(address.into()) {
                    self.update_register(dr, read_value)?;
                    self.update_flag(dr)?;
                }
            }
            Opcode::OP_LDI { dr, offset } => {
                let address = self.pc.wrapping_add(offset);
                let first_read = self.memory.read(address.into()).ok_or(CPUErrors::Execute)?;
                let read_value = self
                    .memory
                    .read(first_read.into())
                    .ok_or(CPUErrors::Execute)?;

                self.update_register(dr, read_value)?;
                self.update_flag(dr)?;
            }
            Opcode::OP_LDR { dr, base_r, offset } => {
                let base_value = self.get_register_value(base_r)?;
                let address = base_value.wrapping_add(offset);
                let read_value = self.memory.read(address.into()).ok_or(CPUErrors::Execute)?;
                self.update_register(dr, read_value)?;
                self.update_flag(dr)?;
            }
            Opcode::OP_LEA { dr, offset } => {
                let result = self.pc.wrapping_add(offset);
                self.update_register(dr, result)?;
                self.update_flag(dr)?;
            }
            Opcode::OP_NOT { dr, sr } => {
                let value = self.get_register_value(sr)?;
                self.update_register(dr, !value)?;
                self.update_flag(dr)?;
            }
            Opcode::OP_RET => {
                self.pc = self.r7;
            }
            Opcode::OP_RTI => {
                println!("unused")
            }
            Opcode::OP_RES => {
                println!("unused");
            }
            Opcode::OP_ST { sr, offset } => {
                let address = self.pc.wrapping_add(offset);
                let sr_register = self.get_register_value(sr)?;

                self.memory
                    .write(address, sr_register)
                    .map_err(|_| CPUErrors::Execute)?;
            }
            Opcode::OP_STI { sr, offset } => {
                let address = self.pc.wrapping_add(offset);
                let read_address = self.memory.read(address.into()).ok_or(CPUErrors::Execute)?;

                let sr_register = self.get_register_value(sr)?;

                self.memory
                    .write(read_address, sr_register)
                    .map_err(|_| CPUErrors::Execute)?;
            }
            Opcode::OP_TRAP { trapvec } => {
                match trapvec {
                    Trap::GetC => todo!(),
                    Trap::Out => todo!(),
                    Trap::Puts => {
                        let mut address = self.r0;
                        let mut value =
                            self.memory.read(address.into()).ok_or(CPUErrors::Execute)?;

                        while value != 0x0000 {
                            let c: u8 = value.try_into().map_err(|_| CPUErrors::Execute)?;
                            let c: char = c.into();
                            print!("{}", c);
                            address = address.wrapping_add(1);
                            value = self.memory.read(address.into()).ok_or(CPUErrors::Execute)?;
                        }

                        io::stdout().flush().map_err(|_| CPUErrors::Execute)?;
                    }
                    Trap::In => todo!(),
                    Trap::Putsp => todo!(),
                    Trap::Halt => todo!(),
                };
            }
            _ => return Err(CPUErrors::Execute),
        };

        Ok(())
    }

    pub fn get_register(&mut self, index: u16) -> Result<&mut u16, CPUErrors> {
        let register_value = match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            4 => &mut self.r4,
            5 => &mut self.r5,
            6 => &mut self.r6,
            7 => &mut self.r7,
            8 => &mut self.pc,
            _ => return Err(CPUErrors::Register),
        };

        Ok(register_value)
    }

    pub fn get_register_value(&self, index: u16) -> Result<u16, CPUErrors> {
        let register_value = match index {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            _ => return Err(CPUErrors::Register),
        };

        Ok(register_value)
    }

    pub fn update_register(&mut self, index: u16, value: u16) -> Result<(), CPUErrors> {
        let register = self.get_register(index)?;
        *register = value;
        Ok(())
    }

    pub fn update_flag(&mut self, register: u16) -> Result<bool, CPUErrors> {
        let register_value = self.get_register(register).map_err(|_| CPUErrors::Flag)?;

        if *register_value == 0 {
            self.cond = ConditionFlags::ZRO.into();
        } else if (*register_value >> 15) == 1 {
            self.cond = ConditionFlags::NEG.into();
        } else {
            self.cond = ConditionFlags::POS.into();
        }

        Ok(true)
    }
}
