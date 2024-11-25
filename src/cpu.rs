use crate::flags::ConditionFlags;
use crate::memory::Memory;
use crate::opcode::Opcode;
pub enum CPUErrors {
    Overflow,
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
        }
    }

    pub fn fetch_instruction(&mut self, memory: &Memory) -> Option<u16> {
        let instruction = memory.read(self.pc.into())?;
        self.pc = self.pc.checked_add(1)?;

        let op = Opcode::from(instruction);
        println!("{:#?}", op);

        Some(instruction)
    }

    pub fn execute(&mut self, opcode: Opcode) -> Result<(), CPUErrors> {
        match opcode {
            Opcode::OP_ADD_SR { dr, sr1, sr2 } => {
                let src_register = self
                    .get_register_value(sr1)
                    .map_err(|_| CPUErrors::Register)?;
                let rhs_register = self
                    .get_register_value(sr2)
                    .map_err(|_| CPUErrors::Register)?;

                let sum = src_register.wrapping_add(rhs_register);
                self.update_register(dr, sum)
                    .map_err(|_| CPUErrors::Execute)?;

                self.update_flag(dr)?;
            }
            Opcode::OP_ADD_IMM { dr, sr1, imm5 } => {
                let src_register = self
                    .get_register_value(sr1)
                    .map_err(|_| CPUErrors::Register)?;

                // TODO: Check this imm5 type
                let sum = src_register.wrapping_add(imm5.try_into().unwrap_or_default());
                self.update_register(dr, sum)
                    .map_err(|_| CPUErrors::Execute)?;

                self.update_flag(dr)?;
            }
            Opcode::OP_AND_SR { dr, sr1, sr2 } => {
                let src_register = self
                    .get_register_value(sr1)
                    .map_err(|_| CPUErrors::Register)?;
                let rhs_register = self
                    .get_register_value(sr2)
                    .map_err(|_| CPUErrors::Register)?;

                let result = src_register & rhs_register;
                self.update_register(dr, result)
                    .map_err(|_| CPUErrors::Execute)?;

                self.update_flag(dr)?;
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
