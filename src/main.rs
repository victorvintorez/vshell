mod config;
mod error;
mod macros;
mod services;
mod logging;

use crate::services::theme::style::load_styles;
use crate::services::theme::style::StyleExt;
use dirs::config_dir;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::future::Future;
use std::path::PathBuf;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tracing::{error, info};

fn main() {
    let _ = logging::setup_logging();

    println!("vshell starting...");

    let vshell = VShell::new();
    vshell.start();
}

pub struct VShell {
    config: config::Config,
}

impl VShell {
    fn new() -> Self {
        let config = config::load_config();

        Self { config }
    }

    fn start(self) {
        info!("Starting vshell {}...", env!("CARGO_PKG_VERSION"));

        let app = gtk4::Application::builder()
            .application_id("dev.vintorez.vshell")
            .build();

        let running = AtomicBool::new(false);

        app.connect_activate(move |app| {
            if running.load(Ordering::Relaxed) {
                info!("vshell is already running");
                return;
            }

            running.store(true, Ordering::Relaxed);

            let style_path = PathBuf::from(config_dir().map_or_else(
                || {
                    error!("Failed to get config directory");
                    exit(1);
                },
                |dir| dir.join("vshell"),
            ));

            if style_path.join("style.sass").exists() {
                info!("Found style.sass in config directory");
                load_styles(style_path, StyleExt::Sass, app.clone());
            } else if style_path.join("style.scss").exists() {
                info!("Found style.scss in config directory");
                load_styles(style_path, StyleExt::Scss, app.clone());
            } else if style_path.join("style.css").exists() {
                info!("Found style.css in config directory");
                load_styles(style_path, StyleExt::Css, app.clone());
            } else {
                error!("Failed to find style.sass, style.scss, or style.css in config directory");
            }
        });
    }

    pub fn runtime() -> Arc<Runtime> {
        static RUNTIME: OnceLock<Arc<Runtime>> = OnceLock::new();
        RUNTIME.get_or_init(|| Arc::new(create_runtime())).clone()
    }
}

fn create_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("to create runtime")
}

pub fn spawn<F>(f: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    VShell::runtime().spawn(f)
}
