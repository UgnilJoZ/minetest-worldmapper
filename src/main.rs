use clap::Parser;
use minetestworld::World;
use std::fs;
use std::path::PathBuf;

mod color;
mod mapblock;
mod render;
use render::render_map;
mod config;
use config::Config;

#[cfg(feature = "smartstr")]
extern crate smartstring;

/// Render a minetest world into a map
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// World directory
    #[clap(short, long)]
    world: PathBuf,

    /// Config file
    #[clap(short, long)]
    config: PathBuf,

    /// The file in which to write the result
    #[clap(short, long)]
    output: PathBuf,
}

fn main() {
    #[cfg(feature = "smartstr")]
    smartstring::validate();
    let args = Args::parse();
    let config = fs::read_to_string(&args.config).unwrap();
    let config: Config = toml::from_str(&config).unwrap();
    let world = World::new(args.world);
    let map = world.get_map().unwrap();
    let picture = render_map(&map, &config).unwrap();
    eprintln!("Saving image");
    picture.save(&args.output).unwrap();
}
