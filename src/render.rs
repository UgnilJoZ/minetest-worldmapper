use crate::{
    color::Color, mapblock::compute_mapblock, mapblock::sorted_positions, mapblock::CHUNK_SIZE,
    Config,
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

pub async fn render_map(map: MapData, config: Config) -> Result<RgbaImage, Box<dyn Error>> {
    let mapblock_positions = map.all_mapblock_positions().await?;
    let mut xz_positions = sorted_positions(&mapblock_positions);
    let bbox = bounding_box(&mapblock_positions).unwrap_or(Bbox { x: 0..0, z: 0..0 });
    eprintln!("{bbox:?}");
    let mut imgbuf = RgbaImage::new(
        MAPBLOCK_LENGTH as u32 * bbox.x.len() as u32,
        MAPBLOCK_LENGTH as u32 * bbox.z.len() as u32 + 1,
    );
    let base_offset = (
        -bbox.x.start * MAPBLOCK_LENGTH as i16,
        bbox.z.end * MAPBLOCK_LENGTH as i16,
    );
    eprintln!("base offset: {base_offset:?}");

    let config = Arc::new(config);
    let map = Arc::new(map);
    let mut chunks = join_all(xz_positions.drain().map(|((x, z), ys)| {
        let config = config.clone();
        let map = map.clone();
        task::spawn(async move {
            let mut chunk = [Color(Rgba::from([0; 4])); CHUNK_SIZE];
            let mut ys = ys.clone();
            while let Some(y) = ys.pop() {
                match map.clone().get_mapblock(Position { x, y, z }).await {
                    Ok(mapblock) => compute_mapblock(&mapblock, &config, &mut chunk),
                    // An error here is noted, but the rendering continues
                    Err(e) => eprintln!("Error reading mapblock at {x},{y},{z}: {e}"),
                }
                if chunk.iter().all(|c| c.alpha() > config.target_alpha) {
                    break;
                }
            }
            (x, z, chunk)
        })
    }))
    .await;

    eprintln!("Writing image");
    for (x, z, chunk) in chunks.drain(..) {
        let offset_x = (base_offset.0 + MAPBLOCK_LENGTH as i16 * x) as u32;
        let offset_z = (base_offset.1 - MAPBLOCK_LENGTH as i16 * (z + 1)) as u32;
        render_mapblock_data(&chunk, &config, &mut imgbuf, (offset_x, offset_z));
    }
    Ok(imgbuf)
}
