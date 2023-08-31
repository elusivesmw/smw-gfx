use std::fs;

fn main() {
    println!("Reading bin file...");

    let path = "./in/GFX00.bin";
    let bin = fs::read(path).unwrap();

    let first_bits = bin[..16].iter()
        .map(|b| format!("{:08b}", b).to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", first_bits);
    //println!("{:x?}", &contents[..16]);


    //let data = 
    bin_to_4bpp(bin);

}


const SIZE: usize = 32; // 4bpp = 32 bytes per 8x8
fn bin_to_4bpp(bin: Vec<u8>) -> () { // gfx_4bpp {
    //let mut tiles: Vec<Vec<u8>> = Vec::new();
    for (i, val) in bin.iter().enumerate().step_by(32) {
        let slice = &bin[i..i+SIZE];
        let tile = slice_to_tile(slice);
        print_tile(tile);

        //tiles.push(tile);
        //if i == 64 { break; } // just one 8x8 row for now
    }

}

fn slice_to_tile(slice: &[u8]) -> Vec<u8> {
    //println!("{:?}", slice);
    let mut tile = Vec::new();

    for r in 0..8 {//8 {
        let r = r * 2;
        //println!("{:?}", r);
        let bp1 = slice[r + 0];
        let bp2 = slice[r + 1];
        let bp3 = slice[r + 16];
        let bp4 = slice[r + 17];

        // translate to an array of colors for this row
        for c in (0..8).rev() {
            // c represents each column in a row

            let palette = get_pixel_color(c, bp1, bp2, bp3, bp4);
            //println!("pixel {:x?} = palette {}", c, palette);

            // palette byte to 2 pixels (for bpp4)
            let (px1, px2) = u8_to_u4s(c);
            tile.push(palette);
            //tile.push(px1);
            //tile.push(px2);
        }
    }

    //println!("{:?}", slice);
    //println!("{:?}", tile);
    tile
}

fn get_pixel_color (c: u8, bp1: u8, bp2: u8 , bp3: u8, bp4: u8) -> u8 {
    let palette: u8 = 0;
    let mask = 1 << c;

    let px_bp1 = if (bp1 & mask) == mask { 1 } else { 0 };
    let px_bp2 = if (bp2 & mask) == mask { 2 } else { 0 };
    let px_bp3 = if (bp3 & mask) == mask { 4 } else { 0 };
    let px_bp4 = if (bp4 & mask) == mask { 8 } else { 0 };

    let color = px_bp4 + px_bp3 + px_bp2 + px_bp1;
    //println!("bp4,3,2,1: {} + {} + {} + {} = {}", px_bp4, px_bp3, px_bp2, px_bp1, color);

    color
}

fn print_tile(tile: Vec<u8>) {
    for (i, px) in tile.iter().enumerate() {
        if i % 8 == 0 { println!(); }
        print!("{:x?}", px);
    }
    println!();
}

fn u8_to_u4s (byte: u8) -> (u8, u8) {
    let msbs = (byte & 0xf0) >> 4;
    let lsbs = byte & 0x0f;
    //println!("in {} = {},{}", byte, msbs, lsbs);
    (msbs, lsbs)
}

/*
fn u8_to_u2s (byte: u8) -> (u8, u8, u8, u8) {
    let msbs = (byte & 0x11000000) >> 6;
    let lsbs = byte & 0x0f;
    //println!("in {} = {},{}", byte, msbs, lsbs);
    (msbs, lsbs)
}
*/
