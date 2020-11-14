#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

mod color;
mod color_names;
mod colors;

use color_names::ColorNames;
use colors::basic::BasicColors;
use colors::html::HTMLColors;
use colors::ntc::NtcColors;
use colors::pantone::PantoneColors;
use colors::roygbiv::RoygbivColors;
use colors::x11::X11Colors;
use color::{Color, ColorError};

bitflags! {
    pub struct Colors: u32 {
        const BASIC   = 0b00000001;
        const HTML    = 0b00000010;
        const NTC     = 0b00000100;
        const PANTONE = 0b00001000;
        const ROYGBIV = 0b00010000;
        const X11     = 0b00100000;
    }
}

pub struct ColorNamer {
    colors: Vec<Color>
}

impl ColorNamer {
    pub fn new(colors: Colors) -> ColorNamer {
        let mut color_vec: Vec<Color> = vec![];

        if !(colors | Colors::BASIC).is_empty() {
            color_vec.append(&mut BasicColors{}.get_colors());
        }

        if !(colors | Colors::HTML).is_empty() {
            color_vec.append(&mut HTMLColors{}.get_colors());
        }

        if !(colors | Colors::NTC).is_empty() {
            color_vec.append(&mut NtcColors{}.get_colors());
        }

        if !(colors | Colors::PANTONE).is_empty() {
            color_vec.append(&mut PantoneColors{}.get_colors());
        }

        if !(colors | Colors::ROYGBIV).is_empty() {
            color_vec.append(&mut RoygbivColors{}.get_colors());
        }

        if !(colors | Colors::X11).is_empty() {
            color_vec.append(&mut X11Colors{}.get_colors());
        }

        ColorNamer {
            colors: color_vec
        }
    }

    pub fn name_hex_color(&self, hex: &str) -> Result<String, ColorError> {
        let color = color::color_from_hex("", &hex)?;

        let mut min_distance: f32 = std::f32::MAX;
        let mut closest_color = color;

        for c in &self.colors {
            let distance = c.distance(color);
            if distance < min_distance {
                min_distance = distance;
                closest_color = *c;
            }
        }

        Ok(closest_color.name.to_string())
    }
}
