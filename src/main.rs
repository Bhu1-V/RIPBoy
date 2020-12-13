use emulator;
fn main() {
    let mut e = emulator::Emulator::new();

    e.start();
    loop {
        e.update();
    }
}
