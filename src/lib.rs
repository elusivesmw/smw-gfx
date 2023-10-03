use std::fs;
use std::error::Error;
use std::io::Write;
mod tile;
use tile::TilesExt;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Reading bin file...");
    let path = config.file;
    let format = config.format;
    let bin = fs::read(path)?;

    let tiles = tile::bin_to_tiles(&bin, format.clone());
    tile::print_tiles(&tiles, 16);
    let contents = tiles.to_file(format);
    let mut f = fs::File::create("temp.bin")?;
    f.write_all(&contents)?;

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

