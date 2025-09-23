use color_sort::{Color, parse_colors, sort_colors};
use serde_json;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let hex_colors: Vec<String> = serde_json::from_str(&input)?;

    let mut colors = parse_colors(&hex_colors)?;
    sort_colors(&mut colors);

    println!("Sorted colors:");
    for color in &colors {
        print_colored_hex(&color);
    }

    Ok(())
}

fn print_colored_hex(color: &Color) {
    let (r, g, b, _) = color.to_rgba();

    let ansi_code = format!("\x1b[38;2;{};{};{}m", r, g, b);
    print!("{}  ████  \x1b[0m", ansi_code);
    println!(" {}", color.original_input);
}
