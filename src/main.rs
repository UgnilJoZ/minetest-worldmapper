use async_std::fs;
use async_std::path::PathBuf;
use clap::Parser;
use minetestworld::World;

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

#[async_std::main]
async fn main() {
    let args = Args::parse();
    let config = fs::read_to_string(&args.config)
        .await
        .expect("reading config");
    let config: Config = toml::from_str(&config).expect("parsing config");
    let world = World::new(args.world);
    let map = world.get_map_data().await.expect("opening world data");
    let terrain_map = compute_terrain(map, &config).await.expect("rendering map");
    let picture = terrain_map.render(&config);
    eprintln!("Saving image");
    picture.save(&args.output).expect("saving image");
}
