use crate::bpp::Bpp;
use image::{self, Rgba, RgbaImage};

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

pub type Tiles = Vec<Tile>;
pub trait TilesExt {
    fn to_file(&self, format: Bpp) -> Vec<u8>;
}

impl TilesExt for Tiles {
    fn to_file(&self, format: Bpp) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for tile in self {
            let tile_file_bytes = tile_to_file_format(tile, format);
            //println!("{:02X?}", tile_file_bytes);
            bytes.extend(tile_file_bytes);
        }

        bytes
    }
}

fn tile_to_file_format(tile: &Tile, format: Bpp) -> Vec<u8> {
    let bytes_per_8x8 = format.bytes_per_8x8(); // 32
    let mut tile_file_bytes = vec![0u8; bytes_per_8x8];
    let pixels_per_tile_row = 8;

    for (r, row) in tile.chunks(pixels_per_tile_row).enumerate() {
        let mut row_bps = (0, 0, 0, 0);
        //println!("{:?}", row);

        for (c, px) in row.iter().rev().enumerate() {
            let px_bps = get_pixel_bitplanes(px, c, format);
            row_bps.0 |= px_bps.0;
            row_bps.1 |= px_bps.1;
            row_bps.2 |= px_bps.2;
            row_bps.3 |= px_bps.3;
        }
        //println!("{:?}", (row_bps.0, row_bps.1, row_bps.2, row_bps.3));

        match format {
            Bpp::_1bpp => {
                tile_file_bytes[r] = row_bps.0;
            }
            Bpp::_2bpp => {
                tile_file_bytes[r * 2 + 0] = row_bps.0;
                tile_file_bytes[r * 2 + 1] = row_bps.1;
            }
            Bpp::_3bpp => {
                tile_file_bytes[r * 2 + 0] = row_bps.0;
                tile_file_bytes[r * 2 + 1] = row_bps.1;
                tile_file_bytes[16 + r] = row_bps.2;
            }
            Bpp::_4bpp => {
                tile_file_bytes[r * 2 + 0] = row_bps.0;
                tile_file_bytes[r * 2 + 1] = row_bps.1;
                tile_file_bytes[r * 2 + 16] = row_bps.2;
                tile_file_bytes[r * 2 + 17] = row_bps.3;
            }
        }
    }

    tile_file_bytes
}

fn get_pixel_bitplanes(px: &u8, c: usize, format: Bpp) -> (u8, u8, u8, u8) {
    // px value should be less than the bpp format max value
    let max_bpp_val = 1 << format.val();
    assert!(px < &max_bpp_val, "px: {}, max: {}", px, &max_bpp_val);

    let mut px_bp1 = 0;
    let mut px_bp2 = 0;
    let mut px_bp3 = 0;
    let mut px_bp4 = 0;

    let mask = 1 << c;
    //println!("c: {:?}, mask: {:?}, {:?}", c, mask, px);
    px_bp1 |= if px & 1 == 1 { mask } else { 0 };
    px_bp2 |= if px & 2 == 2 { mask } else { 0 };
    px_bp3 |= if px & 4 == 4 { mask } else { 0 };
    px_bp4 |= if px & 8 == 8 { mask } else { 0 };

    (px_bp1, px_bp2, px_bp3, px_bp4)
}

pub fn bin_to_tiles(bin: &Vec<u8>, format: Bpp) -> Vec<Tile> {
    let size = format.bytes_per_8x8();
    let mut tiles: Vec<Tile> = Vec::new();
    for chunk in bin.chunks(size) {
        if chunk.len() < size {
            println!("Warning: Unexpected file length.");
            //break; // NOTE: handled in safe_chunk_index() now
        }
        let tile = chunk_to_tile(&chunk, format);

        //print_tile(&tile);
        tiles.push(tile);
        //println!("{:02X?}", chunk);
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
                bp1 = safe_chunk_index(chunk, r);
            }
            Bpp::_2bpp => {
                let r = r * 2;
                bp1 = safe_chunk_index(chunk, r + 0);
                bp2 = safe_chunk_index(chunk, r + 1);
            }
            Bpp::_3bpp => {
                bp1 = safe_chunk_index(chunk, r * 2 + 0);
                bp2 = safe_chunk_index(chunk, r * 2 + 1);
                bp3 = safe_chunk_index(chunk, 16 + r);
            }
            Bpp::_4bpp => {
                let r = r * 2;
                bp1 = safe_chunk_index(chunk, r + 0);
                bp2 = safe_chunk_index(chunk, r + 1);
                bp3 = safe_chunk_index(chunk, r + 16);
                bp4 = safe_chunk_index(chunk, r + 17);
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

fn safe_chunk_index(chunk: &[u8], index: usize) -> u8 {
    if index >= chunk.len() {
        return 0;
    }
    return chunk[index];
}

// Gets a palette for a pixel at in column `c` with bitplanes 1-4
fn get_pixel_palette(c: u8, bp1: u8, bp2: u8, bp3: u8, bp4: u8) -> u8 {
    let mask = 1 << c;

    let px_bp1 = if (bp1 & mask) == mask { 1 } else { 0 };
    let px_bp2 = if (bp2 & mask) == mask { 2 } else { 0 };
    let px_bp3 = if (bp3 & mask) == mask { 4 } else { 0 };
    let px_bp4 = if (bp4 & mask) == mask { 8 } else { 0 };

    let palette = px_bp4 + px_bp3 + px_bp2 + px_bp1;

    palette
}

pub fn print_tiles(tiles: &Vec<Tile>, tiles_per_row: usize) {
    for row in tiles.chunks(tiles_per_row) {
        for py in 0..8 {
            for tile in row.iter() {
                for px in 0..8 {
                    let p = &tile.get(px, py);
                    print_palette_to_ansi(p);
                }
                print!(" ");
            }
            println!();
        }
    }
}

const TILE_LENGTH: u32 = 8;
const TILES_PER_ROW: u32 = 16;

pub fn write_to_file(tiles: &Vec<Tile>, file_out: String, scale: u32) {
    let pixels_per_row: u32 = TILE_LENGTH * TILES_PER_ROW * scale;

    println!("Writing image to {file_out}...");
    let num_tiles = tiles.len() as u32;
    let mut height: u32 = (num_tiles / TILES_PER_ROW) * TILE_LENGTH * scale;

    if num_tiles % TILES_PER_ROW > 0 {
        height += TILE_LENGTH * scale;
    }

    let mut image = RgbaImage::new(pixels_per_row, height);
    //println!("Image details: {}x{}", image.width(), image.height());

    // fill in image pixels by tile
    for (i, tile) in tiles.iter().enumerate() {
        // get tile coordinates in output image
        let tile_y = (i as u32 / TILES_PER_ROW) * TILE_LENGTH * scale;
        let tile_x = (i as u32 % TILES_PER_ROW) * TILE_LENGTH * scale;
        //println!("{}, {}", tile_y, tile_x);

        // get pixels coordinates in output image
        for (j, pixel) in tile.iter().enumerate() {
            let pixel_y = tile_y + j as u32 / TILE_LENGTH * scale;
            let pixel_x = tile_x + j as u32 % TILE_LENGTH * scale;

            let pixel_color = palette_to_rgb(*pixel);
            put_pixel_at_scale(&mut image, pixel_x, pixel_y, pixel_color, scale);
        }
    }

    image.save(file_out).expect("Failed to save image");
}

fn put_pixel_at_scale(image: &mut RgbaImage, x: u32, y: u32, color: Rgba<u8>, scale: u32) {
    // account for scale
    for i in 0..scale {
        for j in 0..scale {
            image.put_pixel(x + j, y + i, color);
        }
    }
}

fn palette_to_rgb(p: u8) -> Rgba<u8> {
    match p {
        // first 8
        0x0 => {
            return Rgba([0, 0, 0, 0]);
        }
        0x1 => {
            return Rgba([255, 255, 255, 255]);
        }
        0x2 => {
            return Rgba([0, 0, 0, 255]);
        }
        0x3 => {
            return Rgba([42, 42, 42, 255]);
        }
        0x4 => {
            return Rgba([85, 85, 85, 255]);
        }
        0x5 => {
            return Rgba([127, 127, 127, 255]);
        }
        0x6 => {
            return Rgba([170, 170, 170, 255]);
        }
        0x7 => {
            return Rgba([212, 212, 212, 255]);
        }

        // second 8
        0x8 => {
            return Rgba([0, 0, 0, 0]);
        }
        0x9 => {
            return Rgba([255, 255, 255, 255]);
        }
        0xa => {
            return Rgba([0, 0, 0, 255]);
        }
        0xb => {
            return Rgba([42, 42, 42, 255]);
        }
        0xc => {
            return Rgba([85, 85, 85, 255]);
        }
        0xd => {
            return Rgba([127, 127, 127, 255]);
        }
        0xe => {
            return Rgba([170, 170, 170, 255]);
        }
        0xf => {
            return Rgba([212, 212, 212, 255]);
        }

        _ => {
            return Rgba([255, 0, 0, 255]);
        }
    }
}

fn print_palette_to_ansi(p: &u8) {
    // ANSI colors
    // 0 blue (for transparency)
    // 1 white
    // 2->7 black->lightgrey
    match p {
        0 => {
            print!("\x1b[48;5;{}m{}{}", 18, p, p);
        }
        1 => {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m{}{}", 255, 232, p, p);
        }
        2 => {
            print!("\x1b[48;5;{}m{}{}", 232, p, p);
        }
        3 => {
            print!("\x1b[48;5;{}m{}{}", 243, p, p);
        }
        4 => {
            print!("\x1b[48;5;{}m{}{}", 246, p, p);
        }
        5 => {
            print!("\x1b[48;5;{}m{}{}", 243, p, p);
        }
        6 => {
            print!("\x1b[48;5;{}m{}{}", 249, p, p);
        }
        7 => {
            print!("\x1b[48;5;{}m{}{}", 252, p, p);
        }
        _ => {
            print!("{:x}{:x}", p, p);
        }
    }
    // reset colors
    print!("\x1b[0m");
}
