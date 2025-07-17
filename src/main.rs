use clap::Parser;
use log_err::LogErrResult;
use minetestworld::World;
use std::path::PathBuf;
use tokio::fs;

mod color;
mod mapblock;
mod render;
use render::compute_terrain;
mod config;
use config::Config;
mod terrain;

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

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    let config = fs::read_to_string(&args.config)
        .await
        .log_expect("reading config");
    let config: Config = toml::from_str(&config).log_expect("parsing config");
    let world = World::open(args.world);
    let map = world.get_map_data().await.log_expect("opening world data");
    let terrain_map = compute_terrain(map, &config)
        .await
        .log_expect("generating terrain map");
    let picture = terrain_map.render(&config).log_expect("rendering map");
    log::info!("Saving image");
    picture.save(&args.output).log_expect("saving image");
}
