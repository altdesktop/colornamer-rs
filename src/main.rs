extern crate colornamer;

use colornamer::Colors;

fn main() {
    let result = colornamer::name_color_hex("#1E90FF", Colors::Basic);
    println!("result = {}", result);
}
