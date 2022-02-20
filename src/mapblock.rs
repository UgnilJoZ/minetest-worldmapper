use crate::color::Color;
use crate::config::Config;
use image::{Rgba, RgbaImage};
use minetestworld::{MapBlock, MapData, MapDataError, Position};
use minetestworld::positions::modulo;
use std::collections::{BinaryHeap, HashMap};
use std::ops::Range;

/// Sort all y values for all x,z mapblock position pairs
///
/// For a given (x,z) key, the value will be a min heap of all y values.
pub(crate) fn sorted_positions(positions: &[Position]) -> HashMap<(i16, i16), BinaryHeap<i16>> {
    let mut result = HashMap::new();
    for pos in positions.iter() {
        let key = (pos.x, pos.z);
        let y_stack = result.entry(key).or_insert(BinaryHeap::new());
        y_stack.push(pos.y);
    }
    result
}

pub(crate) fn compute_mapblock(
    mapblock: &MapBlock,
    colours: &HashMap<String, Color>,
    acc: &mut [Color; 256]
) {
    for z in 0..16 {
        for x in 0..16 {
            let index = (x + 16 * z) as usize;
            if acc[index].alpha() > 230 {
                continue;
            }

            for y in (0..16).rev() {
                let node = mapblock.get_node_at(x, y, z);
                if let Some(colour) = colours.get(&node.param0) {
                    acc[index] = colour.clone();
                    break;
                }
            }
        }
    }
}

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

fn render_mapblock_data(data: &[Color; 256], config: &Config, image: &mut RgbaImage, offset: (u32, u32)) {
    for (i, col) in data.iter().enumerate() {
        let (mut x, mut y) = offset;
        x += modulo(i as u32, 16);
        y += 16 - i as u32 / 16;
        *image.get_pixel_mut(x, y) = col.0;
    }
}

pub fn render_map(map: &MapData, config: &Config) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let mapblock_positions = map.all_mapblock_positions()?;
    let mut xz_positions = sorted_positions(&mapblock_positions);
    let bbox = bounding_box(&mapblock_positions).unwrap_or(Bbox {x: 0..0, z: 0..0});
    eprintln!("BBox: {bbox:?}");
    let mut imgbuf = RgbaImage::new(16 * bbox.x.len() as u32, 16 * bbox.z.len() as u32 + 1);
    let base_offset = (-bbox.x.start * 16, bbox.z.end * 16);
    eprintln!("base offset: {base_offset:?}");

    for (&(x, z), ys) in xz_positions.iter_mut () {
        eprintln!("Processing x={x}, z={z} bar.");
        let mut colordata = [Color(Rgba::from([0; 4])); 256];
        while let Some(y) = ys.pop() {
            match map.get_mapblock(Position {x, y, z: z}) {
                Ok(mapblock) => compute_mapblock(&mapblock, &config.node_colors, &mut colordata),
                // An error here is noted, but the rendering continues
                Err(e) => eprintln!("Error reading mapblock at {x},{y},{z}: {e}"),
            }
        }
        
        let offset_x = (base_offset.0 + 16 * x) as u32;
        let offset_z = (base_offset.1 - 16 * (z+1)) as u32;
        render_mapblock_data(&colordata, &config, &mut imgbuf, (offset_x, offset_z));
    }
    Ok(imgbuf)
}
