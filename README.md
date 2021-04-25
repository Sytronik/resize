# resize [![Build Status](https://travis-ci.org/PistonDevelopers/resize.png?branch=master)](https://travis-ci.org/PistonDevelopers/resize) [![crates.io](https://img.shields.io/crates/v/resize.svg)](https://crates.rs/crates/resize)

Image resampling library in pure Rust.

## Features

* Fast, with support for many pixel formats
* No encoders/decoders, meant to be used with some external library
* Tuned for resizing to the same dimensions multiple times: uses preallocated buffers and matrixes

## Usage

```rust
use resize::Pixel::RGB8;
use resize::Type::Lanczos3;

// Downscale by 2x.
let (w1, h1) = (640, 480);
let (w2, h2) = (320, 240);
// Don't forget to fill `src` with image data (RGB8).
// This requires Vec<RGB<u8>>. If you have Vec<u8>, then use .as_rgb()/.as_rgb_mut() to reinterpret it as a slice of pixels.
let src = vec![RGB::new(0,0,0); w1*h1];
// Destination buffer. Must be mutable.
let mut dst = vec![RGB::new(0,0,0); w2*h2];
// Create reusable instance.
let mut resizer = resize::new(w1, h1, w2, h2, RGB8, Lanczos3);
// Do resize without heap allocations.
// Might be executed multiple times for different `src` or `dst`.
resizer.resize(&src, &mut dst);
```

See [API documentation](http://docs.piston.rs/resize/resize/) for overview of all available methods. See also [this example](https://github.com/PistonDevelopers/resize/blob/master/examples/resize.rs).

## Recommendations

Read [this](https://www.imagemagick.org/Usage/filter/) and [this](https://www.imagemagick.org/Usage/filter/nicolas/) great articles on image resizing technics and resampling filters. Tldr; (with built-in filters of this library) use `Lanczos3` for downscaling, use `Mitchell` for upscaling. You may also want to [downscale in linear colorspace](https://www.imagemagick.org/Usage/resize/#resize_colorspace) (but not upscale). Gamma correction routines currently not included to the library, but actually quite simple to accomplish manually, see [here](https://en.wikipedia.org/wiki/Gamma_correction) for some basic theory.

## License

* Library is licensed under [MIT](LICENSE)
* Image used in examples is licensed under [CC BY-SA 3.0](https://commons.wikimedia.org/wiki/File%3A08-2011._Panthera_tigris_tigris_-_Texas_Park_-_Lanzarote_-TP04.jpg)
