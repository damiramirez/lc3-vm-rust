use std::{env, fs};

use cpu::CPU;
use memory::Memory;

mod cpu;
mod memory;
mod opcode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(file) => file,
        None => return,
    };

    let bytes = match load_obj(filename) {
        Ok(bytes) => bytes,
        Err(_) => return,
    };

    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let _ = memory.load_memory(&bytes);

    for (i, value) in memory.cells.iter().enumerate() {
        if *value != 0 {
            print!("Address {} => ", i);
            cpu.fetch_instruction(&memory);
        }
    }
}

fn load_obj(filename: &str) -> Result<Vec<u16>, String> {
    let data = fs::read(filename).map_err(|e| format!("Problem reading the file: {}", e))?;
    let mut loaded_memory = Vec::new();
    for two_bytes in data.chunks_exact(2) {
        let first_byte = two_bytes.first().ok_or("Errors with the file")?;
        let second_byte = two_bytes.get(1).ok_or("Error with the file")?;
        let joined_bytes = u16::from_be_bytes([*first_byte, *second_byte]);
        loaded_memory.push(joined_bytes);
    }

    Ok(loaded_memory)
}
