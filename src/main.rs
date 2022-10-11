use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use std::time::Duration;


struct Vector2 {x: f32, y: f32}

struct Spaceship {pos: Vector2, vel: Vector2}

fn calc_dist(point_a: Vector2, point_b: Vector2) -> f32 {
    return ((point_b.x - point_a.x).powf(2.0) + (point_b.y - point_a.y).powf(2.0)).powf(0.5);
}

fn update_spaceship(target: &mut Spaceship, delta_time: f32) {
    target.pos.x = target.pos.x + (target.vel.x * delta_time);
    target.pos.y = target.pos.y + (target.vel.y * delta_time);
    if target.vel.x <= 0.10 {
        target.vel.x = 0.0;
    } else {
        target.vel.x = target.vel.x - target.vel.x * 0.25;
    }
    if target.vel.y <= 0.10 {
        target.vel.y = 0.0;
    } else {
        target.vel.y = target.vel.y - target.vel.y * 0.25;
    }
}

fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn main() -> Result<(), String> {
    // Sets up dependencies
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Creates window
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    
    // Creates canvas
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i));

        // Sleepy time
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
