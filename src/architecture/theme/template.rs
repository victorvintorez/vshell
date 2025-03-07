use crate::config::TemplateConfig;
use crate::fl;
use crate::shell::{run_shell_cmd, ShellResult};
use color_eyre::Report;
use expanduser::expanduser;
use material_colors::theme::Theme;
use std::format;
use std::fs::read_to_string;
use std::io::Write;
use std::iter::zip;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;
use std::{collections::HashMap, fs::OpenOptions};
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

    pub fn generate(
        &mut self,
        theme: &Theme,
        wallpaper_path: Option<&PathBuf>,
        default_scheme: SchemesEnum,
    ) -> Result<String, Report> {
        if let Some(templates) = &self.templates {
            info!(
                "{:?}",
                &fl!(
                    "architecture-theme-template_info_templates-loading",
                    count = templates.len()
                )
            );

            let render_data = theme_to_renderdata(theme, wallpaper_path, default_scheme).wrap_err("TODO: i18n")?;

            let cmd_results = Vec<ShellResult>::new();

            for (tmpl_name, template) in templates.iter() {
                let template_path = expanduser(&template.template).wrap_err("TODO: i18n")?;
                let target_path = expanduser(&template.target).wrap_err("TODO: i18n")?;

                // run pre hook
                if let Some(pre) = &template.pre {
                    cmd_results.push(run_shell_cmd(&template.pre_shell, pre));
                }

                let tmpl_data = read_to_string(template_path).wrap_err("TODO: i18n")?;
                self.engine.add_template(tmpl_name.clone(), tmpl_data).wrap_err("TODO: i18n")?;
                let tmpl_rendered = self.engine.template(tmpl_name).render(&render_data).to_string().wrap_err("TODO: i18n")?;
                let mut target = OpenOptions::new().create(true).truncate(true).write(true).open(target_path).wrap_err("TODO: i18n")?;
                let metadata = target.metadata().wrap_err("TODO: i18n")?;

                if metadata.permissions().readonly()
                {
                    error!("TODO: i18n");
                    return Report::new("TODO: i18n");
                }

                target.write_all(tmpl_rendered.as_bytes()).wrap_err("TODO: i18n")?;

                if let Some(post) = &template.post {
                    cmd_results.push(run_shell_cmd(&template.post_shell, post))
                }
            }

            let (successes, errors, outputs): (int, int, Vec<&str>) = cmd_results.iter().map(|res| {
                if(res.success = true) {
                    successes += 1;
                } else {
                    errors += 1;
                }
                outputs.push(res.output)
            });

            Ok(format!("TODO: i18n, {success}, {error}, {output}\n", successes, errors, output.join("\n")))
        }
    }

    fn theme_to_renderdata(
        theme: &Theme,
        wallpaper_path: Option<&PathBuf>,
        default_scheme: SchemesEnum,
    ) -> Result<Value, Report> {
        let mut colors: HashMap<String, ColorVariants> = Default::default();

        // Stupid workaround because the library doesnt implement copy or iter
        let light_scheme = vec![
            ("primary", &theme.schemes.light.primary),
            ("on_primary", &theme.schemes.light.on_primary),
            ("primary_container", &theme.schemes.light.primary_container),
            (
                "on_primary_container",
                &theme.schemes.light.on_primary_container,
            ),
            ("inverse_primary", &theme.schemes.light.inverse_primary),
            ("primary_fixed", &theme.schemes.light.primary_fixed),
            ("primary_fixed_dim", &theme.schemes.light.primary_fixed_dim),
            ("on_primary_fixed", &theme.schemes.light.on_primary_fixed),
            (
                "on_primary_fixed_variant",
                &theme.schemes.light.on_primary_fixed_variant,
            ),
            ("secondary", &theme.schemes.light.secondary),
            ("on_secondary", &theme.schemes.light.on_secondary),
            (
                "secondary_container",
                &theme.schemes.light.secondary_container,
            ),
            (
                "on_secondary_container",
                &theme.schemes.light.on_secondary_container,
            ),
            ("secondary_fixed", &theme.schemes.light.secondary_fixed),
            (
                "secondary_fixed_dim",
                &theme.schemes.light.secondary_fixed_dim,
            ),
            (
                "on_secondary_fixed",
                &theme.schemes.light.on_secondary_fixed,
            ),
            (
                "on_secondary_fixed_variant",
                &theme.schemes.light.on_secondary_fixed_variant,
            ),
            ("tertiary", &theme.schemes.light.tertiary),
            ("on_tertiary", &theme.schemes.light.on_tertiary),
            (
                "tertiary_container",
                &theme.schemes.light.tertiary_container,
            ),
            (
                "on_tertiary_container",
                &theme.schemes.light.on_tertiary_container,
            ),
            ("tertiary_fixed", &theme.schemes.light.tertiary_fixed),
            (
                "tertiary_fixed_dim",
                &theme.schemes.light.tertiary_fixed_dim,
            ),
            ("on_tertiary_fixed", &theme.schemes.light.on_tertiary_fixed),
            (
                "on_tertiary_fixed_variant",
                &theme.schemes.light.on_tertiary_fixed_variant,
            ),
            ("error", &theme.schemes.light.error),
            ("on_error", &theme.schemes.light.on_error),
            ("error_container", &theme.schemes.light.error_container),
            (
                "on_error_container",
                &theme.schemes.light.on_error_container,
            ),
            ("surface_dim", &theme.schemes.light.surface_dim),
            ("surface", &theme.schemes.light.surface),
            ("surface_tint", &theme.schemes.light.surface_tint),
            ("surface_bright", &theme.schemes.light.surface_bright),
            (
                "surface_container_lowest",
                &theme.schemes.light.surface_container_lowest,
            ),
            (
                "surface_container_low",
                &theme.schemes.light.surface_container_low,
            ),
            ("surface_container", &theme.schemes.light.surface_container),
            (
                "surface_container_high",
                &theme.schemes.light.surface_container_high,
            ),
            (
                "surface_container_highest",
                &theme.schemes.light.surface_container_highest,
            ),
            ("on_surface", &theme.schemes.light.on_surface),
            (
                "on_surface_variant",
                &theme.schemes.light.on_surface_variant,
            ),
            ("outline", &theme.schemes.light.outline),
            ("outline_variant", &theme.schemes.light.outline_variant),
            ("inverse_surface", &theme.schemes.light.inverse_surface),
            (
                "inverse_on_surface",
                &theme.schemes.light.inverse_on_surface,
            ),
            ("surface_variant", &theme.schemes.light.surface_variant),
            ("background", &theme.schemes.light.background),
            ("on_background", &theme.schemes.light.on_background),
            ("shadow", &theme.schemes.light.shadow),
            ("scrim", &theme.schemes.light.scrim),
        ];

        let dark_scheme = vec![
            ("primary", &theme.schemes.dark.primary),
            ("on_primary", &theme.schemes.dark.on_primary),
            ("primary_container", &theme.schemes.dark.primary_container),
            (
                "on_primary_container",
                &theme.schemes.dark.on_primary_container,
            ),
            ("inverse_primary", &theme.schemes.dark.inverse_primary),
            ("primary_fixed", &theme.schemes.dark.primary_fixed),
            ("primary_fixed_dim", &theme.schemes.dark.primary_fixed_dim),
            ("on_primary_fixed", &theme.schemes.dark.on_primary_fixed),
            (
                "on_primary_fixed_variant",
                &theme.schemes.dark.on_primary_fixed_variant,
            ),
            ("secondary", &theme.schemes.dark.secondary),
            ("on_secondary", &theme.schemes.dark.on_secondary),
            (
                "secondary_container",
                &theme.schemes.dark.secondary_container,
            ),
            (
                "on_secondary_container",
                &theme.schemes.dark.on_secondary_container,
            ),
            ("secondary_fixed", &theme.schemes.dark.secondary_fixed),
            (
                "secondary_fixed_dim",
                &theme.schemes.dark.secondary_fixed_dim,
            ),
            ("on_secondary_fixed", &theme.schemes.dark.on_secondary_fixed),
            (
                "on_secondary_fixed_variant",
                &theme.schemes.dark.on_secondary_fixed_variant,
            ),
            ("tertiary", &theme.schemes.dark.tertiary),
            ("on_tertiary", &theme.schemes.dark.on_tertiary),
            ("tertiary_container", &theme.schemes.dark.tertiary_container),
            (
                "on_tertiary_container",
                &theme.schemes.dark.on_tertiary_container,
            ),
            ("tertiary_fixed", &theme.schemes.dark.tertiary_fixed),
            ("tertiary_fixed_dim", &theme.schemes.dark.tertiary_fixed_dim),
            ("on_tertiary_fixed", &theme.schemes.dark.on_tertiary_fixed),
            (
                "on_tertiary_fixed_variant",
                &theme.schemes.dark.on_tertiary_fixed_variant,
            ),
            ("error", &theme.schemes.dark.error),
            ("on_error", &theme.schemes.dark.on_error),
            ("error_container", &theme.schemes.dark.error_container),
            ("on_error_container", &theme.schemes.dark.on_error_container),
            ("surface_dim", &theme.schemes.dark.surface_dim),
            ("surface", &theme.schemes.dark.surface),
            ("surface_tint", &theme.schemes.dark.surface_tint),
            ("surface_bright", &theme.schemes.dark.surface_bright),
            (
                "surface_container_lowest",
                &theme.schemes.dark.surface_container_lowest,
            ),
            (
                "surface_container_low",
                &theme.schemes.dark.surface_container_low,
            ),
            ("surface_container", &theme.schemes.dark.surface_container),
            (
                "surface_container_high",
                &theme.schemes.dark.surface_container_high,
            ),
            (
                "surface_container_highest",
                &theme.schemes.dark.surface_container_highest,
            ),
            ("on_surface", &theme.schemes.dark.on_surface),
            ("on_surface_variant", &theme.schemes.dark.on_surface_variant),
            ("outline", &theme.schemes.dark.outline),
            ("outline_variant", &theme.schemes.dark.outline_variant),
            ("inverse_surface", &theme.schemes.dark.inverse_surface),
            ("inverse_on_surface", &theme.schemes.dark.inverse_on_surface),
            ("surface_variant", &theme.schemes.dark.surface_variant),
            ("background", &theme.schemes.dark.background),
            ("on_background", &theme.schemes.dark.on_background),
            ("shadow", &theme.schemes.dark.shadow),
            ("scrim", &theme.schemes.dark.scrim),
        ];

        for ((name, light), (_, dark)) in zip(light_scheme.into_iter(), dark_scheme.into_iter()) {
            colors.insert(
                name.to_string(),
                transform_color(name, theme.source, default_scheme, *light, *dark)?,
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
    Engine::new()
}
