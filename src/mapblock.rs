use crate::color::Color;
use crate::config::Config;
use crate::terrain::TerrainCell;
use minetestworld::MAPBLOCK_LENGTH;
use minetestworld::{MapBlock, Position};
use std::collections::{BinaryHeap, HashMap};

pub const CHUNK_SIZE: usize = MAPBLOCK_LENGTH as usize * MAPBLOCK_LENGTH as usize;

/// Sort all y values for all x,z mapblock position pairs
///
/// For a given (x,z) key, the value will be a max heap of all y values.
pub(crate) fn sorted_positions(positions: &[Position]) -> HashMap<(i16, i16), BinaryHeap<i16>> {
    let mut result = HashMap::new();
    for pos in positions.iter() {
        let key = (pos.x, pos.z);
        let y_stack = result.entry(key).or_insert_with(BinaryHeap::new);
        y_stack.push(pos.y);
    }
    result
}

pub(crate) fn compute_mapblock(
    mapblock: &MapBlock,
    config: &Config,
    base_height: i16,
    acc: &mut [TerrainCell; CHUNK_SIZE],
) {
    if mapblock.name_id_mappings.values().eq([b"air"]) {
        return;
    }
    for z in 0..MAPBLOCK_LENGTH {
        for x in 0..MAPBLOCK_LENGTH {
            let index = (x + MAPBLOCK_LENGTH * z) as usize;
            if acc[index].alpha() > 230 {
                continue;
            }

            for y in (0..MAPBLOCK_LENGTH).rev() {
                let node = mapblock.get_node_at(x, y, z);
                if let Some(color) = config.node_colors.get(&node.param0) {
                    acc[index].add_background(color.clone());
                    if config.hill_shading.enabled && acc[index].alpha() > config.hill_shading.min_alpha {
                        acc[index].set_height(base_height + y as i16);
                    }
                    if acc[index].alpha() > config.sufficient_alpha {
                        continue;
                    }
                }
            }
        }
    }
}
