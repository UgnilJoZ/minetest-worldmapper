use crate::{
    color::Color, mapblock::compute_mapblock, mapblock::sorted_positions, mapblock::CHUNK_SIZE,
    Config, terrain::Terrain, terrain::TerrainCell
};
use async_std::task;
use futures::future::join_all;
use image::{Rgba, RgbaImage};
use minetestworld::MAPBLOCK_LENGTH;
use minetestworld::{positions::modulo, MapData, Position};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug)]
pub struct Bbox {
    pub x: Range<i16>,
    pub z: Range<i16>,
}

pub fn bounding_box(positions: &[Position]) -> Option<Bbox> {
    let mut x: Option<(i16, i16)> = None;
    let mut z: Option<(i16, i16)> = None;

    for pos in positions.iter() {
        if let &Some((xmin, xmax)) = &x {
            if pos.x < xmin {
                x = Some((pos.x, xmax));
            }

            if pos.x + 1 > xmax {
                x = Some((xmin, pos.x + 1));
            }
        } else {
            x = Some((pos.x, pos.x + 1));
        }

        if let &Some((zmin, zmax)) = &z {
            if pos.z < zmin {
                z = Some((pos.z, zmax));
            }

            if pos.z + 1 > zmax {
                z = Some((zmin, pos.z + 1));
            }
        } else {
            z = Some((pos.z, pos.z + 1));
        }
    }

    Some(Bbox {
        x: x?.0..x?.1,
        z: z?.0..z?.1,
    })
}

fn render_mapblock_data(
    data: &[Color; CHUNK_SIZE],
    config: &Config,
    image: &mut RgbaImage,
    offset: (u32, u32),
) {
    for (i, col) in data.iter().enumerate() {
        let (mut x, mut y) = offset;
        x += modulo(i as u32, MAPBLOCK_LENGTH as u32);
        y += MAPBLOCK_LENGTH as u32 - i as u32 / MAPBLOCK_LENGTH as u32;
        *image.get_pixel_mut(x, y) = col.with_background(&config.background_color).0;
    }
}

/// Renders the surface color of the terrain along with its heightmap
pub async fn compute_terrain(map: MapData, config: &Config) -> Result<Terrain, Box<dyn Error>> {
    let mapblock_positions = map.all_mapblock_positions().await?;
    let mut xz_positions = sorted_positions(&mapblock_positions);
    let bbox = bounding_box(&mapblock_positions).unwrap_or(Bbox { x: 0..0, z: 0..0 });
    log::info!("{bbox:?}");
    let mut terrain = Terrain::new(
        MAPBLOCK_LENGTH as usize * bbox.x.len(),
        MAPBLOCK_LENGTH as usize * bbox.z.len() + 1,
    );
    let base_offset = (
        -bbox.x.start * MAPBLOCK_LENGTH as i16,
        bbox.z.end * MAPBLOCK_LENGTH as i16,
    );

    let config = Arc::new(config.clone());
    let map = Arc::new(map);
    let mut chunks = join_all(xz_positions.drain().map(|((x, z), ys)| {
        let config = config.clone();
        let map = map.clone();
        task::spawn(async move {
            let mut chunk = [TerrainCell::default(); CHUNK_SIZE];
            let mut ys = ys.clone();
            while let Some(y) = ys.pop() {
                match map.get_mapblock(Position { x, y, z }).await {
                    Ok(mapblock) => compute_mapblock(&mapblock, &config, y * MAPBLOCK_LENGTH as i16, &mut chunk),
                    // An error here is noted, but the rendering continues
                    Err(e) => log::info!("Error reading mapblock at {x},{y},{z}: {e}"),
                }
                if chunk.iter().all(|c| c.alpha() > config.sufficient_alpha) {
                    break;
                }
            }
            (x, z, chunk)
        })
    }))
    .await;

    log::info!("Finishing surface map");
    for (x, z, chunk) in chunks.drain(..) {
        let offset_x = (base_offset.0 + MAPBLOCK_LENGTH as i16 * x) as u32;
        let offset_z = (base_offset.1 - MAPBLOCK_LENGTH as i16 * (z + 1)) as u32;
        terrain.insert_chunk((offset_x, offset_z), chunk)
    }
    Ok(terrain)
}

fn shade(color: &mut Color, height_diff: i16) {
    if height_diff < 0 {
        let descent: u8 = (-height_diff).try_into().unwrap_or(255);
        color.darken(descent);
    }
    if height_diff > 0 {
        let ascent: u8 = height_diff.try_into().unwrap_or(255);
        color.lighten_up(ascent);
    }
}

impl Terrain {
    fn heightdiff(&self, x: u32, y: u32) -> i16 {
        let x_diff = self.height_diff_x(x, y).unwrap_or(0);
        let y_diff = self.height_diff_y(x, y).unwrap_or(0);
        x_diff + y_diff
    }

    pub fn render(&self, config: &Config) -> RgbaImage {
        let mut image = RgbaImage::new(self.width().try_into().expect("width has to fit into u32"), self.height().try_into().expect("height has to fit into u32"));
        for y in 0..self.height() {
            let y = y as u32;
            for x in 0..self.width() {
            let x = x as u32;
                let mut col = self.get_color(x, y).unwrap_or(config.background_color);
                if config.hill_shading.enabled {
                    shade(&mut col, self.heightdiff(x, y));
                }
                *image.get_pixel_mut(x, y) = col.with_background(&config.background_color).0;
            }
        }
        image
    }
}
