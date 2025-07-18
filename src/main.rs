use raylib::prelude::*;
use std::time::Duration;
use crate::framebuffer::*;
use crate::line::*;

mod framebuffer;
mod line;

const CELL_SIZE: usize = 10;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const GRID_WIDTH: usize = WIDTH / CELL_SIZE;
const GRID_HEIGHT: usize = HEIGHT / CELL_SIZE;

// ------------------ PATRONES ------------------

fn place_pulsar(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0,10), (5,10), (7,10), (12,10),
        (2,12), (3,12), (4,12), (8,12), (9,12), (10,12),
    ];

    for (dx, dy) in pattern.iter() {
        let px = x + dx;
        let py = y + dy;
        if px < GRID_WIDTH && py < GRID_HEIGHT {
            grid[py][px] = true;
        }
    }
}

fn place_block(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if y + 1 < GRID_HEIGHT && x + 1 < GRID_WIDTH {
        grid[y][x] = true;
        grid[y][x + 1] = true;
        grid[y + 1][x] = true;
        grid[y + 1][x + 1] = true;
    }
}

fn place_blinker(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH {
        grid[y][x] = true;
        grid[y][x + 1] = true;
        grid[y][x + 2] = true;
    }
}

fn place_glider(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 2 < GRID_WIDTH && y + 2 < GRID_HEIGHT {
        grid[y][x + 1] = true;
        grid[y + 1][x + 2] = true;
        grid[y + 2][x] = true;
        grid[y + 2][x + 1] = true;
        grid[y + 2][x + 2] = true;
    }
}

fn place_beacon(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    if x + 3 < GRID_WIDTH && y + 3 < GRID_HEIGHT {
        grid[y][x] = true;
        grid[y][x + 1] = true;
        grid[y + 1][x] = true;
        grid[y + 1][x + 1] = true;

        grid[y + 2][x + 2] = true;
        grid[y + 2][x + 3] = true;
        grid[y + 3][x + 2] = true;
        grid[y + 3][x + 3] = true;
    }
}

fn place_many_patterns(grid: &mut Vec<Vec<bool>>) {
    // Ojos con blocks
    for i in 0..4 {
        place_block(grid, 5, 5 + i * 4);
        place_block(grid, 30, 5 + i * 4);
    }

    // Boca con gliders, blinkers y blocks
    for i in 0..3 {
        place_glider(grid, 12 + i * 4, 22);
        place_block(grid, 12 + i * 4, 27);
        place_blinker(grid, 12 + i * 4, 32);
    }

    // Pulsars alrededor
    for i in 0..5 {
        for j in 0..3 {
            let x = 5 + i * 14;
            let y = 40 + j * 14;
            if x + 12 < GRID_WIDTH && y + 13 < GRID_HEIGHT {
                place_pulsar(grid, x, y);
            }
        }
    }
}

fn generate_pattern_grid() -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
    place_many_patterns(&mut grid);
    grid
}

// ------------------ MAIN ------------------

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Conway's Game of Life :3 Power Mode")
        .build();

    let mut fb = Framebuffer::new(WIDTH as u32, HEIGHT as u32);
    fb.set_background_color(Color::BLACK);

    let mut grid = generate_pattern_grid();

    while !rl.window_should_close() {
        fb.clear();

        grid = update_grid(&grid);
        draw_grid(&mut fb, &grid);

        fb.swap_buffers(&mut rl, &thread);
        std::thread::sleep(Duration::from_millis(100));
    }
}

// ------------------ GAME OF LIFE LOGIC ------------------

fn update_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = grid.clone();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let live_neighbors = count_neighbors(grid, x, y);
            let alive = grid[y][x];

            new_grid[y][x] = match (alive, live_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_grid
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut count = 0;

    for dy in [-1i32, 0, 1] {
        for dx in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && nx < GRID_WIDTH as i32 && ny < GRID_HEIGHT as i32 {
                if grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}

// ------------------ RENDER ------------------

fn draw_grid(fb: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] {
                let px = (x * CELL_SIZE) as f32;
                let py = (y * CELL_SIZE) as f32;

                let color = match (x * 17 + y * 29) % 6 {
                    0 => Color::YELLOW,
                    1 => Color::GREEN,
                    2 => Color::BLUE,
                    3 => Color::RED,
                    4 => Color::PURPLE,
                    _ => Color::MAGENTA,
                };

                fb.set_current_color(color);
                draw_square(fb, px, py, CELL_SIZE as f32);
            }
        }
    }
}

fn draw_square(fb: &mut Framebuffer, x: f32, y: f32, size: f32) {
    let x0 = x as i32;
    let y0 = y as i32;
    let x1 = (x + size) as i32;
    let y1 = (y + size) as i32;

    line(fb, x0, y0, x1, y0); // arriba
    line(fb, x1, y0, x1, y1); // derecha
    line(fb, x1, y1, x0, y1); // abajo
    line(fb, x0, y1, x0, y0); // izquierda
}
