extern crate sdl2;
extern crate virtual_nes;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::fs::File;
use std::time::{Duration, Instant};
use virtual_nes::VirtualConsole;

pub fn main() {
    let name = "Megaman II (E) [!]";
    let mut f = File::open(format!("./roms/{}.nes", name)).unwrap();
    let mut console = VirtualConsole::new(&mut f).unwrap();
    console.reset();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut timer = sdl_context.timer().unwrap();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let window = video_subsystem
        .window("gilneas", 256, 240)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    //let mut widnow_surface = window.surface(&event_pump).unwrap();

    //let mut canvas = window.into_canvas().build().unwrap();

    //let texture_creator = canvas.texture_creator();
    // let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 240).unwrap();
    //let surface = Surface::new(256, 240, PixelFormatEnum::RGB24).unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    //let mut event_pump = event_pump.clone();
    // let mut i = 0;
    let mut timestamp = 0;
    // let mut ts = Instant::now();
    'running: loop {
        // let now = Instant::now();
        // let dt = (now - ts).as_secs();
        // ts = now;
        let ts = timer.ticks();// / 1000;
        let dt = ts - timestamp;
        println!("millis: {}", dt);
        timestamp = ts;
        // if timestamp >= 2 {break 'running;}
        flame::start("console");
        console.step_seconds(dt as i64);
        flame::end("console");
        

        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();

        flame::start("texture update");
        //texture.update(None, console.get_pixels(), 3 * 256).unwrap();
        let mut pixels = console.get_pixels().to_owned();
        let surface = Surface::from_data(&mut pixels, 256, 240, 3 * 256, PixelFormatEnum::RGB24).unwrap();
        flame::end("texture update");
        flame::start("canvas copy");
        // canvas.copy(&texture, None, None).expect("Render failed");
        let mut widnow_surface = window.surface(&event_pump).unwrap();
        surface.blit(None, &mut widnow_surface, None).unwrap();
        widnow_surface.finish().unwrap();
        flame::end("canvas copy");
        //canvas.present();
        

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        //canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    //console.dump();
}
