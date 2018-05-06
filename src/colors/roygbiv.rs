use color_names::ColorNames;
use color::{Color, color_from_hex};

pub struct RoygbivColors {
}

impl ColorNames for RoygbivColors {
    fn get_colors(&self) -> Vec<Color> {
        return vec![
            color_from_hex("red", "#FF0000"),
            color_from_hex("orange", "#FFA500"),
            color_from_hex("yellow", "#FFFF00"),
            color_from_hex("green", "#008000"),
            color_from_hex("blue", "#0000FF"),
            color_from_hex("indigo", "#4B0082"),
            color_from_hex("violet", "#EE82EE")
        ]
    }
}
