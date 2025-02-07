use crate::config::TemplateConfig;
use crate::fl;
use crate::info;
use material_colors::theme::Theme;
use std::collections::HashMap;
use std::format;
use std::path::Path;
use tracing::warn;
use upon::Engine;

pub struct TemplateManager {
    pub templates: Option<HashMap<String, TemplateConfig>>,
    pub engine: Engine<'static>,
}

impl TemplateManager {
    pub fn new(templates: Option<HashMap<String, TemplateConfig>>) -> Self {
        let engine = init_template_engine();
        TemplateManager { templates, engine }
    }

    pub fn generate(&self, theme: &Theme) {
        info!(
            "{:?}",
            &fl!("architecture-theme-template_info_templates-loading")
        );

        if let Some(templates) = &self.templates {
            for (tmpl_name, template) in templates {
                let template_path = Path::new(&template.template);
                let target_path = Path::new(&template.target);

                if !template_path.exists() {
                    warn!(
                        "{:?}",
                        fl!(
                            "architecture-theme-template_warn_template-not-found",
                            name = tmpl_name,
                            path = format!("{:?}", template_path)
                        )
                    );
                    continue;
                } else if !target_path
                    .parent()
                    .expect(&fl!(
                        "architecture-theme-template_expect_template-target-dir"
                    ))
                    .exists()
                {
                    warn!(
                        "{:?}",
                        fl!(
                            "architecture-theme-template_error_template-target-dir-fail",
                            name = tmpl_name,
                            path = format!("{:?}", target_path)
                        )
                    );
                    continue;
                }

                // TODO: Implement template generation
            }
        }
    }
}

pub fn init_template_engine() -> Engine<'static> {
    let syntax = upon::Syntax::default();
    Engine::with_syntax(syntax)
}
