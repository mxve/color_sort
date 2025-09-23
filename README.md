# color_sort

[![Crates.io Version](https://img.shields.io/crates/v/color_sort?style=for-the-badge&logo=rust)](https://crates.io/crates/color_sort) ![CI](https://img.shields.io/github/actions/workflow/status/mxve/color_sort/ci.yml?style=for-the-badge&logo=github)

A Rust library for sorting and filtering colors in HEX, RGB(A) and HSL formats.
This library does not follow any conventions, it justworks™ but may do so differently from what you would expect.

## Features

- Parse colors from multiple formats
  - Hex
    - 3-digit: `#f00` RGB
    - 4-digit: `#f00a` RGBA
    - 6-digit: `#ff0000` full RGB
    - 8-digit: `#ff0000aa` full RGBA
  - RGB/RGBA: `rgb(255,0,0)`, `rgba(255,0,0,0.5)`
  - HSL: `hsl(0,100%,50%)`
- Sort colors by spectrum, luminance, or opacity
- Filter colors by hue, saturation, or transparency
- Convert between color formats

## Usage

```rust
use color_sort::*;

// Parse mixed color formats
let colors = parse_colors(&[
    "#f00".to_string(),
    "#ff0000".to_string(),
    "#ff0000aa".to_string(),
    "rgb(0, 255, 0)".to_string(),
    "rgba(255, 0, 0, 0.5)".to_string(),
    "hsl(240, 100%, 50%)".to_string(),
])?;

// Sort by spectrum (red -> yellow -> green -> cyan -> blue -> magenta)
let mut sorted = colors.clone();
sort_colors(&mut sorted);

// Convert to format
let hex_colors = convert_colors(&colors, TargetFormat::Hex);

// Filter by hue
let filter = FilterOptions::default().with_hues([HueFilter::Red]);
let red_colors = filter_colors(&colors, &filter);
```

## Examples

```bash
cat examples/colors.json | cargo run --example sort_hex_stdin
```
