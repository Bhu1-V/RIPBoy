pub fn bit_set(mut i: u8, b: u8) -> u8 {
    i = match b {
        0 => i | 0b_0000_0001,
        1 => i | 0b_0000_0010,
        2 => i | 0b_0000_0100,
        3 => i | 0b_0000_1000,
        4 => i | 0b_0001_0000,
        5 => i | 0b_0010_0000,
        6 => i | 0b_0100_0000,
        7 => i | 0b_1000_0000,
        _ => panic!("unhandled bit_set fun"),
    };
    i
}

pub fn bit_reset(mut i: u8, b: u8) -> u8 {
    i = match b {
        0 => i & 0b_1111_1110,
        1 => i & 0b_1111_1101,
        2 => i & 0b_1111_1011,
        3 => i & 0b_1111_0111,
        4 => i & 0b_1110_1111,
        5 => i & 0b_1101_1111,
        6 => i & 0b_1011_1111,
        7 => i & 0b_0111_1111,
        _ => panic!("unhandled bit_reset fun"),
    };
    i
}

pub fn test_bit(i: u8, b: u8) -> bool {
    match b {
        0 => {
            if (i & 1) == 1 {
                true
            } else {
                false
            }
        }
        1 => {
            if ((i >> 1) & 1) == 1 {
                true
            } else {
                false
            }
        }
        2 => {
            if ((i >> 2) & 1) == 1 {
                true
            } else {
                false
            }
        }
        3 => {
            if ((i >> 3) & 1) == 1 {
                true
            } else {
                false
            }
        }
        4 => {
            if ((i >> 4) & 1) == 1 {
                true
            } else {
                false
            }
        }
        5 => {
            if ((i >> 5) & 1) == 1 {
                true
            } else {
                false
            }
        }
        6 => {
            if ((i >> 6) & 1) == 1 {
                true
            } else {
                false
            }
        }
        7 => {
            if ((i >> 7) & 1) == 1 {
                true
            } else {
                false
            }
        }
        _ => panic!("Unhandled test_bit case"),
    }
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
