// Note: Game BoyTM, Game Boy PocketTM, Super Game BoyTM and Game Boy ColorTM are registered trademarks of
// Nintendo CO., LTD. © 1989 to 1999 by Nintendo CO., LTD.
use gameboy::motherboard::MotherBoard;
use gameboy::gpu::{SCREEN_H, SCREEN_W};

use risc0_zkvm::guest::env;

fn main() {
    let mut rom = env::read::<String>();
    let mut mbrd = MotherBoard::power_up(rom.into_bytes());

    loop {
        // Breaking at an arbitrary cycle count
        if env::cycle_count() >= 1000000 {
            let mut window_buffer: Vec<u32> = vec![0; SCREEN_W * SCREEN_H];
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
            env::write(&window_buffer);
            break;
        }

        mbrd.next();

        if !mbrd.cpu.flip() {
            continue;
        }
    }
}
