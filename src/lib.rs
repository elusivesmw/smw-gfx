use std::fs;
use std::error::Error;
mod tile;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("reading bin file...");
    let path = config.file;
    let format = config.format;
    let bin = fs::read(path)?;

    let converted = tile::bin_to_tiles(&bin, format.clone());
    tile::print_tiles(&converted);

    Ok(())
}

pub struct Config {
    file: String,
    format: tile::Bpp,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("expected file and format arguments");
        }
        let file = args[1].clone();
        let format = tile::Bpp::new(args[2].clone())?;

        Ok(Config { file, format })
    }
}

