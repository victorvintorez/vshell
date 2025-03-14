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
    pub theme: Theme,
    pub color_scheme: SchemesEnum,
    pub template_manager: TemplateManager,
}

impl ThemeManager {
    pub fn new(
        templates: Option<HashMap<String, TemplateConfig>>,
        config_dir: &Path,
        default_scheme: &SchemesEnum,
    ) -> Self {
        let theme = ThemeBuilder::with_source(Argb::from_u32(0xffffffff)).build();
        let template_manager = TemplateManager::new(templates);
        let wallpaper_path = match config_dir.join("default.png").exists() {
            true => config_dir.join("default.png").to_path_buf(),
            false => PathBuf::new(),
        };

        ThemeManager {
            wallpaper_path,
            theme,
            template_manager,
            color_scheme: default_scheme.clone(),
        }
    }

    pub fn update_theme(&mut self) -> Result<String, Report> {
        let image = read(&self.wallpaper_path).wrap_err("TODO: i18n")?;
        let mut data = ImageReader::read(image).wrap_err("TODO: i18n")?;
        data.resize(128, 128, FilterType::Lanczos3);

        self.theme = ThemeBuilder::with_source(ImageReader::extract_color(&data)).build();
        self.template_manager.generate(
            &self.theme,
            Some(&self.wallpaper_path),
            SchemesEnum::Dark,
        )?;

        Ok("".to_owned())
    }

    pub fn update_wallpaper(&mut self, wallpaper: String) -> Result<String, Report> {
        let path = expanduser(wallpaper).wrap_err("TODO: i18n")?;
        self.wallpaper_path = path;

        self.update_theme().wrap_err("TODO: i18n")?;
        Ok("TODO: i18n".to_owned())
    }

    pub fn update_scheme(&mut self, new_scheme: SchemesEnum) -> Result<String, Report> {
        self.color_scheme = new_scheme;

        self.update_theme().wrap_err("TODO: i18n")?;
        Ok("TODO: i18n".to_owned())
    }
}
