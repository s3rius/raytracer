use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use crate::color::Color;

pub struct PPMImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl PPMImage {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width,
            height,
            data: (0..width * height).map(|_| Color::BLACK).collect(),
        };
    }

    fn get_index(&self, x: usize, y: usize) -> anyhow::Result<usize> {
        let index = y * self.width + x;
        if index >= self.data.len() {
            anyhow::bail!("Pixel coordinates out of bounds");
        }
        Ok(index)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) -> anyhow::Result<()> {
        let index = self.get_index(x, y)?;
        self.data[index] = *color;
        Ok(())
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> anyhow::Result<Color> {
        let index = self.get_index(x, y)?;
        Ok(self.data[index])
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> anyhow::Result<&mut Color> {
        let index = self.get_index(x, y)?;
        Ok(&mut self.data[index])
    }

    pub fn save(&self, filename: &str) -> anyhow::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;
        let mut writer = BufWriter::new(file);
        writeln!(&mut writer, "P3")?;
        writeln!(&mut writer, "{} {}", self.width, self.height)?;
        writeln!(&mut writer, "255")?;

        for color in &self.data {
            writeln!(&mut writer, "{} {} {}", color.r, color.g, color.b)?;
        }

        Ok(())
    }
}

/// Convert from list of list of colors to an image.
///
/// First vec should represent y row,
/// and each inner list is x row.
impl From<Vec<Vec<Color>>> for PPMImage {
    fn from(value: Vec<Vec<Color>>) -> Self {
        let height = value.len();
        let width = value.iter().flatten().count() / height;
        Self {
            height,
            width,
            data: value.into_iter().flatten().collect(),
        }
    }
}
