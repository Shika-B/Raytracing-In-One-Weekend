use crate::primitives::{Color, BLACK};

use std::fs;

pub struct Image {
    pub content: Vec<Color>,
    pub height: usize,
    pub width: usize,
}

impl Image {
    pub fn new_dark(height: usize, width: usize) -> Self {
        Self {
            content: vec![BLACK; height * width],
            height,
            width,
        }
    }
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.content[self.width * y + x] = color;
    }
    pub fn get(&self, x: usize, y: usize) -> Color {
        self.content[self.width * y + x]
    }
    pub fn write_to(&self, filename: &str) {
        let mut data = format!(
            "P3
{} {} 
255",
            self.width, self.height
        );
        for y in 0..self.height {
            data.push_str("\n");
            for x in 0..self.width {
                for channel in &self.get(x, y) {
                    let linear_channel = channel.min(1.0);
                    let gamma_channel = if linear_channel >= 0.0 {
                        linear_channel.sqrt()
                    } else {
                        0.0
                    };
                    let rgb_channel = (255.0 * gamma_channel).round() as u8;
                    data.push_str(&rgb_channel.to_string());
                    data.push_str(" ");
                }
            }
        }

        fs::write(filename, data).expect("Failed to save image");
    }
}
