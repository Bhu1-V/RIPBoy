#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_comparisons)]
mod cpu;
mod gpu;
mod useful_func;

use std::{cmp::Ordering, path::PathBuf, process::exit, time::{Duration, Instant}};

use cpu::CPU;
use minifb::{self, Key, KeyRepeat, Menu, ScaleMode};
use wfd::{self, DialogParams};

const DELTA_INTERVAL: Duration = Duration::from_millis((1000.0 / 59.73) as u64);
const MAX_CYCLES: u32 = 69905;

pub struct Emulator {
    cpu: CPU,
    pub window: minifb::Window,
    cycles: u128,
    game_rom_path : PathBuf,
    times_renderes: u128,
    pub rom_available : bool,
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
        let mut window = minifb::Window::new("Press \"O\" to Load The GB ROM.", 160, 144, win_opt).unwrap();
        window.limit_update_rate(Some(Duration::from_micros(16600)));
        // let mut current_path = std::env::current_dir().unwrap();
        // current_path.push("retroid.gb");
        Emulator {
            cpu: CPU::new(),
            window,
            times_renderes: 0,
            cycles: 0,
            game_rom_path : PathBuf::default(),
            initalised_time: Instant::now(),
            last_frame_time: Instant::now(),
            rom_available : false,
        }
    }

    pub fn open(&mut self) {
        if self.window.is_open() {
            self.window.get_keys_pressed(KeyRepeat::No).map(|keys| {
                for t in keys {
                    match t {
                        Key::O => {
                            self.get_game_rom();
                        }
                        
                        _ => ()
                    }
                }
            });
            self.window.update();
        } else {
            println!(
                "Rendered Total of {} in {:?}",
                self.times_renderes,
                self.initalised_time.elapsed()
            );
            exit(0);
        }
    }

    pub fn start(&mut self) {
        println!("{:?}",self.game_rom_path);
        match self.cpu.bus.load_catridge(&self.game_rom_path) {
            Ok(()) => self.cpu.init_game(),
            Err(error) => println!("{}", error),
        }
        self.cpu.init_game();
    }

    pub fn emulate(&mut self) {

        if self.game_rom_path == PathBuf::default() {
            return;
        }

        self.window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                match t {
                    Key::O => {
                        self.get_game_rom();

                    }

                    _ => ()
                }
            }
        });

        self.window.get_keys_pressed(KeyRepeat::Yes).map(|keys| {
            for t in keys {
                match t {
                    Key::A => {
                        self.cpu.get_key_pressed(4);
                    }
                    Key::S => {
                        
                        self.cpu.get_key_pressed(5);
                    }
                    Key::Enter => {
                        
                        self.cpu.get_key_pressed(7);
                    }
                    Key::Space => {
                        
                        self.cpu.get_key_pressed(6);
                    }

                    Key::Right => {
                        
                        self.cpu.get_key_pressed(0);
                    }
                    Key::Left => {
                        
                        self.cpu.get_key_pressed(1);
                    }
                    Key::Up => {
                        
                        self.cpu.get_key_pressed(2);
                    }
                    Key::Down => {
                        
                        self.cpu.get_key_pressed(3);
                    }
                    _ => (),
                }
            }
        });

        self.window.get_keys_released().map(|keys| {
            for t in keys {
                match t {
                    Key::A => {
        
                        self.cpu.set_key_relased(4);
                    }
                    Key::S => {
        
                        self.cpu.set_key_relased(5);
                    }
                    Key::Enter => {
                        
                        self.cpu.set_key_relased(7);
                    }
                    Key::Space => {
                        
                        self.cpu.set_key_relased(6);
                    }

                    Key::Right => {
                        
                        self.cpu.set_key_relased(0);
                    }
                    Key::Left => {
                    
                        self.cpu.set_key_relased(1);
                    }
                    Key::Up => {
            
                        self.cpu.set_key_relased(2);
                    }
                    Key::Down => {
                    
                        self.cpu.set_key_relased(3);
                    }
                    _ => (),
                }
            }
        });

        let this_frame_time = Instant::now();

        if self
            .last_frame_time
            .checked_add(DELTA_INTERVAL)
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

    fn get_game_rom(&mut self) {
        let params = DialogParams {
            default_extension : ".gb",
            ok_button_label : "Emulate",
            file_name_label : "GB ROM",
            file_types : vec![("GameBoy ROMS (*.gb)", "*.gb;*.gbc")],
            title : "LOAD GB ROM",
            ..DialogParams::default()
        };

        let open_result = wfd::open_dialog(params).unwrap();
        self.game_rom_path = open_result.selected_file_path;
        self.window.set_title("RIP BOY");
        self.rom_available = true;
        self.start();
    }

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
