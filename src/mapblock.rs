use crate::config::Config;
use crate::terrain::TerrainCell;
use futures::stream::{Stream, StreamExt};
use minetestworld::MAPBLOCK_LENGTH;
use minetestworld::{MapBlock, Position};
use std::collections::{BinaryHeap, HashMap};
use std::ops::Range;

pub const CHUNK_SIZE: usize = MAPBLOCK_LENGTH as usize * MAPBLOCK_LENGTH as usize;

#[derive(Debug)]
pub struct Bbox {
    pub x: Range<i16>,
    pub z: Range<i16>,
}

impl Bbox {
    fn from_opt_ranges(x: Option<Range<i16>>, z: Option<Range<i16>>) -> Option<Bbox> {
        Some(Bbox { x: x?, z: z? })
    }
}

fn update_range(range: &mut Option<Range<i16>>, new_value: i16) {
    if let Some(range) = range {
        if new_value < range.start {
            range.start = new_value;
        } else if new_value >= range.end {
            range.end = new_value + 1;
        }
    } else {
        *range = Some(new_value..new_value + 1);
    }
}

/// Allows efficient traversing the map from above
///
/// For a given (x,z) key, the value will be a max heap of all y values.
type SortedPositions = HashMap<(i16, i16), BinaryHeap<i16>>;

/// Analyzes the given position stream and returns its Bbox and SortedPosition
///
/// Takes a stream that yields Result<Position, _>.
pub(crate) async fn analyze_positions<S, E>(mut positions: S) -> Result<(SortedPositions, Bbox), E>
where
    S: Stream<Item = Result<Position, E>> + Unpin,
{
    let mut sorted_positions = HashMap::new();
    let (mut x_range, mut z_range) = (None, None);
    while let Some(pos) = positions.next().await {
        let pos = pos?;
        update_range(&mut x_range, pos.x);
        update_range(&mut z_range, pos.z);
        let key = (pos.x, pos.z);
        let y_stack = sorted_positions.entry(key).or_insert_with(BinaryHeap::new);
        y_stack.push(pos.y);
    }
    Ok((
        sorted_positions,
        Bbox::from_opt_ranges(x_range, z_range).unwrap_or(Bbox { x: 0..0, z: 0..0 }),
    ))
}

pub fn compute_mapblock(
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
                let node = mapblock.get_node_at(Position::new(x, y, z));
                if let Some(color) = config.get_color(&node.param0) {
                    acc[index].add_background(*color);
                    if config.hill_shading.enabled
                        && acc[index].alpha() > config.hill_shading.min_alpha
                    {
                        acc[index].set_height(base_height + y as i16);
                    }
                    if acc[index].alpha() > config.sufficient_alpha {
                        acc[index].set_height(base_height + y as i16);
                        break;
                    }
                }
            }
        }
    }
}
