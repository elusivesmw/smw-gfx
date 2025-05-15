#[derive(Debug, Copy, Clone)]
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
            1 => Ok(Bpp::_1bpp),
            2 => Ok(Bpp::_2bpp),
            3 => Ok(Bpp::_3bpp),
            4 => Ok(Bpp::_4bpp),
            _ => Err("Unsupported bpp format"),
        }
    }

    pub fn val(&self) -> u8 {
        match self {
            Bpp::_1bpp => Bpp::_1bpp as u8,
            Bpp::_2bpp => Bpp::_2bpp as u8,
            Bpp::_3bpp => Bpp::_3bpp as u8,
            Bpp::_4bpp => Bpp::_4bpp as u8,
        }
    }

    pub fn bytes_per_8x8(&self) -> usize {
        self.val() as usize * 8
    }
}
