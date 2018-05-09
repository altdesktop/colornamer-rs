extern crate colornamer;

use colornamer::Colors;

fn main() {
    let result = colornamer::name_hex_color("#1E90FF", Colors::Basic).unwrap();
    println!("result = {}", result);
}
