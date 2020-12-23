use emulator::{self, Emulator};

fn main() {
    let mut emu = Emulator::new();
    println!("Press \"O\" to Select a ROM to load. ");
    while !emu.rom_available {
        emu.open();
    }

    loop {
        emu.emulate();
    }
}
