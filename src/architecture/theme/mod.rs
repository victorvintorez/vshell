pub(crate) mod color;
pub(crate) mod css;
pub(crate) mod format;
pub(crate) mod template;

use crate::architecture::theme::template::TemplateManager;
use crate::config::TemplateConfig;
use color::SchemesEnum;
use color_eyre::eyre::Context;
use color_eyre::{eyre::Report, Result};
use expanduser::expanduser;
use material_colors::color::Argb;
use material_colors::{
    image::{FilterType, ImageReader},
    theme::{Theme, ThemeBuilder},
};
use std::collections::HashMap;
use std::fs::read;
use std::path::{Path, PathBuf};

pub struct ThemeManager {
    pub wallpaper_path: PathBuf,
    pub source_color: Argb,
    pub theme: Theme,
    pub color_scheme: SchemesEnum,
    pub template_manager: TemplateManager,
}

impl ThemeManager {
    pub fn new(templates: Option<HashMap<String, TemplateConfig>>, config_dir: &Path,te(&self.theme, &self.wallpaper_path, SchemesEnum::Dark);
        Ok(())
    }

    pub fn update_wallpaper(&mut self, wallpaper: String) -> Result<(), Report> {
        let path = expanduser(wallpaper).wrap_err("TODO: i18n")?;
        self.wallpaper_path = path;

        self.update_theme().wrap_err("TODO: i18n")?;
        Ok(())
    }

    pub fn update_scheme(&mut self, new_scheme: SchemesEnum) -> Result<(), Report> {
        self.color_scheme = new_scheme;

        self.update_theme().wrap_err("TODO: i18n");
        Ok(())
    }
}
