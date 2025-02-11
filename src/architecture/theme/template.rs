use crate::config::TemplateConfig;
use crate::fl;
use color_eyre::Report;
use material_colors::theme::Theme;
use std::format;
use std::iter::zip;
use std::path::Path;
use std::process::Command;
use std::result::Result;
use std::{collections::HashMap, rc::Rc};
use tracing::{error, info, warn};
use upon::{value, Engine, Value};

use super::color::{transform_color, ColorVariants, SchemesEnum};

pub struct TemplateManager {
    pub templates: Option<HashMap<String, TemplateConfig>>,
    pub engine: Engine<'static>,
}

impl TemplateManager {
    pub fn new(templates: Option<HashMap<String, TemplateConfig>>) -> Self {
        let engine = init_template_engine();
        TemplateManager { templates, engine }
    }

    pub fn generate(&self, theme: Rc<Theme>) {
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

    pub fn theme_to_renderdata(
        theme: &Theme,
        wallpaper_path: Option<&String>,
        default_scheme: SchemesEnum,
    ) -> Result<Value, Report> {
        let mut colors: HashMap<String, ColorVariants> = Default::default();

        // FIXME: figure out why this isnt working
        for ((name, light), (_, dark)) in zip(
            theme.schemes.light.into_iter(),
            theme.schemes.dark.into_iter(),
        ) {
            colors.insert(
                name.to_string(),
                transform_color(&name, theme.source, default_scheme, light, dark)?,
            );
        }

        colors.insert(
            String::from("source_color"),
            transform_color(
                "source_color",
                theme.source,
                default_scheme,
                theme.source,
                theme.source,
            )?,
        );

        Ok(value! {
            colors: colors,
            wallpaper_path: wallpaper_path,
        })
    }
}

pub fn init_template_engine() -> Engine<'static> {
    let syntax = upon::Syntax::default();
    Engine::with_syntax(syntax)
}
