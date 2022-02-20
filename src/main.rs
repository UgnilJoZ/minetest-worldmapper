use clap::Parser;
use minetestworld::MapData;
use std::fs;
use std::path::PathBuf;

mod color;
mod mapblock;
use mapblock::render_map;
mod config;
use config::Config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    sqlitefile: PathBuf,

    /// config file
    #[clap(short, long)]
    config: PathBuf,

    /// The file in which to write the result
    #[clap(short, long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let config = fs::read_to_string(&args.config).unwrap();
    let config: Config = toml::from_str(&config).unwrap();
    println!("{config:?}");
    let map = MapData::from_sqlite_file(args.sqlitefile).unwrap();
    let picture = render_map(&map, &config).unwrap();
    picture.save(&args.output).unwrap();
}
