use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

// ---------------------- PATRONES -----------------------

pub fn place_glider(fb: &mut Framebuffer, x: usize, y: usize) {
    let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    draw_pattern(fb, x, y, &pattern);
}

pub fn place_gun(fb: &mut Framebuffer, x: usize, y: usize) {
    let pattern = [
        (5, 1), (5, 2), (6, 1), (6, 2),
        (5, 11), (6, 11), (7, 11),
        (4, 12), (3, 13), (3, 14),
        (8, 12), (9, 13), (9, 14),
        (6, 15),
        (4, 16), (5, 17), (6, 17), (7, 17), (6, 18),
        (8, 16),
        (3, 21), (4, 21), (5, 21),
        (3, 22), (4, 22), (5, 22),
        (2, 23), (6, 23),
        (1, 25), (2, 25), (6, 25), (7, 25),
        (3, 35), (4, 35), (3, 36), (4, 36),
    ];
    draw_pattern(fb, x, y, &pattern);
}

pub fn place_blinker(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[(0, 0), (1, 0), (2, 0)]);
}

pub fn place_beacon(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)]);
}

pub fn place_loaf(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[(0,0), (1,1), (2,0), (2,1), (1,2), (3,1), (0,3)]);
}

pub fn place_boat(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[(0,0), (1,0), (0,1), (2,1), (1,2)]);
}

pub fn place_block(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

pub fn place_tub(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
}

pub fn place_toad(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[(1,0), (2,0), (3,0), (0,1), (1,1), (2,1)]);
}

pub fn place_light_weight_spaceship(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[
        (1,0), (4,0), (0,1), (0,2), (4,2), (0,3), (1,3), (2,3), (3,3)
    ]);
}

pub fn place_light_weight_spaceship_flipped(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[
        (0,1), (3,1), (4,2), (0,3), (1,3), (2,3), (3,3), (4,3),
    ]);
}

pub fn place_middle_weight_spaceship(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[
        (0,1), (0,2), (1,3), (2,3), (3,3), (4,3),
        (4,2), (4,1), (3,0), (1,0),
    ]);
}

pub fn place_middle_weight_spaceship_flipped(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[
        (0,2), (1,1), (1,3), (2,0), (2,4), (3,0), (3,4), (4,0), (4,4),
    ]);
}

pub fn place_heavy_weight_spaceship(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[
        (1,0), (2,0), (3,0), (4,0), (5,0), (6,0),
        (0,1), (6,1), (6,2), (0,3), (5,3)
    ]);
}

pub fn place_heavy_weight_spaceship_flipped(fb: &mut Framebuffer, x: usize, y: usize) {
    draw_pattern(fb, x, y, &[
        (0,1), (0,2), (0,3), (1,0), (1,4), (2,0), (2,4), (3,0), (3,4),
        (4,1), (4,2), (4,3), (2,5), (1,5),
    ]);
}

pub fn place_pulsar(fb: &mut Framebuffer, x: usize, y: usize) {
    let offsets = [
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
    draw_pattern(fb, x, y, &offsets);
}

// ---------------------- COLOREADO + DIBUJO -----------------------

fn draw_pattern(fb: &mut Framebuffer, base_x: usize, base_y: usize, pattern: &[(usize, usize)]) {
    for &(dx, dy) in pattern {
        let hue = ((base_x + dx + base_y + dy) % 360) as f32;
        let color = super::hsv_to_rgb(hue, 1.0, 1.0);
        fb.point(base_x + dx, base_y + dy, color);
    }
}

// ---------------------- DISTRIBUCIÓN -----------------------
pub fn populate_field(fb: &mut Framebuffer) {
    // Glider guns laterales
    place_gun(fb, 0, 5);
    place_gun(fb, 60, 5);

    // Pulsar central
    place_pulsar(fb, 44, 44);

    // ---------- LWSS ----------
    for y in (0..100).step_by(12) {
        place_light_weight_spaceship(fb, 0, y);
        place_light_weight_spaceship_flipped(fb, 95, y);
    }

    for x in (10..90).step_by(20) {
        place_light_weight_spaceship(fb, x, 20);
        place_light_weight_spaceship_flipped(fb, x, 75);
    }

    // ---------- MWSS ----------
    for x in (5..95).step_by(15) {
        place_middle_weight_spaceship(fb, x, 0);
        place_middle_weight_spaceship_flipped(fb, x, 95);
    }

    for y in (15..90).step_by(20) {
        place_middle_weight_spaceship(fb, 10, y);
        place_middle_weight_spaceship_flipped(fb, 85, y);
    }

    // ---------- HWSS ----------
    for &(x, y) in &[
        (0, 0), (90, 0), (0, 90), (90, 90), 
        (45, 0), (45, 90), 
        (0, 45), (90, 45), 
    ] {
        place_heavy_weight_spaceship(fb, x, y);
    }

    // ---------- Blinkers y Beacons ----------
    for &(x, y) in &[
        (20, 44), (68, 44), (44, 20), (44, 68),
        (28, 28), (60, 28), (28, 60), (60, 60),
    ] {
        place_blinker(fb, x, y);
    }

    place_beacon(fb, 34, 34);
    place_beacon(fb, 54, 34);
    place_beacon(fb, 34, 54);
    place_beacon(fb, 54, 54);

    // ---------- Still lifes: loaf, boat, block, tub ----------
    for &(x, y) in &[
        (10, 80), (30, 85), (50, 90), (70, 95),
        (15, 15), (80, 15), (15, 80), (80, 80)
    ] {
        place_loaf(fb, x, y);
        place_boat(fb, x + 3, y);
        place_block(fb, x + 6, y + 3);
        place_tub(fb, x + 9, y + 1);
    }

    // ---------- Más movimiento intermedio ----------
    for x in (20..80).step_by(12) {
        place_light_weight_spaceship(fb, x, 40);
        place_middle_weight_spaceship_flipped(fb, x, 55);
    }
}
