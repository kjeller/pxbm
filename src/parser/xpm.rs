use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::{Regex, RegexBuilder};
use std::{collections::HashMap, str::FromStr};

use crate::color::Color;

use super::Parser;

pub enum XpmFormat {
    Xpm1,
    Xpm2,
    Xpm3,
}

// TODO: support hotspot and XPMEXT
pub struct Xpm {
    format: XpmFormat,
    width: usize,
    _height: usize,
    _num_colors: usize,
    chars_per_pixel: usize,
    color_mapping: HashMap<String, Color>,
    data: Vec<char>,
}

impl Xpm {
    pub fn parse(input: &[u8]) -> Result<Xpm> {
        let input = std::str::from_utf8(input)?;
        let header = input.lines().next().unwrap();
        match header {
            "! XPM2" => Self::parse_xpm2(input),
            "/* XPM */" => Self::parse_xpm3(input),
            _ => Self::parse_xpm1(input),
        }
    }

    fn space_quote_stripper(s: &str) -> Result<String> {
        Ok(s.split('"')
            .nth(1)
            .ok_or_else(|| anyhow!("Unmatched double quote in file"))?
            .to_owned())
    }

    fn parse_xpm1(input: &str) -> Result<Xpm> {
        let suffixed_define =
            |suffix: &str| Regex::new(&format!(r"#define\s+\w+_{suffix}\s+(\d+)")).unwrap();
        let re_format = suffixed_define("format");
        let re_width = suffixed_define("width");
        let re_height = suffixed_define("height");
        let re_ncolors = suffixed_define("ncolors");
        let re_chars_per_pixel = suffixed_define("chars_per_pixel");

        let format = re_format
            .captures(input)
            .ok_or_else(|| anyhow!("Format not found"))?[1]
            .parse::<usize>()?;
        if format != 1 {
            return Err(anyhow!("Incorrect format number"));
        }

        let width = re_width
            .captures(input)
            .ok_or_else(|| anyhow!("Width not found"))?[1]
            .parse::<usize>()?;
        let height = re_height
            .captures(input)
            .ok_or_else(|| anyhow!("Height not found"))?[1]
            .parse::<usize>()?;
        let num_colors = re_ncolors
            .captures(input)
            .ok_or_else(|| anyhow!("Number of colors not found"))?[1]
            .parse::<usize>()?;
        let chars_per_pixel = re_chars_per_pixel
            .captures(input)
            .ok_or_else(|| anyhow!("Chars per pixel not found"))?[1]
            .parse::<usize>()?;

        let re_colors_list =
            RegexBuilder::new(r"static\s+char\s*\*\s*\w+_colors\s*\[\]\s*=\s*\{(.*?)\};")
                .dot_matches_new_line(true)
                .multi_line(true)
                .build()
                .unwrap();
        let colors_list = &re_colors_list
            .captures(input)
            .ok_or_else(|| anyhow!("Colors list not found"))?[1];

        let colors_list = colors_list
            .split(',')
            .map(Self::space_quote_stripper)
            .collect::<Result<Vec<String>, _>>()?;

        if colors_list.len() % 2 != 0 {
            return Err(anyhow!("Invalid colors list"));
        } else if colors_list.len() / 2 != num_colors {
            return Err(anyhow!(
                "Expected {num_colors} colors, found {}",
                colors_list.len() / 2
            ));
        }
        let colors_list = colors_list.iter().tuples();
        let mut color_mapping = HashMap::new();
        for (pattern, color) in colors_list {
            color_mapping.insert(pattern.to_owned(), Color::from_str(&color)?);
        }

        let re_pixels_list =
            RegexBuilder::new(r"static\s+char\s*\*\s*\w+_pixels\s*\[\]\s*=\s*\{(.*?)\};")
                .dot_matches_new_line(true)
                .multi_line(true)
                .build()
                .unwrap();
        let pixels_list = (&re_pixels_list
            .captures(input)
            .ok_or_else(|| anyhow!("Pixel data not found"))?[1])
            .to_owned();
        let pixels_list = pixels_list
            .split(',')
            .map(Self::space_quote_stripper)
            .collect::<Result<Vec<String>, _>>()?;
        let data = pixels_list.iter().join("").chars().collect();

        Ok(Self {
            format: XpmFormat::Xpm1,
            width,
            _height: height,
            _num_colors: num_colors,
            chars_per_pixel,
            color_mapping,
            data,
        })
    }

    fn parse_xpm2(input: &str) -> Result<Xpm> {
        let mut lines = input.lines();
        let _format = lines.next().ok_or_else(|| anyhow!("Format not found"))?;
        let mut values = lines
            .next()
            .ok_or_else(|| anyhow!("Values not found"))?
            .split_whitespace();
        let width: usize = values
            .next()
            .ok_or_else(|| anyhow!("Width not found"))?
            .parse()?;
        let height: usize = values
            .next()
            .ok_or_else(|| anyhow!("Height not found"))?
            .parse()?;
        let num_colors: usize = values
            .next()
            .ok_or_else(|| anyhow!("Number of colors not found"))?
            .parse()?;
        let chars_per_pixel: usize = values
            .next()
            .ok_or_else(|| anyhow!("Chars per pixel not found"))?
            .parse()?;

        let mut color_mapping = HashMap::new();
        for i in 0..num_colors {
            let mut definition = lines
                .next()
                .ok_or_else(|| anyhow!("Expected {num_colors} colors, found {i}"))?
                .split_whitespace();
            let pixel = definition
                .next()
                .ok_or_else(|| anyhow!("Pixel symbol not found"))?;
            if pixel.len() != chars_per_pixel {
                return Err(anyhow!(
                    "Expected {chars_per_pixel} chars per pixel, got \"{pixel}\" ({} chars)",
                    pixel.len()
                ));
            }
            let c = definition
                .next()
                .ok_or_else(|| anyhow!("Color type for \"{pixel}\" not found"))?;
            if c != "c" {
                // TODO: support more than just `c` color types
                return Err(anyhow!("Color types other than \"c\" not yet implemented"))
            }
            let color = Color::from_str(
                definition
                    .next()
                    .ok_or_else(|| anyhow!("Color definition for \"{pixel}\" not found"))?,
            )?;
            color_mapping.insert(pixel.to_owned(), color);
        }

        let data = lines.flat_map(|x| x.chars()).collect();
        Ok(Self {
            format: XpmFormat::Xpm2,
            width,
            _height: height,
            _num_colors: num_colors,
            chars_per_pixel,
            color_mapping,
            data,
        })
    }

    fn parse_xpm3(input: &str) -> Result<Xpm> {
        // XPM3 is essentially the same as XPM2, but with C syntax
        // So transform the input
        let re = RegexBuilder::new(r"static\s+char\s*\*\s*\w+\s*\[\]\s*=\s*\{(.*?)\};")
            .dot_matches_new_line(true)
            .multi_line(true)
            .build()
            .unwrap();
        let data = &re
            .captures(input)
            .ok_or_else(|| anyhow!("XPM3 array not found"))?[1];

        let colors_list = String::from("! XPM2\n")
            + &data
                .split(',')
                .map(Self::space_quote_stripper)
                .collect::<Result<Vec<String>, _>>()?
                .join("\n");
        let mut xpm = Self::parse_xpm2(&colors_list)?;
        xpm.format = XpmFormat::Xpm3;
        Ok(xpm)
    }
}

impl Parser for Xpm {
    fn print(&self, _r: u8, _g: u8, _b: u8) {
        for (i, pixel) in self
            .data
            .iter()
            .chunks(self.chars_per_pixel)
            .into_iter()
            .enumerate()
        {
            let pixel = String::from_iter(pixel);
            if let Some(color) = self.color_mapping.get(&pixel) {
                print!("{color}");
            } else {
                println!();
                eprintln!("Found unknown pixel symbol: \"{pixel}\"");
                break;
            }
            if (i + 1) % self.width == 0 {
                println!();
            }
        }
    }
}
