use std::io::Read;
use thiserror::Error;

const MEMORY_SIZE: usize = 1 << 16;
const MR_KBSR: u16 = 0xFE00; /* keyboard status */
const MR_KBDR: u16 = 0xFE02; /* keyboard data */

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Failed to write in memory")]
    Write,
    #[error("Origin is missing in the file")]
    EmptyOrigin,
    #[error("Failed to load program")]
    LoadProgram,
    #[error("Failed to read the keyboard")]
    Keyboard,
}

pub struct Memory {
    pub cells: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            cells: [0; MEMORY_SIZE],
        }
    }

    pub fn write(&mut self, address: u16, value: u16) -> Result<(), MemoryError> {
        if let Some(cell) = self.cells.get_mut::<usize>(address.into()) {
            *cell = value;
            Ok(())
        } else {
            Err(MemoryError::Write)
        }
    }

    pub fn read(&mut self, address: usize) -> Option<u16> {
        let keyboard_add: usize = MR_KBSR.into();
        if address == keyboard_add {
            self.handle_keyboard().ok()?;
        }
        self.cells.get(address).copied()
    }

    pub fn load_program(&mut self, data: &[u16]) -> Result<(), MemoryError> {
        let origin: usize = match data.first() {
            Some(&value) => value.into(),
            None => return Err(MemoryError::EmptyOrigin),
        };

        let data_no_origin = data.get(1..data.len()).ok_or(MemoryError::LoadProgram)?;

        for (i, data) in data_no_origin.iter().enumerate() {
            let position = origin.checked_add(i).ok_or(MemoryError::LoadProgram)?;
            self.write(position.try_into().unwrap_or_default(), *data)?;
        }

        Ok(())
    }

    fn handle_keyboard(&mut self) -> Result<(), MemoryError> {
        let mut buffer = [0; 1];
        std::io::stdin()
            .read_exact(&mut buffer)
            .map_err(|_| MemoryError::Keyboard)?;

        if buffer[0] != 0 {
            self.write(MR_KBSR, 1 << 15)?;
            self.write(MR_KBDR, u16::from(*buffer.first().unwrap_or(&0)))?;
        } else {
            self.write(MR_KBSR, 0)?;
        }

        Ok(())
    }
}
