# colornamer-rs

Give me a color and I will name it.

## About

Naming things is one of the hard things in computer science.

Colornamer-rs is a library written in Rust that helps you come up with a good name for a color.

It does this by converting the color to [Lab color space](https://en.wikipedia.org/wiki/Lab_color_space) and using the Delta E formula to compare the color difference in a list of colors with known names and finding the nearest neighbor.

## Installing

Colornamer-rs is available as a [crate](https://crates.io/crates/colornamer) on crates.io. Add `colornamer` to your dependencies with the latest stable version to use the library.

```toml
[dependencies]
colornamer = "1.0.1"
```

## Example

```rust
extern crate colornamer;

use colornamer::{ColorNamer, Colors};

fn main() {
    let colornamer = ColorNamer::new(Colors::HTML);
    let name: String = colornamer.name_hex_color("#1E90FF").unwrap()

    println!("I will call this color '{}'", name);
}
```

## Lists

The color names are derived from several lists:

* roygbiv
* basic
* HTML
* X11
* Pantone
* ntc - an [astounding collection](http://chir.ag/projects/ntc/) of over 1500 named colors.

## Projects that use colornamer-rs

* [i3-style](https://github.com/acrisci/i3-style)

## Acknowledgements

This library is based on the wonderful [color-namer](https://github.com/colorjs/color-namer) library for nodejs.

Color difference formulas were found at [easyrgb.com](http://www.easyrgb.com/en/math.php).

Further reading:

* [Wikipedia article for color distance](https://en.wikipedia.org/wiki/Color_difference).

## License

MIT (see LICENSE)
