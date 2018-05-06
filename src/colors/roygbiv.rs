use color_names::ColorNames;
use color::{Color, color_from_triplet};

pub struct RoygbivColors {
}

impl ColorNames for RoygbivColors {
    fn get_colors(&self) -> Vec<Color> {
        return vec![
            color_from_triplet("red", (255, 0, 0)),
            color_from_triplet("orange", (255, 165, 0)),
            color_from_triplet("yellow", (255, 255, 0)),
            color_from_triplet("green", (0, 128, 0)),
            color_from_triplet("blue", (0, 0, 255)),
            color_from_triplet("indigo", (75, 0, 130)),
            color_from_triplet("violet", (238, 130, 238))
        ]
    }
}
