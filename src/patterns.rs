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
    // Glider guns (infinitos)
    place_gun(fb, 0, 10);    
    place_gun(fb, 60, 10);   

    // Pulsar central
    place_pulsar(fb, 44, 44);

    // LWSS desde los lados horizontales
    for y in (2..90).step_by(8) {
        place_light_weight_spaceship(fb, 0, y);     
        place_light_weight_spaceship_flipped(fb, 90, y); 
    }

    // MWSS desde arriba y abajo
    for x in (2..90).step_by(10) {
        place_middle_weight_spaceship(fb, x, 0);    
        place_middle_weight_spaceship_flipped(fb, x, 90); 
    }

    // HWSS en esquinas y diagonales
    for i in (0..80).step_by(16) {
        place_heavy_weight_spaceship(fb, i, i);
        place_heavy_weight_spaceship_flipped(fb, 90 - i, i);
    }

    // Refuerzo de LWSS dentro del campo
    for &(x, y) in &[
        (10, 40), (25, 55), (40, 25), (60, 70), (75, 40)
    ] {
        place_light_weight_spaceship(fb, x, y);
    }

    // MWSS cruzados cerca del centro
    for &(x, y) in &[
        (20, 35), (35, 20), (50, 65), (65, 50)
    ] {
        place_middle_weight_spaceship(fb, x, y);
    }

    // Gliders cruzados para más caos
    for i in (2..88).step_by(8) {
        place_glider(fb, i, i);
        place_glider(fb, 90 - i, i);
    }

    // Still lifes (estáticos) en zonas vacías
    for &(x, y) in &[
        (5, 85), (15, 78), (25, 82), (35, 88),
        (50, 85), (65, 75), (80, 85), (85, 78)
    ] {
        place_block(fb, x, y);
    }

    for &(x, y) in &[
        (10, 65), (30, 68), (45, 70), (75, 65)
    ] {
        place_tub(fb, x, y);
    }

    for &(x, y) in &[
        (20, 80), (55, 78), (70, 88)
    ] {
        place_boat(fb, x, y);
    }

    for &(x, y) in &[
        (40, 80), (60, 75)
    ] {
        place_loaf(fb, x, y);
    }

    // Toads (osciladores pequeños) como adorno inferior
    for &(x, y) in &[
        (12, 95), (32, 93), (52, 96), (72, 94)
    ] {
        place_toad(fb, x, y);
    }

        // 🌟 Reanimar parte superior con más spaceships y gliders
    for &(x, y) in &[
        (10, 2), (25, 3), (40, 2), (55, 4), (70, 3)
    ] {
        place_light_weight_spaceship(fb, x, y);
    }

    for &(x, y) in &[
        (15, 6), (30, 7), (45, 6), (60, 8)
    ] {
        place_middle_weight_spaceship(fb, x, y);
    }

    for &(x, y) in &[
        (20, 10), (35, 10), (50, 10), (65, 10)
    ] {
        place_glider(fb, x, y);
    }

    // ✨ Osciladores en la franja superior
    for &(x, y) in &[
        (12, 5), (32, 4), (52, 6), (72, 5)
    ] {
        place_toad(fb, x, y);
    }

    // 🧱 Still lifes arriba como bloques decorativos
    for &(x, y) in &[
        (5, 0), (20, 0), (35, 1), (50, 0), (70, 0)
    ] {
        place_block(fb, x, y);
    }

    for &(x, y) in &[
        (15, 2), (45, 3), (65, 1)
    ] {
        place_tub(fb, x, y);
    }

    for &(x, y) in &[
        (25, 1), (60, 2)
    ] {
        place_boat(fb, x, y);
    }

        // 🚀 LWSS desde el borde derecho hacia el centro (horizontal)
    for &(x, y) in &[
        (90, 10), (90, 25), (90, 40), (90, 55), (90, 70)
    ] {
        place_light_weight_spaceship_flipped(fb, x, y);
    }

    // 🚀 MWSS en diagonal desde el borde derecho
    for (i, x) in (65..90).step_by(5).enumerate() {
        place_middle_weight_spaceship_flipped(fb, x, 10 + i * 10);
    }

    // 🌀 Gliders descendiendo en zigzag desde la esquina superior derecha
    for &(x, y) in &[
        (85, 0), (80, 10), (75, 20), (70, 30), (65, 40)
    ] {
        place_glider(fb, x, y);
    }

    // ✨ Toads laterales
    for &(x, y) in &[
        (85, 20), (85, 40), (85, 60)
    ] {
        place_toad(fb, x, y);
    }

    // 🧱 Still lifes (tubs y boats) decorando la derecha
    for &(x, y) in &[
        (88, 5), (88, 30), (88, 50), (88, 70)
    ] {
        place_tub(fb, x, y);
    }

    for &(x, y) in &[
        (86, 15), (86, 35), (86, 65)
    ] {
        place_boat(fb, x, y);
    }

    for &(x, y) in &[
        (84, 25), (84, 45)
    ] {
        place_block(fb, x, y);
    }


}
