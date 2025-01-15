use crate::{glib_recv_mpsc, spawn, try_send};
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use gtk4::{gdk, glib, Application, CssProvider};
use gtk4::prelude::{GtkApplicationExt, WidgetExt};
use tracing::{debug, error, info};
use notify::{recommended_watcher, Error, Event, EventKind, RecursiveMode, Watcher};
use notify::event::ModifyKind;
use tokio::time::sleep;
use tokio::sync::mpsc;

pub fn load_styles(style_path: PathBuf, style_type: StyleExt, app: Application) {
    let style_path = if style_path.is_absolute() {
        style_path
    } else {
        env::current_dir().expect("to exist").join(style_path)
    };

    let provider = CssProvider::new();

    provider.load_from_string(css_to_string(style_path.clone(), style_type).as_str());

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("to exist"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );

    let (tx, rx) = mpsc::channel(8);

    spawn(async move {
        let style_path2 = style_path.clone();
        let mut watcher = recommended_watcher(move |res: Result<Event, Error>| match res {
            Ok(event) if matches!(event.kind, EventKind::Modify(ModifyKind::Data(_))) => {
                debug!("{event:?}");
                if event.paths.first().is_some_and(|p| p == &style_path2) {
                    try_send!(tx, style_path2.clone());
                }
            }
            Err(e) => error!("Error watching style file: {:?}", e),
            _ => {}
        })
            .expect("to watch style file");

        let dir_path = style_path.parent().expect("to exist");

        watcher
            .watch(dir_path, RecursiveMode::NonRecursive)
            .expect("to start watching style file");

        debug!("Watching style file: {:?}", style_path.display());

        loop {
            sleep(Duration::from_secs(1)).await;
        }
    });

    glib_recv_mpsc!(rx, path => {
        info!("Reloading style");
        provider.load_from_string(css_to_string(path, style_type).as_str());
        for win in app.windows() {
            win.queue_draw();
        };
    });
}

fn css_to_string(style_path: PathBuf, style_type: StyleExt) -> String {
    match style_type {
        StyleExt::Sass => grass::from_path(style_path, &grass::Options::default()).map_or_else(
            |err| {
                error!("Failed to load style.sass: {}", err);
                String::new()
            },
            |style| style,
        ),
        StyleExt::Scss => grass::from_path(style_path, &grass::Options::default()).map_or_else(
            |err| {
                error!("Failed to load style.scss: {}", err);
                String::new()
            },
            |style| style,
        ),
        StyleExt::Css => std::fs::read_to_string(style_path).map_or_else(
            |err| {
                error!("Failed to load style.css: {}", err);
                String::new()
            },
            |style| style,
        ),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StyleExt {
    Sass,
    Scss,
    Css,
}

impl StyleExt {
    pub fn from_path(style_path: &PathBuf) -> Option<Self> {
        if style_path.is_file() {
            match style_path.extension().and_then(|ext| ext.to_str()) {
                Some("sass") => Some(StyleExt::Sass),
                Some("scss") => Some(StyleExt::Scss),
                Some("css") => Some(StyleExt::Css),
                _ => None,
            }
        } else {
            if style_path.join("style.sass").exists() {
                Some(StyleExt::Sass)
            } else if style_path.join("style.scss").exists() {
                Some(StyleExt::Scss)
            } else if style_path.join("style.css").exists() {
                Some(StyleExt::Css)
            } else {
                None
            }
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
