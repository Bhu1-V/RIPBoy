// use std::thread;
// use std::time::Duration;
use emulator::{self, Emulator};
// use fltk::{app::*, button::*, frame::*, window::*};
// use std::sync::{Arc,Mutex};

fn main() {
    let mut emu = Emulator::new();
    emu.start();

    loop {
        emu.emulate();
    }
}
