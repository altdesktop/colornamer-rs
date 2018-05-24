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

impl Color {
	/// http://www.easyrgb.com/en/math.php
	fn to_lab(&self) -> (f32, f32, f32) {
		let xyz_normalize = |c: f32| {
			let c_normal = c / 255.0;
			if c_normal > 0.04045 {
				((c_normal + 0.055) / 1.055).powf(2.4)
			} else {
				c_normal / 12.92
			}
		};

		let r = xyz_normalize(self.r);
		let g = xyz_normalize(self.g);
		let b = xyz_normalize(self.b);

		let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
		let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
		let z = r * 0.0193 + g * 0.1192 + b * 0.9505;

		let lab_normalize = |c: f32| {
			if c > 0.008856 {
				c.powf(1.0 / 3.0)
			} else {
				7.787 * c + 16.0 / 116.0
			}
		};

		let lx = lab_normalize(x);
		let ly = lab_normalize(y);
		let lz = lab_normalize(z);

		let l = 116.0 * ly - 16.0;
		let a = 500.0 * (lx - ly);
		let b = 200.0 * (ly - lz);

		(l, a, b)
	}

	pub fn distance(&self, other: &Self) -> f32 {
		let (sl, sa, sb) = self.to_lab();
		let (ol, oa, ob) = other.to_lab();

		let dl = sl - ol;
		let da = sa - oa;
		let db = sb - ob;

		(dl*dl + da*da + db*db).sqrt()
	}
}
