// use std::thread;
// use std::time::Duration;
use emulator::{self, Emulator};
// use fltk::{app::*, button::*, frame::*, window::*};
// use std::sync::{Arc,Mutex};

fn main() {
    let mut emu = Emulator::new();
    emu.start();
    // let a = Arc::new(Mutex::new(emu.cpu));

    // thread::spawn( move || {
    //     let c = Arc::clone(&a);
    //     let app = App::default();
    //     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    //     let mut frame = Frame::new(0, 0, 400, 200, "");
    //     let mut but = Button::new(160, 210, 80, 40, "Click me!");
    //     let data = c.lock().unwrap();
    //     println!("Printing Registers = {:?}",data.registers);
    //     wind.end();
    //     wind.show();
    //     but.set_callback(move || frame.set_label("Hello World!"));
    //     app.run().unwrap();
    //     thread::sleep(Duration::from_millis(1));
    // });

    // let new_cpu_arc = Arc::clone(&a);
    loop {
        emu.update();
        // thread::sleep(Duration::from_millis(2));
    }


}
