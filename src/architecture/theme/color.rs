use color_eyre::Report;
use colorsys::Hsl;
use material_colors::color::Argb;
use serde::{Deserialize, Serialize};
use std::result::Result;
use std::{
    collections::{BTreeSet, HashMap},
    fmt::{Display, Formatter, Result as FmtResult},
    iter::zip,
};

use super::format::{argb_to_rgb, fmt_hex, fmt_hex_strip, fmt_hsl, fmt_hsla, fmt_rgb, fmt_rgba};

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
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
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
    let mut colors: HashMap<String, ColorVariants> = Default::default();

    for ((name, light), (_, dark)) in zip(&color_schemes.light, &color_schemes.dark) {
        colors.insert(
            name.to_string(),
            transform_color(name, source_color, default_scheme, *light, *dark)?,
        );
    }

    colors.insert(
        String::from("source_color"),
        transform_color(
            "source_color",
            source_color,
            default_scheme,
            *source_color,
            *source_color,
        )?,
    );

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
            default: argb_to_color(*source_color),
            light: argb_to_color(*source_color),
            dark: argb_to_color(*source_color),
        });
    }

    Ok(ColorVariants {
        default: argb_to_color(default_scheme_color),
        light: argb_to_color(color_light),
        dark: argb_to_color(color_dark),
    })
}

fn argb_to_color(color: Argb) -> Color {
    let base_color = argb_to_rgb(color);
    let hsl_color = Hsl::from(&base_color);
    Color {
        hex: fmt_hex(&base_color),
        hex_stripped: fmt_hex_strip(&base_color),
        rgb: fmt_rgb(&base_color),
        rgba: fmt_rgba(&base_color, true),
        hsl: fmt_hsl(&hsl_color),
        hsla: fmt_hsla(&hsl_color, true),
        red: format!("{:?}", base_color.red() as u8),
        green: format!("{:?}", base_color.green() as u8),
        blue: format!("{:?}", base_color.blue() as u8),
        alpha: format!("{:?}", base_color.alpha() as u8),
        hue: format!("{:?}", &hsl_color.hue()),
        lightness: format!("{:?}", &hsl_color.lightness()),
        saturation: format!("{:?}", &hsl_color.saturation()),
    }
}
