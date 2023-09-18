use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("reading bin file...");
    let path = config.file;
    let format = config.format;
    let bin = fs::read(path)?;
    //print_first_bits(&bin);

    let converted = bin_to_tiles(&bin, format.clone());
    print_4bpp(&converted);

    Ok(())
}


pub struct Config {
    file: String,
    format: Bpp,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("expected file and format arguments");
        }
        let file = args[1].clone();
        let format = Bpp::new(args[2].clone())?;

        Ok(Config { file, format })
    }
}


fn bin_to_tiles(bin: &Vec<u8>, format: Bpp) -> Vec<Tile> {
    let bytes_per_8x8 = format.bytes_per_8x8();
    //println!("{}", bytes_per_8x8);
    let mut tiles: Vec<Tile> = Vec::new();
    for (i, _val) in bin.iter().enumerate().step_by(bytes_per_8x8) {
        if i+bytes_per_8x8 > bin.iter().count() { println!("warn: unexpected file length"); break; }
        let slice = &bin[i..i+bytes_per_8x8];
        let tile = slice_to_tile(&slice, format);

        //print_tile(&tile);
        tiles.push(tile);
    }
    //let test = tiles[0].get(0,0);
    //tiles[0].set(0, 0, test + 1);
    tiles
}


#[derive(Copy,Clone)]
enum Bpp {
    _1bpp = 1,
    _2bpp = 2,
    _3bpp = 3,
    _4bpp = 4,
}

impl Bpp {
    fn new(format: String) -> Result<Bpp, &'static str> {
        let format: u8 = format.parse().unwrap_or_default();
        match format {
            1 => { Ok(Bpp::_1bpp) }
            2 => { Ok(Bpp::_2bpp) }
            3 => { Ok(Bpp::_3bpp) }
            4 => { Ok(Bpp::_4bpp) }
            _ => { return Err("Unsupported bpp format"); }
        }
    }

    fn val (&self) -> u8 {
        match self {
            Bpp::_1bpp => { Bpp::_1bpp as u8 }
            Bpp::_2bpp => { Bpp::_2bpp as u8 }
            Bpp::_3bpp => { Bpp::_3bpp as u8 }
            Bpp::_4bpp => { Bpp::_4bpp as u8 }
        }
    }

    fn bytes_per_8x8(&self) -> usize {
        let val = self.val() as usize;
        val * 8 as usize
    }
}

struct Tile {
    bpp: Bpp,
    data: Vec<u8>,
}

impl Tile {
    fn new(bpp: Bpp) -> Self {
        Self {
            bpp,
            data: Vec::new(),
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * 8 + x]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.data[y * 8 + x] = val;
    }
}

fn slice_to_tile(slice: &[u8], bpp: Bpp) -> Tile {
    let mut tile = Tile::new(bpp);
    let mut bp1 = 0;
    let mut bp2 = 0;
    let mut bp3 = 0;
    let mut bp4 = 0;

    for r in 0..8 {
        match bpp {
            Bpp::_1bpp => {
                bp1 = slice[r];
            }
            Bpp::_2bpp => {
                let r = r * 2;
                bp1 = slice[r + 0];
                bp2 = slice[r + 1];
            }
            Bpp::_3bpp => {
                bp1 = slice[r*2 + 0];
                bp2 = slice[r*2 + 1];
                bp3 = slice[16 + r];
            }
            Bpp::_4bpp => {
                let r = r * 2;
                //println!("{:?}", r);
                bp1 = slice[r + 0];
                bp2 = slice[r + 1];
                bp3 = slice[r + 16];
                bp4 = slice[r + 17];
            }
        }
        // translate to an array of colors for this row
        for c in (0..8).rev() {
            let palette = get_pixel_color(c, bp1, bp2, bp3, bp4);
            tile.data.push(palette);
        }
    }

    tile
}

fn get_pixel_color (c: u8, bp1: u8, bp2: u8 , bp3: u8, bp4: u8) -> u8 {
    let mask = 1 << c;

    let px_bp1 = if (bp1 & mask) == mask { 1 } else { 0 };
    let px_bp2 = if (bp2 & mask) == mask { 2 } else { 0 };
    let px_bp3 = if (bp3 & mask) == mask { 4 } else { 0 };
    let px_bp4 = if (bp4 & mask) == mask { 8 } else { 0 };

    let color = px_bp4 + px_bp3 + px_bp2 + px_bp1;
    //println!("bp4,3,2,1: {} + {} + {} + {} = {}", px_bp4, px_bp3, px_bp2, px_bp1, color);

    color
}


fn print_first_bits(bin: &Vec<u8>) {
    let first_bits = bin[..16].iter()
        .map(|b| format!("{:08b}", b).to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", first_bits);
}

fn print_tile(tile: &Vec<u8>) {
    for (i, px) in tile.iter().enumerate() {
        if i % 8 == 0 { println!(); }
        print!("{:x?}", px);
    }
    println!();
}


fn print_4bpp(converted: &Vec<Tile>) {
    let tiles_per_row = 16;
    let num_rows = converted.iter().len() / tiles_per_row;
    for row in 0..num_rows {
        let row_start = row * tiles_per_row;
        let row_end = row_start + tiles_per_row;
        let ry = &converted[row_start..row_end];
        for py in 0..8 {
            for tx in 0..tiles_per_row {
                let tile = &ry[tx];
                for px in 0..8 {
                    let p = &tile.get(px, py);
                    print!("{:x?}{:x?}", p, p);
                }
                print!(" ");
            }
            println!();
        }
    }
}

