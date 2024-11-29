use cpu::CPU;
use std::{
    env,
    fs::{self},
};
use termios::*;

mod cpu;
mod flags;
mod memory;
mod opcode;

fn main() {
    // Configure Termios
    let stdin = 0;
    let Ok(mut termios) = Termios::from_fd(stdin) else {
        eprintln!("Failed to initialize terminal");
        return;
    };

    termios.c_lflag &= !(ICANON | ECHO);

    if let Err(e) = tcsetattr(0, TCSANOW, &termios)
        .inspect_err(|e| eprintln!("Failed to initialize terminal: {}", e))
    {
        println!("{}", e);
        return;
    }

    let args: Vec<String> = env::args().collect();
    let Some(filename) = args.get(1) else {
        eprintln!("Failed to get the filename from args");
        return;
    };

    let bytes = match load_obj(filename) {
        Ok(bytes) => bytes,
        Err(_) => return,
    };

    let mut cpu = CPU::new();
    if let Err(err) = cpu.memory.load_program(&bytes) {
        eprintln!("Error loading program: {}", err);
        return;
    };
    if let Err(err) = cpu.execute_program() {
        eprintln!("Error running program: {}", err);
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
