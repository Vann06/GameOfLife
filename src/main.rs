mod framebuffer;
mod patterns;
use raylib::prelude::*;
use framebuffer::Framebuffer;
use patterns::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SCALE: usize = 8;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size((WIDTH * SCALE) as i32, (HEIGHT * SCALE) as i32)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(10);

    let mut fb1 = Framebuffer::new(WIDTH, HEIGHT);
    let mut fb2 = Framebuffer::new(WIDTH, HEIGHT);

    populate_field(&mut fb1);
    

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        fb1.draw(&mut d, SCALE);
        step(&fb1, &mut fb2);
        fb1.swap_buffers(&mut fb2);
    }
}

fn step(current: &Framebuffer, next: &mut Framebuffer) {
    for y in 0..HEIGHT as i32 {
        for x in 0..WIDTH as i32 {
            let mut neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && ny >= 0 && (nx as usize) < WIDTH && (ny as usize) < HEIGHT {
                        if current.get_color(nx as usize, ny as usize) != Color::BLACK {
                            neighbors += 1;
                        }
                    }
                }
            }

            let alive = current.get_color(x as usize, y as usize) != Color::BLACK;
            let next_state = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            if next_state {
                let hue = ((x as f32 / WIDTH as f32 + y as f32 / HEIGHT as f32) * 180.0) % 360.0;
                let color = hsv_to_rgb(hue, 1.0, 1.0);
                next.point(x as usize, y as usize, color);
            } else {
                next.point(x as usize, y as usize, Color::BLACK);
            }
        }
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    Color::new(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
        255,
    )
}
