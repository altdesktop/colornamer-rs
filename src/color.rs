extern crate vpsearch;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub name: &'static str
}

lazy_static! {
    static ref HEX_ALPHABETS: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('0', 0);
        m.insert('1', 1);
        m.insert('2', 2);
        m.insert('3', 3);
        m.insert('4', 4);
        m.insert('5', 5);
        m.insert('6', 6);
        m.insert('7', 7);
        m.insert('8', 8);
        m.insert('9', 9);
        m.insert('A', 10);
        m.insert('B', 11);
        m.insert('C', 12);
        m.insert('D', 13);
        m.insert('E', 14);
        m.insert('F', 15);
        m
    };
}

#[derive(Debug)]
pub enum ColorError {
    InvalidColorError
}

pub fn color_from_hex(name: &'static str, hex: &str) -> Result<Color, ColorError> {
    let hex_vec: Vec<char> = if hex.starts_with("#") {
        hex.to_string().to_uppercase().chars().skip(1).collect()
    } else {
        hex.to_string().to_uppercase().chars().collect()
    };

    let mut value: Vec<u32> = vec![];

    if hex_vec.len() < 6 {
        return Err(ColorError::InvalidColorError);
    }

    // XXX use step_by when it is stable
    let mut i = 0;
    while i < 6 {
        if !HEX_ALPHABETS.contains_key(&hex_vec[i]) || !HEX_ALPHABETS.contains_key(&hex_vec[i+1]) {
            return Err(ColorError::InvalidColorError);
        }

        let int1 = HEX_ALPHABETS.get(&hex_vec[i]).unwrap();
        let int2 = HEX_ALPHABETS.get(&hex_vec[i+1]).unwrap();
        value.insert(i / 2, int1 * 16 + int2);

        i += 2;
    }

    Ok(Color {
        r: value[0] as f32,
        g: value[1] as f32,
        b: value[2] as f32,
        name: name
    })
}

pub fn color_from_triplet(name: &'static str, t: (u8, u8, u8)) -> Color {
    Color {
        r: t.0 as f32,
        g: t.1 as f32,
        b: t.2 as f32,
        name,
    }
}

impl vpsearch::MetricSpace for Color {
    type UserData = ();
    type Distance = f32;

    fn distance(&self, other: &Self, _: &Self::UserData) -> Self::Distance {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        (dr*dr + dg*dg + db*db).sqrt()
    }
}
