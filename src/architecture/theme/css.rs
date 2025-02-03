use crate::{fl, glib_recv_mpsc, spawn, try_send};
use gtk4::prelude::{GtkApplicationExt, WidgetExt};
use gtk4::{gdk, glib, Application, CssProvider};
use notify::event::ModifyKind;
use notify::{recommended_watcher, Error, Event, EventKind, RecursiveMode, Watcher};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, info};

pub fn load_styles(style_path: PathBuf, style_type: StyleExt, app: Application) {
    let style_path = if style_path.is_absolute() {
        style_path
    } else {
        env::current_dir()
            .expect(&fl!(
                "architecture-theme-style_expect_style-path-current-dir"
            ))
            .join(style_path)
    };

    let provider = CssProvider::new();

    provider.load_from_string(css_to_string(style_path.clone(), style_type).as_str());

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default()
            .expect(&fl!("architecture-theme-style_expect_gdk-display-default")),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );

    let (tx, rx) = mpsc::channel(8);

    spawn(async move {
        let style_path2 = style_path.clone();
        let mut watcher = recommended_watcher(move |res: Result<Event, Error>| match res {
            Ok(ev) if matches!(ev.kind, EventKind::Modify(ModifyKind::Data(_))) => {
                debug!(
                    "{}",
                    fl!(
                        "architecture-theme-style_debug_style-watch-event",
                        event = format!("{:?}", ev)
                    )
                );
                if ev.paths.first().is_some_and(|p| p == &style_path2) {
                    try_send!(tx, style_path2.clone());
                }
            }
            Err(e) => error!(
                "{}",
                fl!(
                    "architecture-theme-style_error_style-watch-fail",
                    error = format!("{:?}", e)
                )
            ),
            _ => {}
        })
        .expect(&fl!("architecture-theme-style_expect_build-style-watcher"));

        let dir_path = style_path.parent().expect(&fl!(
            "architecture-theme-style_expect_style-path-parent-dir"
        ));

        watcher
            .watch(dir_path, RecursiveMode::NonRecursive)
            .expect(&fl!("architecture-theme-style_expect_start-style-watcher"));

        debug!(
            "{}",
            fl!(
                "architecture-theme-style_debug_style-file-watching",
                path = format!("{:?}", style_path.display())
            )
        );

        loop {
            sleep(Duration::from_secs(1)).await;
        }
    });

    glib_recv_mpsc!(rx, file_path => {
        info!("{}", fl!("architecture-theme-style_info_style-file-reloading", path = format!("{:?}", file_path.display())));
        provider.load_from_string(css_to_string(file_path, style_type).as_str());
        for win in app.windows() {
            win.queue_draw();
        };
    });
}

fn css_to_string(style_path: PathBuf, style_type: StyleExt) -> String {
    match style_type {
        StyleExt::Sass => {
            grass::from_path(style_path, &grass::Options::default()).unwrap_or_else(|err| {
                error!(
                    "{}",
                    fl!(
                        "architecture-theme-style_error_style-file-load-sass",
                        error = format!("{:?}", err)
                    )
                );
                String::new()
            })
        }
        StyleExt::Scss => {
            grass::from_path(style_path, &grass::Options::default()).unwrap_or_else(|err| {
                error!(
                    "{}",
                    fl!(
                        "architecture-theme-style_error_style-file-load-scss",
                        error = format!("{:?}", err)
                    )
                );
                String::new()
            })
        }
        StyleExt::Css => std::fs::read_to_string(style_path).unwrap_or_else(|err| {
            error!(
                "{}",
                fl!(
                    "architecture-theme-style_error_style-file-load-css",
                    error = format!("{:?}", err)
                )
            );
            String::new()
        }),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StyleExt {
    Sass,
    Scss,
    Css,
}

impl TryFrom<&Path> for StyleExt {
    type Error = &'static str;

    fn try_from(style_path: &Path) -> Result<Self, Self::Error> {
        if style_path.is_file() {
            match style_path.extension().and_then(|ext| ext.to_str()) {
                Some("sass") => Ok(StyleExt::Sass),
                Some("scss") => Ok(StyleExt::Scss),
                Some("css") => Ok(StyleExt::Css),
                _ => Err("Not a valid style file"),
            }
        } else if style_path.join("style.sass").exists() {
            Ok(StyleExt::Sass)
        } else if style_path.join("style.scss").exists() {
            Ok(StyleExt::Scss)
        } else if style_path.join("style.css").exists() {
            Ok(StyleExt::Css)
        } else {
            Err("Style File not found")
        }
    }
}

impl std::fmt::Display for StyleExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleExt::Sass => write!(f, "sass"),
            StyleExt::Scss => write!(f, "scss"),
            StyleExt::Css => write!(f, "css"),
        }
    }
}
