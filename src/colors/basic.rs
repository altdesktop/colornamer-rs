use color_names::ColorNames;
use color::{Color, color_from_triplet};

pub struct BasicColors {
}

impl ColorNames for BasicColors {
    fn get_colors(&self) -> Vec<Color> {
        return vec![
            color_from_triplet("black", (0, 0, 0)),
            color_from_triplet("blue", (0, 0, 255)),
            color_from_triplet("cyan", (0, 255, 255)),
            color_from_triplet("green", (0, 128, 0)),
            color_from_triplet("teal", (0, 128, 128)),
            color_from_triplet("turquoise", (64, 224, 208)),
            color_from_triplet("indigo", (75, 0, 130)),
            color_from_triplet("gray", (128, 128, 128)),
            color_from_triplet("purple", (128, 0, 128)),
            color_from_triplet("brown", (165, 42, 42)),
            color_from_triplet("tan", (210, 180, 140)),
            color_from_triplet("violet", (238, 130, 238)),
            color_from_triplet("beige", (245, 245, 220)),
            color_from_triplet("fuchsia", (255, 0, 255)),
            color_from_triplet("gold", (255, 215, 0)),
            color_from_triplet("magenta", (255, 0, 255)),
            color_from_triplet("orange", (255, 165, 0)),
            color_from_triplet("pink", (255, 192, 203)),
            color_from_triplet("red", (255, 0, 0)),
            color_from_triplet("white", (255, 255, 255)),
            color_from_triplet("yellow", (255, 255, 0))
        ]
    }
}
