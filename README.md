# minetest-worldmapper

[![Build](https://github.com/UgnilJoZ/minetest-worldmapper/actions/workflows/rust.yaml/badge.svg)](https://github.com/UgnilJoZ/minetest-worldmapper/actions/workflows/rust.yaml)
[![Crates.io](https://img.shields.io/crates/v/minetest-worldmapper.svg)](https://crates.io/crates/minetest-worldmapper)
[![Dependency Status](https://deps.rs/crate/minetest-worldmapper/0.2.4/status.svg)](https://deps.rs/crate/minetest-worldmapper/0.2.4)

A multi-threaded mapper for Minetest with node transparency support.

It generates a view of the world from above, rendering one pixel per voxel.

## Usage
First, compile the project with cargo:

```bash
cargo build --release
```

Then, call the executable `target/release/minetest-worldmapper` with the three required arguments:

|  Option  | Short | Description                               |
| -------- | ----- | ----------------------------------------- |
| --world  | -w    | The directory of the world to render.     |
| --config | -c    | The config file. The format should follow the [config format][1]. |
| --output | -o    | The image file which the map should be rendered to. |

### Example usage
```bash
minetest-worldmapper --world TestWorld/ --config config.example.toml --output map.png
```

### Config file
If a voxel is rendered and its color are entirely determined by the config file. An [example config file][2] is part of this repo. Its main purpose is to map content IDs to colors.

| Config option      | Type         | Description                         |
| ------------------ | ------------ | ----------------------------------- |
| `version`          | [Integer][3] | Config version. Currently always 1. |
| `background_color` | [String][4]  | Hex color string in the format "rrggbb" or "rrggbbaa". Serves as a fallback color if all voxels for a given pixel are exhausted and there is transparency left. |
| `sufficient_alpha` | [Integer][3] | When determining a pixel's color, stop going through transparent nodes when reaching this opacity value. Between 0 and 255. |
| `node_colors`      | [Table][5]  | A mapping from content names to color strings in the same format as `background_color`. |

## Current limitations
* LevelDB is not supported as backend.
* Only map chunks with map format version 29 (the current) are supported.
* No hillshading feature yet.

## Example pictures
![Zoomed in](https://user-images.githubusercontent.com/7910828/154993848-744bd8f6-782e-4048-8f8d-3871e53cdc0a.png)
![Big map](https://user-images.githubusercontent.com/7910828/154993962-51475253-4eed-4d5a-8427-694949423a9d.png)

## Reading Minetest worlds with Rust
The crate [minetestworld](https://github.com/UgnilJoZ/rust-minetestworld/) is the basis for this renderer.

[1]: #config-file
[2]: https://github.com/UgnilJoZ/minetest-worldmapper/blob/main/config.example.toml
[3]: https://toml.io/en/v1.0.0#integer
[4]: https://toml.io/en/v1.0.0#string
[5]: https://toml.io/en/v1.0.0#table
