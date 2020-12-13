#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_comparisons)]
mod cpu;
mod gpu;
mod useful_func;

use cpu::CPU;
use minifb;

pub struct Emulator{
    cpu : CPU,
    window : minifb::Window,
}

impl Emulator {

    pub fn new () -> Emulator {
        let window = minifb::Window::new("GB", 160, 144, minifb::WindowOptions::default()).unwrap();
        Emulator {
            cpu : CPU::new(),
            window, 
        }
    }

    pub fn start(&mut self) {

        match self.cpu.bus.load_catridge() {
            Ok(()) => self.cpu.init_game(),
            Err(error) => println!("{}",error),
        }
    }

    pub fn update(&mut self) {
        println!("{}",self.cpu.pc);
        self.cpu.step();
        self.cpu.update_timers(4);
        self.cpu.update_graphics(4);
        self.cpu.do_interupts();

        self.render();
    }

    pub fn render(&mut self) {
        self.window.update_with_buffer(&self.cpu.bus.gpu.buffer, 160, 144).unwrap();
    }

}