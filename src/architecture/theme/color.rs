use std::{collections::{BTreeSet, HashMap}, fmt::{Display, Formatter, Result}, iter::zip};

use clap::ValueEnum;
use material_colors::color::Argb;
use serde::{Deserialize, Serialize};
use color_eyre::{Report, Result};

#[derive(Debug)]
pub struct ColorSchemes {
    pub light: BTreeSet<(String, Argb)>,
    pub dark: BTreeSet<(String, Argb)>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SchemesEnum {
    Light,
    Dark,
}

impl Display for SchemesEnum {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let str = match self {
            SchemesEnum::Light => "light",
            SchemesEnum::Dark => "dark",
        };

        write!(f, "{str}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    hex: String,
    hex_stripped: String,
    rgb: String,
    rgba: String,
    hsl: String,
    hsla: String,
    red: String,
    green: String,
    blue: String,
    alpha: String,
    hue: String,
    saturation: String,
    lightness: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorVariants {
    pub light: Color,
    pub dark: Color,
    pub default: Color,
}

pub fn transform_colors(
    color_schemes: &ColorSchemes,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
) -> Result<HashMap<String, ColorVariants>, Report> {
    // TODO: Transform Schemes to Color Result
    let mut colors: HashMap<String, ColorVariants> = Default::default();

    for ((name, light), (_, dark)) in zip::(&color_schemes.light, &color_schemes.dark) {
        colors.insert(name.to_string(), )
    }

    Ok(colors)
}
pub fn transform_color(
    field: &str,
    source_color: &Argb,
    default_scheme: &SchemesEnum,
    color_light: Argb,
    color_dark: Argb,
) -> Result<ColorVariants, Report> {
    let default_scheme_color = match default_scheme {
        SchemesEnum::Light => color_light,
        SchemesEnum::Dark => color_dark,
    };

    if field == "source_color" {
        return Ok(ColorVariants {
            default: // TODO: Generate Color Strings
        })
    }
}

fn argb_to_color(color: Argb) -> Color {
    let base_color = // TODO: transform
}
