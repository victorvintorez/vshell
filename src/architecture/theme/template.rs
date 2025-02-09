use crate::config::TemplateConfig;
use crate::fl;
use color_eyre::Report;
use material_colors::theme::Theme;
use std::collections::HashMap;
use std::format;
use std::path::Path;
use std::process::Command;
use tracing::{error, info, warn};
use upon::{Engine, Value};

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
        if let Some(templates) = &self.templates {
            info!(
                "{:?}",
                &fl!(
                    "architecture-theme-template_info_templates-loading",
                    count = templates.len()
                )
            );

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

                // run pre hook
                if let Some(pre) = &template.pre {
                    info!(
                        "{:?}",
                        fl!(
                            "architecture-theme-template_info_pre-hook-running",
                            name = tmpl_name,
                            hook = pre
                        )
                    );

                    match Command::new("/bin/sh").args(["-c", pre]).output() {
                        Ok(output) => {
                            if output.status.success() {
                                info!(
                                    "{}",
                                    fl!(
                                        "architecture-theme-template_warn_pre-hook-result-success",
                                        name = tmpl_name,
                                        hook = pre,
                                        output = format!("{:?}", output)
                                    )
                                );
                            } else {
                                error!(
                                    "{}",
                                    fl!(
                                        "architecture-theme-template_error_pre-hook-result-fail",
                                        name = tmpl_name,
                                        hook = pre,
                                        output = format!("{:?}", output)
                                    )
                                );
                            }
                        }
                        Err(e) => {
                            error!(
                                "{}",
                                fl!(
                                    "architecture-theme-template_error_pre-hook-output-fail",
                                    name = tmpl_name,
                                    hook = pre,
                                    error = format!("{:?}", e)
                                )
                            );
                        }
                    }
                }

                // TODO: generate template

                // run post hook
                if let Some(post) = &template.post {
                    info!(
                        "{:?}",
                        fl!(
                            "architecture-theme-template_info_post-hook-running",
                            name = tmpl_name,
                            hook = post
                        )
                    );

                    match Command::new("/bin/sh").args(["-c", post]).output() {
                        Ok(output) => {
                            if output.status.success() {
                                info!(
                                    "{}",
                                    fl!(
                                        "architecture-theme-template_warn_post-hook-result-success",
                                        name = tmpl_name,
                                        hook = post,
                                        output = format!("{:?}", output)
                                    )
                                );
                            } else {
                                error!(
                                    "{}",
                                    fl!(
                                        "architecture-theme-template_error_post-hook-result-fail",
                                        name = tmpl_name,
                                        hook = post,
                                        output = format!("{:?}", output)
                                    )
                                );
                            }
                        }
                        Err(e) => {
                            error!(
                                "{}",
                                fl!(
                                    "architecture-theme-template_error_post-hook-output-fail",
                                    name = tmpl_name,
                                    hook = post,
                                    error = format!("{:?}", e)
                                )
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn theme_to_renderdata(theme: &Theme) -> Result<Value, Report> {
        // Impl Theme to Render Data
    }
}

pub fn init_template_engine() -> Engine<'static> {
    let syntax = upon::Syntax::default();
    Engine::with_syntax(syntax)
}
