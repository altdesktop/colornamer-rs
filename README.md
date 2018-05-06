# colornamer-rs

Give me a color and I will name it.

*colornamer-rs is in the early stages of development and the api should be considered unstable*

## About

Naming things is one of the hard things in computer science. colornamer-rs is a library written in Rust that helps you come up with a good name for a color. It does this by converting the color to RGB space and then finding the nearest neighbor.

## Example

```rust
extern crate colornamer;

use colornamer::Colors;

fn main() {
    // dodgerblue
    let name = colornamer::name_color_hex("#1E90FF", Colors::HTML);
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

## TODO

* Use [Lab color space](https://en.wikipedia.org/wiki/Lab_color_space) instead of RGB.

## Acknowledgements

This library is based on the wonderful [color-namer](https://github.com/colorjs/color-namer) library for nodejs.

## License

MIT (see LICENSE)
