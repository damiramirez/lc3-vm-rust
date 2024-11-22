use crate::memory::Memory;
use crate::opcode::Opcode;
enum CPUErrors {
    Overflow,
}

#[derive(Default)]
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
            pc: 0x3000,
            ..Default::default()
        }
    }

    pub fn fetch_instruction(&mut self, memory: &Memory) -> Option<u16> {
        let instruction = memory.read(self.pc.into())?;
        self.pc = self.pc.checked_add(1)?;

        Opcode::from(instruction);

        Some(instruction)
    }
}
