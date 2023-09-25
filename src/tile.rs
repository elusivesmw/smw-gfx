pub type Tile = Vec<u8>;

pub trait TileExt {
    fn get(&self, x: usize, y: usize) -> u8;
    fn set(&mut self, x: usize, y: usize, val: u8);
}

impl TileExt for Tile {
    fn get(&self, x: usize, y: usize) -> u8 {
        self[y * 8 + x]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self[y * 8 + x] = val;
    }
}

#[derive(Debug,Copy,Clone)]
pub enum Bpp {
    _1bpp = 1,
    _2bpp = 2,
    _3bpp = 3,
    _4bpp = 4,
}

impl Bpp {
    pub fn new(format: String) -> Result<Bpp, &'static str> {
        let format: u8 = format.parse().unwrap_or_default();
        match format {
            1 => { Ok(Bpp::_1bpp) }
            2 => { Ok(Bpp::_2bpp) }
            3 => { Ok(Bpp::_3bpp) }
            4 => { Ok(Bpp::_4bpp) }
            _ => { Err("Unsupported bpp format") }
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
        self.val() as usize * 8
    }
}

pub fn bin_to_tiles(bin: &Vec<u8>, format: Bpp) -> Vec<Tile> {
    let size = format.bytes_per_8x8();
    let mut tiles: Vec<Tile> = Vec::new();
    for chunk in bin.chunks(size) {
        if chunk.len() < size { println!("Warning: Unexpected file length"); break; }
        let tile = chunk_to_tile(&chunk, format);

        //print_tile(&tile);
        tiles.push(tile);
    }
    //let test = tiles[0].get(0,0);
    //tiles[0].set(0, 0, test + 1);
    tiles
}

pub fn chunk_to_tile(chunk: &[u8], bpp: Bpp) -> Tile {
    let mut tile = Tile::new();
    let mut bp1;
    let mut bp2 = 0;
    let mut bp3 = 0;
    let mut bp4 = 0;

    for r in 0..8 {
        match bpp {
            Bpp::_1bpp => {
                bp1 = chunk[r];
            }
            Bpp::_2bpp => {
                let r = r * 2;
                bp1 = chunk[r + 0];
                bp2 = chunk[r + 1];
            }
            Bpp::_3bpp => {
                bp1 = chunk[r*2 + 0];
                bp2 = chunk[r*2 + 1];
                bp3 = chunk[16 + r];
            }
            Bpp::_4bpp => {
                let r = r * 2;
                //println!("{:?}", r);
                bp1 = chunk[r + 0];
                bp2 = chunk[r + 1];
                bp3 = chunk[r + 16];
                bp4 = chunk[r + 17];
            }
        }
        // translate to an array of palettes for this row
        for c in (0..8).rev() {
            let palette = get_pixel_palette(c, bp1, bp2, bp3, bp4);
            tile.push(palette);
        }
    }

    tile
}

/// Gets a palette for a pixel at in column `c` with bitplanes 1-4
fn get_pixel_palette (c: u8, bp1: u8, bp2: u8 , bp3: u8, bp4: u8) -> u8 {
    let mask = 1 << c;

    let px_bp1 = if (bp1 & mask) == mask { 1 } else { 0 };
    let px_bp2 = if (bp2 & mask) == mask { 2 } else { 0 };
    let px_bp3 = if (bp3 & mask) == mask { 4 } else { 0 };
    let px_bp4 = if (bp4 & mask) == mask { 8 } else { 0 };

    let palette = px_bp4 + px_bp3 + px_bp2 + px_bp1;

    palette
}

pub fn print_tiles(tiles: &Vec<Tile>) {
    let tiles_per_row = 16;
    let num_rows = tiles.iter().len() / tiles_per_row;
    for row in 0..num_rows {
        let row_start = row * tiles_per_row;
        let row_end = row_start + tiles_per_row;
        let ry = &tiles[row_start..row_end];
        for py in 0..8 {
            for tx in 0..tiles_per_row {
                let tile = &ry[tx];
                for px in 0..8 {
                    let p = &tile.get(px, py);
                    palette_to_color(p);
                }
                print!(" ");
            }
            println!();
        }
    }
}

fn palette_to_color(p: &u8) {
    // ANSI colors
    // 0 blue (for transparency)
    // 1 white
    // 2->7 black->lightgrey
    match p {
        0 => { print!("\x1b[48;5;{}m{}{}", 18, p, p); }
        1 => { print!("\x1b[48;5;{}m\x1b[38;5;{}m{}{}", 255, 232, p, p); }
        2 => { print!("\x1b[48;5;{}m{}{}", 232, p, p); }
        3 => { print!("\x1b[48;5;{}m{}{}", 243, p, p); }
        4 => { print!("\x1b[48;5;{}m{}{}", 246, p, p); }
        5 => { print!("\x1b[48;5;{}m{}{}", 243, p, p); }
        6 => { print!("\x1b[48;5;{}m{}{}", 249, p, p); }
        7 => { print!("\x1b[48;5;{}m{}{}", 252, p, p); }
        _ => { print!("{}{}", p, p); }
    }
    // reset colors
    print!("\x1b[0m");
}

