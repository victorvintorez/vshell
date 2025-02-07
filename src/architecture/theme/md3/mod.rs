use crate::architecture::theme::template::TemplateManager;
use crate::config::TemplateConfig;
use crate::fl;
use material_colors::color::Argb;
use material_colors::{
    image::{FilterType, ImageReader},
    theme::{Theme, ThemeBuilder},
};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

pub struct ThemeManager<'a> {
    pub wallpaper_path: PathBuf,
    pub theme: Theme,
    pub template_manager: TemplateManager<'a>,
}

impl ThemeManager<'_> {
    pub fn new<'a>(templates: &'a Option<HashMap<String, TemplateConfig>>) -> ThemeManager<'a> {
        let theme = ThemeBuilder::with_source(Argb::from_u32(0xffffffff)).build();
        let template_manager = TemplateManager::new(templates);

        ThemeManager {
            wallpaper_path: Path::new("").to_path_buf(),
            theme,
            template_manager,
        }
    }

    pub fn init_theme(&mut self) {
        let image = std::fs::read(&self.wallpaper_path)
            .expect(&fl!("architecture-theme-md3_expect_wallpaper-read"));
        let mut data =
            ImageReader::read(image).expect(&fl!("architecture-theme-md3_expect_image-read"));
        data.resize(128, 128, FilterType::Lanczos3);

        self.theme = ThemeBuilder::with_source(ImageReader::extract_color(&data)).build();
    }
}
