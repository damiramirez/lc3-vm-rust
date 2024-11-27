const MEMORY_SIZE: usize = 1 << 16;

pub enum MemoryError {
    OutOfIndex,
    EmptyOrigin,
    Overflow,
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
            Err(MemoryError::OutOfIndex)
        }
    }

    pub fn read(&self, address: usize) -> Option<u16> {
        self.cells.get(address).copied()
    }

    pub fn load_program(&mut self, data: &[u16]) -> Result<(), MemoryError> {
        let origin: usize = match data.first() {
            Some(&value) => value.into(),
            None => return Err(MemoryError::EmptyOrigin),
        };

        let data_no_origin = data.get(1..data.len()).ok_or(MemoryError::Overflow)?;

        for (i, data) in data_no_origin.iter().enumerate() {
            let position = origin.checked_add(i).ok_or(MemoryError::Overflow)?;
            self.write(position.try_into().unwrap_or_default(), *data)?;
        }

        Ok(())
    }
}
