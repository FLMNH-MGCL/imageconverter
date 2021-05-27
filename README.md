# Image Converter

[![DOI](https://zenodo.org/badge/341654801.svg)](https://zenodo.org/badge/latestdoi/341654801)

Converts images from CR2 to JPG formats. Developed for the McGuire Center for Lepidoptera at the Florida Museum of Natural History.

Given starting and destination paths, the program will collect and convert .CR2 files to .JPG. [magick_rust](https://github.com/nlfiedler/magick-rust), a wrapper around [ImageMagick](https://imagemagick.org/index.php), will be attempted first, falling back on [imagepipe](https://github.com/pedrocr/imagepipe). Support for other file conversions is possible, but will not be implemented until the Museum's needs require it.

### Installation and Usage

Be sure to install [Rust](https://www.rust-lang.org/) and [ImageMagick](https://imagemagick.org/index.php) on your system. When installing ImageMagick on a Windows computer, please ensure you check the checkbox "Install development headers and libraries for C and C++".

You may run the program with cargo:

```bash
$ cargo run --release -- --help
$ cargo run --release -- [options] [flags]
```

Alternatively, you may build an executable and run that directly:

```bash
-- linux / macos --
$ cargo build --release
$ ./target/release/imageconverter [options] [flags]

-- windows --
$ cargo build --release
$ .\target\release\imageconverter.exe [options] [flags]
```

## Benchmarks

[magick_rust](https://github.com/nlfiedler/magick-rust) conversions average 40s for Hi-Resolution CR2 files, while [imagepipe](https://github.com/pedrocr/imagepipe) conversions average 90s for the same files. As such, the order is magick -> imagepipe.

[rayon](https://docs.rs/rayon/1.5.0/rayon/) is used to expedite the overall conversion times, offloading the conversions to separate threads in parallel execution.
