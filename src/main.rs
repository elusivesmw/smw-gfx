use std::fs;

fn main() {
    println!("Reading bin file...");

    let path = "./in/GFX00.bin";
    let bin = fs::read(path).unwrap();
    //print_first_bits(&bin);

    let converted = bin_to_4bpp(&bin);
    print_4bpp(&converted);
}

const SIZE: usize = 32; // 4bpp = 32 bytes per 8x8
fn bin_to_4bpp(bin: &Vec<u8>) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    for (i, _val) in bin.iter().enumerate().step_by(SIZE) {
        let slice = &bin[i..i+SIZE];
        let tile = slice_to_tile(&slice);

        //print_tile(&tile);
        tiles.push(tile);
    }
    //let test = tiles[0].get(0,0);
    //tiles[0].set(0, 0, test + 1);
    tiles
}

struct Tile {
    //format: 
    pixels: Vec<u8>,
}

impl Tile {
    fn new() -> Self {
        Self {
            pixels: Vec::new()
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * 8 + x]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.pixels[y * 8 + x] = val;
    }
}

fn slice_to_tile(slice: &[u8]) -> Tile {
    let mut tile = Tile::new();

    for r in 0..8 {
        let r = r * 2;
        //println!("{:?}", r);
        let bp1 = slice[r + 0];
        let bp2 = slice[r + 1];
        let bp3 = slice[r + 16];
        let bp4 = slice[r + 17];

        // translate to an array of colors for this row
        for c in (0..8).rev() {
            let palette = get_pixel_color(c, bp1, bp2, bp3, bp4);
            tile.pixels.push(palette);
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
            for tx in 0..16 {
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

