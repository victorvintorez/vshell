use clap::ValueEnum;
use material_colors::color::Argb;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter, Result},
};

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
