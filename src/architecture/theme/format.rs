use colorsys::{ColorAlpha, Hsl, Rgb};
use material_colors::color::Argb;

pub fn argb_to_rgb(color: Argb) -> Rgb {
    Rgb::from([
        color.red as f64,
        color.green as f64,
        color.blue as f64,
        color.alpha as f64,
    ])
}

pub fn argb_to_hsl(color: Argb) -> Hsl {
    argb_to_rgb(color).as_ref().into()
}

pub fn rgb_to_hsl(color: Rgb) -> Hsl {
    color.as_ref().into()
}

pub fn fmt_hex(color: &Rgb) -> String {
    color.to_hex_string()
}

pub fn fmt_hex_strip(color: &Rgb) -> String {
    color.to_hex_string()[1..].to_string()
}

pub fn fmt_rgb(color: &Rgb) -> String {
    format!(
        "rgb({:?}, {:?}, {:?})",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8
    )
}

pub fn fmt_rgba(color: &Rgb, divide: bool) -> String {
    if divide {
        format!(
            "rgba({:?}, {:?}, {:?}, {:.1})",
            color.red() as u8,
            color.green() as u8,
            color.blue() as u8,
            color.alpha() / 255.
        )
    }
    format!(
        "rgba({:?}, {:?}, {:?}, {:.1})",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8,
        color.alpha()
    )
}

pub fn fmt_hsl(color: &Hsl) -> String {
    format!(
        "hsl({:?}, {:?}, {:?})",
        color.hue() as u8,
        color.saturation() as u8,
        color.lightness() as u8
    )
}

pub fn fmt_hsla(color: &Hsl, divide: bool) -> String {
    if divide {
        return format!(
            "hsla({:?}, {:?}, {:?}, {:.1})",
            color.hue() as u8,
            color.saturation() as u8,
            color.lightness() as u8,
            color.alpha() / 255.
        );
    }
    return format!(
        "hsla({:?}, {:?}, {:?}, {:.1})",
        color.hue() as u8,
        color.saturation() as u8,
        color.lightness() as u8,
        color.alpha()
    );
}
