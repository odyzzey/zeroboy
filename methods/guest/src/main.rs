// Note: Game BoyTM, Game Boy PocketTM, Super Game BoyTM and Game Boy ColorTM are registered trademarks of
// Nintendo CO., LTD. © 1989 to 1999 by Nintendo CO., LTD.
use gameboy::motherboard::MotherBoard;
use gameboy::gpu::{SCREEN_H, SCREEN_W};

use risc0_zkvm::guest::env;
const ROM: &[u8] = include_bytes!("../../rom.gb");
fn main() {
    //env::write(&env::cycle_count().to_string());
    let cycle_limit: u64 = env::read();

    let mut mbrd = MotherBoard::power_up(ROM.to_vec());
    loop {
        // Breaking at an arbitrary cycle count
        if env::cycle_count() >= cycle_limit as usize {
            /* let mut window_buffer: Vec<u32> = vec![0; SCREEN_W * SCREEN_H];
            let mut i: usize = 0;
            for l in mbrd.mmu.borrow().gpu.data.iter() {
                for w in l.iter() {
                    let b = u32::from(w[0]) << 16;
                    let g = u32::from(w[1]) << 8;
                    let r = u32::from(w[2]);
                    let a = 0xff00_0000;

                    window_buffer[i] = a | b | g | r;
                    i += 1;
                }
            }
            env::commit(&window_buffer); */
            // env::commit(&get_registers(&mbrd));
            // env::write(&get_registers(&mbrd));
            println!("{}", get_registers(&mbrd));
            env::exit(0);
        }

        mbrd.next();

        if !mbrd.cpu.flip() {
            continue;
        }
    }
}

fn get_registers(mbrd: &MotherBoard) -> String {
    let mut registers = String::new();
    registers.push_str(&format!("A: {:02X} ", mbrd.cpu.cpu.reg.a));
    registers.push_str(&format!("F: {:02X} ", mbrd.cpu.cpu.reg.f));
    registers.push_str(&format!("B: {:02X} ", mbrd.cpu.cpu.reg.b));
    registers.push_str(&format!("C: {:02X} ", mbrd.cpu.cpu.reg.c));
    registers.push_str(&format!("D: {:02X} ", mbrd.cpu.cpu.reg.d));
    registers.push_str(&format!("E: {:02X} ", mbrd.cpu.cpu.reg.e));
    registers.push_str(&format!("H: {:02X} ", mbrd.cpu.cpu.reg.h));
    registers.push_str(&format!("L: {:02X} ", mbrd.cpu.cpu.reg.l));
    registers
}
