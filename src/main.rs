use std::{
    env,
    fs::{self},
};

use cpu::CPU;

use termios::*;

mod cpu;
mod flags;
mod memory;
mod opcode;

fn main() {
    // Configure Termios
    let stdin = 0;
    let mut termios = match Termios::from_fd(stdin) {
        Ok(termios) => termios,
        Err(e) => {
            eprintln!("Failed to initialize terminal: {}", e);
            return;
        }
    };
    termios.c_lflag &= !(ICANON | ECHO);
    if let Err(e) = tcsetattr(stdin, TCSANOW, &termios) {
        eprintln!("Failed to initialize terminal: {}", e);
        return;
    }

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
    if cpu.memory.load_program(&bytes).is_err() {
        eprintln!("Error loading program");
        return;
    }
    if cpu.execute_program().is_err() {
        eprintln!("Error running program");
    }
}

fn load_obj(filename: &str) -> Result<Vec<u16>, String> {
    let data = fs::read(filename).map_err(|e| format!("Problem reading the file: {}", e))?;
    let mut loaded_memory = Vec::new();
    for two_bytes in data.chunks_exact(2) {
        let first_byte = two_bytes.first().ok_or("Error obtaining the first byte")?;
        let second_byte = two_bytes.get(1).ok_or("Error obtaining the second byte")?;
        let joined_bytes = u16::from_be_bytes([*first_byte, *second_byte]);
        loaded_memory.push(joined_bytes);
    }

    Ok(loaded_memory)
}
