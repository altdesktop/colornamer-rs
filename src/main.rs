extern crate colornamer;

use colornamer::{ColorNamer, Colors};

fn main() {
    let colornamer = ColorNamer::new(Colors::BASIC);

    let result = colornamer.name_hex_color("#1E90FF").unwrap();

    println!("result = {}", result);
}
