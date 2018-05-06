use std::vec::Vec;
use color::Color;

pub trait ColorNames {
    fn get_colors(&self) -> Vec<Color>;
}
