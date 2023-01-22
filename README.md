# minetest-worldmapper

[![Build](https://github.com/UgnilJoZ/minetest-worldmapper/actions/workflows/rust.yaml/badge.svg)](https://github.com/UgnilJoZ/minetest-worldmapper/actions/workflows/rust.yaml)
[![Crates.io](https://img.shields.io/crates/v/minetest-worldmapper.svg)](https://crates.io/crates/minetest-worldmapper)
[![Dependency Status](https://deps.rs/crate/minetest-worldmapper/0.3.2/status.svg)](https://deps.rs/crate/minetest-worldmapper/0.3.2)

A multi-threaded mapper for Minetest with node transparency support.

It generates a view of the world from above, rendering one pixel per voxel.

This project is currently tested on Linux only. If you have Windows and it doesn't work, please file a [bug report](https://github.com/UgnilJoZ/minetest-worldmapper/issues). But if you have another OS and it does not work, please report also a bug.

## [Example picture](https://github.com/UgnilJoZ/minetest-worldmapper/wiki/Images)
![Zoomed map](https://user-images.githubusercontent.com/7910828/213939971-6e568b96-6ae3-4ea3-8176-1a4b93655265.png)
![Example map](https://user-images.githubusercontent.com/7910828/213940057-c38e87b1-f44f-47d7-891e-62ea715ab226.png)

## Usage
First, compile the project with cargo:

```bash
cargo build --release
```

### Command-line arguments

Then, call the executable `target/release/minetest-worldmapper` with the three required arguments:

|  Option  | Short | Description                               |
| -------- | ----- | ----------------------------------------- |
| --world  | -w    | The directory of the world to render.     |
| --config | -c    | The config file. The format should follow the [config format][1]. |
| --output | -o    | The image file which the map should be rendered to. |

### Logging
Via the [`RUST_LOG`](https://docs.rs/env_logger/latest/env_logger/#enabling-logging) environment variable, you can choose one out of the log levels `trace`, `debug`, `info`, `warn`, and `error`. The default is `error`.

### Example usage
```bash
minetest-worldmapper --world TestWorld/ --config config.example.toml --output map.png
```

### Example usage with logging
```bash
RUST_LOG=debug minetest-worldmapper --world TestWorld/ --config config.example.toml --output map.png
```

### Config file
If a voxel is rendered and its color are entirely determined by the config file, which is based on TOML.
An [example config file][2] is part of this repo. Its main purpose is to map the voxel content to colors.

| Config option           | Type         | Description                         |
| ----------------------- | ------------ | ----------------------------------- |
| `sufficient_alpha`      | [Integer][3] | (optional, defaults to `230`) When determining a pixel's color, stop going through transparent nodes when reaching this opacity value. Between 0 and 255. |
| `background_color`      | [String][4]  | Hex color string; either in the format "rrggbb" (full opacity) or "rrggbbaa" (color with alpha value). Serves as a fallback color if all voxels for a given pixel are exhausted and there is transparency left. |
| `hillshading.enabled`   | [Boolean][3] | (optional, defaults to `true`) Enables terrain relief visualisation. |
| `hillshading.min_alpha` | [Integer][6] | (optional, defaults to `128`) At which alpha value a node counts as "terrain" |
| `node_colors`           | [Table][5]   | Maps node [itemstrings][7] to color strings (which have the same format as `background_color`). Every node not listed here is treated like air. |

#### Minimal config example
```toml
background_color = "888888"

[node_colors]
"default:water_source" = "00228888"
```

## Current limitations
* LevelDB is not supported as backend.
* Only map chunks with map format version 29 (the current) are supported.


## Reading Minetest worlds with Rust
The crate [minetestworld](https://github.com/UgnilJoZ/rust-minetestworld/) is the basis for this renderer.

[1]: #config-file
[2]: https://github.com/UgnilJoZ/minetest-worldmapper/blob/main/config.example.toml
[3]: https://toml.io/en/v1.0.0#integer
[4]: https://toml.io/en/v1.0.0#string
[5]: https://toml.io/en/v1.0.0#table
[6]: https://toml.io/en/v1.0.0#boolean
[7]: https://wiki.minetest.net/Itemstrings
