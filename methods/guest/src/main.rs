// Note: Game BoyTM, Game Boy PocketTM, Super Game BoyTM and Game Boy ColorTM are registered trademarks of
// Nintendo CO., LTD. © 1989 to 1999 by Nintendo CO., LTD.
use gameboy::motherboard::MotherBoard;
use gameboy::gpu::{SCREEN_H, SCREEN_W};

use risc0_zkvm::guest::env;

fn main() {
    let mut rom = String::from(env::read::<String>());
    let mut mbrd = MotherBoard::power_up(&rom);
    let rom_name = mbrd.mmu.borrow().cartridge.title();

    loop {
        if env::cycle_count() >= 1000000 {
            break;
        }
        // this needs a break condition
        mbrd.next();

        if !mbrd.cpu.flip() {
            continue;
        }
    }
    // read the input
}
