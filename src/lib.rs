use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

mod bpp;
mod tile;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.file);
    let format = config.format;
    let scale = 4;

    let paths = collect_paths(path)?;
    for path in paths {
        run_file(path.as_path(), format, scale)?;
    }

    Ok(())
}

fn collect_paths(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut paths = Vec::new();
    if path.is_file() && path_is_valid(path) {
        paths.push(path.to_path_buf());
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if !path_is_valid(entry.path().as_path()) {
                continue;
            }
            paths.push(entry.path());
        }
    };

    Ok(paths)
}

const MAX_FILE_SIZE: u64 = 256 * 1024;
fn path_is_valid(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        if ext != "bin" {
            println!("File {path:?} is not a bin. Skipping.");
            return false;
        }
        if let Ok(metadata) = fs::metadata(path) {
            let len = metadata.len();
            if len > MAX_FILE_SIZE {
                println!("File {path:?} is too large {len}. Skipping.");
                return false;
            }
            return true;
        }
    }
    return false;
}

fn run_file(file_path: &Path, format: bpp::Bpp, scale: u32) -> Result<(), Box<dyn Error>> {
    let bin = fs::read(file_path)?;

    if let Some(filename) = file_path.file_stem() {
        let tiles = tile::bin_to_tiles(&bin, format.clone());
        //tile::print_tiles(&tiles, 8);
        tile::write_to_file(
            &tiles,
            format!("in/{}.png", filename.to_string_lossy()),
            scale,
        );
    };

    Ok(())
}

pub struct Config {
    file: String,
    format: bpp::Bpp,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("expected file and format arguments");
        }
        let file = args[1].clone();
        let format = bpp::Bpp::new(args[2].clone())?;

        Ok(Config { file, format })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tile::TilesExt;

    #[test]
    fn round_trip_1bpp() {
        round_trip(bpp::Bpp::_1bpp, "1bpp_test.bin");
    }

    #[test]
    fn round_trip_2bpp() {
        round_trip(bpp::Bpp::_2bpp, "2bpp_test.bin");
    }

    #[test]
    fn round_trip_3bpp() {
        round_trip(bpp::Bpp::_3bpp, "3bpp_test.bin");
    }

    #[test]
    fn round_trip_4bpp() {
        round_trip(bpp::Bpp::_4bpp, "4bpp_test.bin");
    }

    fn round_trip(format: bpp::Bpp, file_in: &str) {
        let in_dir = "in";
        let out_dir = "tests_out";

        fs::create_dir_all(out_dir).unwrap();
        let in_path: PathBuf = [in_dir, file_in].iter().collect();
        let out_path: PathBuf = [out_dir, file_in].iter().collect();

        let in_bin = fs::read(in_path).unwrap();
        let tiles = tile::bin_to_tiles(&in_bin, format.clone());
        let contents = tiles.to_file(format);

        let mut f = fs::File::create(&out_path).unwrap();
        f.write_all(&contents).unwrap();

        let out_bin = fs::read(&out_path).unwrap();
        //fs::remove_dir_all(out_dir).unwrap();

        assert_eq!(in_bin, out_bin);
    }
}
