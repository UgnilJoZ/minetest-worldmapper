use crate::color::Color;
use crate::mapblock::CHUNK_SIZE;
use minetestworld::MAPBLOCK_LENGTH;

#[derive(Default, Clone, Copy)]
pub struct TerrainCell {
    ground_color: Option<Color>,
    height: Option<i16>,
}

impl TerrainCell {
    pub fn alpha(&self) -> u8 {
        self.ground_color.map(|c| c.alpha()).unwrap_or(0)
    }

    pub fn add_background(&mut self, color: Color) {
        let new_color = self
            .ground_color
            .map(|c| c.with_background(&color))
            .unwrap_or(color);
        self.ground_color = Some(new_color);
    }

    pub fn set_height(&mut self, height: i16) {
        self.height = Some(height)
    }
}

pub struct Terrain {
    width: usize,
    height: usize,
    flat_data: Vec<TerrainCell>,
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Self {
        Terrain {
            width,
            height,
            flat_data: vec![Default::default(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Panics if out of bounds
    fn get_cell_mut(&mut self, x: u32, y: u32) -> &mut TerrainCell {
        self.flat_data
            .get_mut(y as usize * self.width + x as usize)
            .unwrap()
    }

    pub fn get_color(&self, x: u32, y: u32) -> Option<Color> {
        self.get_cell(x, y)?.ground_color
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&TerrainCell> {
        self.flat_data.get(y as usize * self.width + x as usize)
    }

    /// Panics if out of bounds
    pub fn insert_chunk(&mut self, offset: (u32, u32), chunk: [TerrainCell; CHUNK_SIZE]) {
        for (i, cell) in chunk.into_iter().enumerate() {
            let (mut x, mut y) = offset;
            x += i as u32 % MAPBLOCK_LENGTH as u32;
            y += MAPBLOCK_LENGTH as u32 - i as u32 / MAPBLOCK_LENGTH as u32;
            *self.get_cell_mut(x, y) = cell;
        }
    }

    pub fn height_diff_x(&self, x: u32, y: u32) -> Option<i16> {
        let a = self
            .get_cell(x.saturating_sub(1), y)
            .or_else(|| self.get_cell(x, y))?;
        let this_cell = self.get_cell(x, y)?.height?;
        let b = self.get_cell(x + 1, y).or_else(|| self.get_cell(x, y))?;
        let mut ascent = b.height? - this_cell;
        ensure_nonnegative(&mut ascent);
        let mut descent = a.height? - this_cell;
        ensure_nonnegative(&mut descent);
        Some(ascent.saturating_sub(descent))
    }

    pub fn height_diff_y(&self, x: u32, y: u32) -> Option<i16> {
        let a = self
            .get_cell(x, y.saturating_sub(1))
            .or_else(|| self.get_cell(x, y))?;
        let this_cell = self.get_cell(x, y)?.height?;
        let b = self.get_cell(x, y + 1).or_else(|| self.get_cell(x, y))?;
        let mut ascent = b.height? - this_cell;
        ensure_nonnegative(&mut ascent);
        let mut descent = a.height? - this_cell;
        ensure_nonnegative(&mut descent);
        Some(ascent.saturating_sub(descent))
    }
}

fn ensure_nonnegative(value: &mut i16) {
    if *value < 0 {
        *value = 0;
    }
}
