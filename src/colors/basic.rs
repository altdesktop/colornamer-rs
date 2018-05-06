use color_names::ColorNames;
use color::{Color, color_from_hex};

pub struct BasicColors {
}

impl ColorNames for BasicColors {
    fn get_colors(&self) -> Vec<Color> {
        return vec![
            color_from_hex("black", "#000000"),
            color_from_hex("blue", "#0000FF"),
            color_from_hex("cyan", "#00FFFF"),
            color_from_hex("green", "#008000"),
            color_from_hex("teal", "#008080"),
            color_from_hex("turquoise", "#40E0D0"),
            color_from_hex("indigo", "#4B0082"),
            color_from_hex("gray", "#808080"),
            color_from_hex("purple", "#800080"),
            color_from_hex("brown", "#A52A2A"),
            color_from_hex("tan", "#D2B48C"),
            color_from_hex("violet", "#EE82EE"),
            color_from_hex("beige", "#F5F5DC"),
            color_from_hex("fuchsia", "#FF00FF"),
            color_from_hex("gold", "#FFD700"),
            color_from_hex("magenta", "#FF00FF"),
            color_from_hex("orange", "#FFA500"),
            color_from_hex("pink", "#FFC0CB"),
            color_from_hex("red", "#FF0000"),
            color_from_hex("white", "#FFFFFF"),
            color_from_hex("yellow", "#FFFF00")
        ]
    }
}
