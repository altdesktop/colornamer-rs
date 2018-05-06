#[macro_use] extern crate lazy_static;
extern crate vpsearch;

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

pub enum Colors {
    Basic,
    HTML,
    Ntc,
    Pantone,
    Roygbiv,
    X11
}

pub fn name_color_hex(hex: &str, colors: Colors) -> String {
    let names = match colors {
        Colors::Basic => BasicColors{}.get_colors(),
        Colors::HTML => HTMLColors{}.get_colors(),
        Colors::Ntc => NtcColors{}.get_colors(),
        Colors::Pantone => PantoneColors{}.get_colors(),
        Colors::Roygbiv => RoygbivColors{}.get_colors(),
        Colors::X11 => X11Colors{}.get_colors()
    };

    let vp = vpsearch::Tree::new(&names);
    let (index, _) = vp.find_nearest(&color::color_from_hex("", hex));

    String::from(names[index].name)
}
