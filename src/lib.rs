#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_comparisons)]
mod cpu;
mod gpu;
mod useful_func;

use std::{
    cmp::Ordering,
    process::exit,
    time::{Duration, Instant},
};

use cpu::CPU;
use minifb::{self, Key, KeyRepeat, Menu, ScaleMode};

const delta_interval: Duration = Duration::from_millis((1000.0 / 59.73) as u64);
const MAX_CYCLES: u32 = 69905;

pub struct Emulator {
    pub cpu: CPU,
    window: minifb::Window,
    once: bool,
    cycles: u128,
    times_renderes: u128,
    initalised_time: Instant,
    last_frame_time: Instant,
}

impl Emulator {
    pub fn new() -> Emulator {
        let win_opt = minifb::WindowOptions {
            topmost: false,
            transparency: false,
            borderless: false,
            title: true,
            resize: true,
            scale: minifb::Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
        };
        let mut menu = Menu::new("File").unwrap();
        menu.add_item("Load ROM", 100).shortcut(Key::O, 200).build();
        let mut window = minifb::Window::new("GB", 160, 144, win_opt).unwrap();
        window.add_menu(&menu);
        Emulator {
            cpu: CPU::new(),
            window,
            once: false,
            times_renderes: 0,
            cycles: 0,
            initalised_time: Instant::now(),
            last_frame_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        match self.cpu.bus.load_catridge() {
            Ok(()) => self.cpu.init_game(),
            Err(error) => println!("{}", error),
        }
        self.cpu.init_game();
    }

    pub fn emulate(&mut self) {
        let this_frame_time = Instant::now();

        let diff = this_frame_time
            .duration_since(self.last_frame_time)
            .as_millis();

        self.window.get_keys_pressed(KeyRepeat::Yes).map(|keys| {
            for t in keys {
                match t {
                    Key::A => {
                        println!("key Pressed A");
                        self.cpu.get_key_pressed(4);
                    }
                    Key::S => {
                        println!("key Pressed S");
                        self.cpu.get_key_pressed(5);
                    }
                    Key::Enter => {
                        println!("key Pressed Enter");
                        self.cpu.get_key_pressed(7);
                    }
                    Key::Space => {
                        println!("key Pressed Space");
                        self.cpu.get_key_pressed(6);
                    }

                    Key::Right => {
                        println!("key Pressed Right");
                        self.cpu.get_key_pressed(0);
                    }
                    Key::Left => {
                        println!("key Pressed Left");
                        self.cpu.get_key_pressed(1);
                    }
                    Key::Up => {
                        println!("key Pressed Up");
                        self.cpu.get_key_pressed(2);
                    }
                    Key::Down => {
                        println!("key Pressed Down");
                        self.cpu.get_key_pressed(3);
                    }
                    Key::C => {
                        println!("");
                        self.cpu.get_key_pressed(3);
                    }
                    Key::I => println!(
                        "CPU is Halted = {} PC = {:X} and OP code = {:X} {:X} {:X}",
                        self.cpu.is_halted,
                        self.cpu.pc,
                        self.cpu.bus.read_byte(self.cpu.pc),
                        self.cpu.bus.read_byte(self.cpu.pc + 1),
                        self.cpu.bus.read_byte(self.cpu.pc)
                    ),
                    _ => (),
                }
            }
        });

        self.window.get_keys_released().map(|keys| {
            for t in keys {
                match t {
                    Key::A => {
                        println!("key Realeased A");
                        self.cpu.set_key_relased(4);
                    }
                    Key::S => {
                        println!("key Realeased S");
                        self.cpu.set_key_relased(5);
                    }
                    Key::Enter => {
                        println!("key Realeased Enter");
                        self.cpu.set_key_relased(7);
                    }
                    Key::Space => {
                        println!("key Realeased Space");
                        self.cpu.set_key_relased(6);
                    }

                    Key::Right => {
                        println!("key Realeased Right");
                        self.cpu.set_key_relased(0);
                    }
                    Key::Left => {
                        println!("key Realeased Left");
                        self.cpu.set_key_relased(1);
                    }
                    Key::Up => {
                        println!("key Realeased Up");
                        self.cpu.set_key_relased(2);
                    }
                    Key::Down => {
                        println!("key Realeased Down");
                        self.cpu.set_key_relased(3);
                    }
                    _ => (),
                }
            }
        });

        let this_frame_time = Instant::now();

        if self
            .last_frame_time
            .checked_add(delta_interval)
            .unwrap()
            .cmp(&this_frame_time)
            == Ordering::Less
        {
            self.update();
            self.last_frame_time = this_frame_time;
        }
    }

    fn update(&mut self) {
        let mut cycles_this_updates = 0;

        while cycles_this_updates <= MAX_CYCLES {
            self.cpu.step();
            self.cpu.do_interupts();
            self.cpu.update_timers(self.cpu.m as u32);
            if self.cpu.update_graphics(self.cpu.m as i16) {
                // self._r();
                self.render();
            }
            self.cycles = self.cycles.wrapping_add(self.cpu.m as u128);
            cycles_this_updates += self.cpu.m as u32;
        }
    }

    // pub fn _r(&mut self){
    //     if self.started_time.elapsed() >= Duration::from_millis(18) {
    //         self.render();
    //         self.started_time = Instant::now();
    //     }
    // }

    pub fn render(&mut self) {
        self.times_renderes += 1;

        if self.window.is_open() {
            self.window
                .update_with_buffer(&self.cpu.bus.gpu.buffer, 160, 144)
                .unwrap();
        } else {
            println!(
                "Rendered Total of {} in {:?}",
                self.times_renderes,
                self.initalised_time.elapsed()
            );
            exit(0);
        }
    }
}
