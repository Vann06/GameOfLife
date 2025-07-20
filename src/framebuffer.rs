use raylib::prelude::*;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::BLACK; width * height],
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x]
        } else {
            Color::BLACK
        }
    }

    pub fn swap_buffers(&mut self, other: &mut Framebuffer) {
        std::mem::swap(&mut self.buffer, &mut other.buffer);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, scale: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.get_color(x, y);
                d.draw_rectangle(
                    (x * scale) as i32,
                    (y * scale) as i32,
                    scale as i32,
                    scale as i32,
                    color,
                );
            }
        }
    }
}
