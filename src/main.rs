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
fn bin_to_4bpp(bin: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    for (i, _val) in bin.iter().enumerate().step_by(SIZE) {
        let slice = &bin[i..i+SIZE];
        let tile = slice_to_tile(&slice);

        //print_tile(&tile);
        tiles.push(tile);
    }
    tiles
}

fn slice_to_tile(slice: &[u8]) -> Vec<u8> {
    let mut tile = Vec::new();

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
            tile.push(palette);
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


fn print_4bpp(converted: &Vec<Vec<u8>>) {
    let num_rows = converted.iter().len() / 16; // tiles per row
    for tri in 0..num_rows {
        let tr = &converted[tri*16..tri*16 + 16]; // tiles per row
        for pri in 0..8 {
            for ti in 0..16 {
                let tile = &tr[ti];
                for pi in 0..8 {
                    let p = &tile[pri * 8 + pi];
                    print!("{}", p);
                }
                print!(" ");
            }
            println!();
        }
    }
}

/*
fn u8_to_u4s (byte: u8) -> (u8, u8) {
    let msbs = (byte & 0xf0) >> 4;
    let lsbs = byte & 0x0f;
    //println!("in {} = {},{}", byte, msbs, lsbs);
    (msbs, lsbs)
}
*/
