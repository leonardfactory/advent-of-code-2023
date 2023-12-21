use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const WHITE: Rgb = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
}

impl FromStr for Rgb {
    type Err = ();

    /**
     * Parse an RGB color code like #ff00ff or ff00ff into Rgb
     */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = if s.starts_with('#') { &s[1..] } else { s };
        let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| ())?;
        let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| ())?;
        let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| ())?;
        Ok(Rgb { r, g, b })
    }
}
