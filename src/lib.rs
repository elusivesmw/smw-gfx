use log::{trace, warn};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

mod bpp;
mod tile;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.file);
    let format = config.format;
    let scale = 4;
    let print = false;

    let paths = collect_paths(path)?;
    for path in paths {
        run_file(path.as_path(), format, scale, print)?;
    }

    Ok(())
}

fn collect_paths(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if path.is_file() {
        return Ok(if path_is_valid(path) {
            vec![path.to_path_buf()]
        } else {
            vec![]
        });
    }

    let mut paths = Vec::new();
    if path.is_dir() {
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
    let ext = match path.extension() {
        Some(ext) => ext,
        None => {
            trace!("File {path:?} has no extension. Skipping.");
            return false;
        }
    };

    if ext != "bin" {
        trace!("File {path:?} is not a bin. Skipping.");
        return false;
    }

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => {
            trace!("File {path:?}, cannot get metadata. Skipping.");
            return false;
        }
    };

    let len = metadata.len();
    if len > MAX_FILE_SIZE {
        warn!("File {path:?}, of size {len}B, is too large (max of {MAX_FILE_SIZE}B). Skipping.");
        return false;
    }

    return true;
}

const TILES_PER_ROW: usize = 16;
const PX_WIDTH: usize = 1;
const PRINT_SPACE: bool = false;

fn run_file(
    file_path: &Path,
    format: bpp::Bpp,
    scale: u32,
    print: bool,
) -> Result<(), Box<dyn Error>> {
    let bin = fs::read(file_path)?;
    let tiles = tile::bin_to_tiles(&bin, format.clone());

    if print {
        tile::print_tiles(&tiles, TILES_PER_ROW, PX_WIDTH, PRINT_SPACE);
    }

    let filename = match file_path.file_stem() {
        Some(f) => f,
        None => {
            println!("Could not get filename of path {file_path:?}");
            return Ok(());
        }
    };

    // consider keeping the String path as a &Path or PathBuf
    let pwd = file_path.parent().unwrap_or(Path::new(""));
    let out_path = format!(
        "{}/{}.png",
        pwd.to_string_lossy(),
        filename.to_string_lossy()
    );

    tile::write_to_file(&tiles, out_path, scale);

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
        let in_dir = "tests/in";
        let out_dir = "tests/out";

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
