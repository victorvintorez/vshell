mod architecture;
mod config;
mod error;
mod logging;
mod macros;

use crate::architecture::theme::style::load_styles;
use crate::architecture::theme::style::StyleExt;
use clap::Parser;
use color_eyre::Report;
use dirs::config_dir;
use gtk4::prelude::*;
use std::env;
use std::future::Future;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, OnceLock};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

fn main() {
    let _guard = logging::setup_logging();

    let args = architecture::cli::Args::parse();

    match args.command {
        Some(command) => {
            let rt = create_runtime();
            rt.block_on(async move {
                // SEND IPC COMMAND
            })
        }
        None => VShell::new(args.config_path).start(args.style_path),
    }
}

#[derive(Debug)]
pub struct VShell {
    config: config::Config,
    config_dir: PathBuf,
}

impl VShell {
    fn new(config_dir: Option<PathBuf>) -> Self {
        let (config, config_dir) = config::load_config(config_dir);
        Self { config, config_dir }
    }

    fn start(self, style_path: Option<PathBuf>) {
        info!("Starting vshell {}...", env!("CARGO_PKG_VERSION"));

        let app = gtk4::Application::builder()
            .application_id("dev.vintorez.vshell")
            .build();

        let running = AtomicBool::new(false);

        let (activate_tx, activate_rx) = mpsc::channel();

        let instance = Rc::new(self);
        let instance2 = instance.clone();
        let args = Rc::clone(&instance);

        // start wayland client

        app.connect_activate(move |app| {
            if running.load(Ordering::Relaxed) {
                info!("vshell is already running");
                return;
            }

            running.store(true, Ordering::Relaxed);

            // start IPC

            let mut style_path = style_path.clone().unwrap_or_else(|| PathBuf::from(config_dir().map_or_else(
                || {
                    error!("Failed to get style path");
                    exit(1);
                },
                |dir| dir.join("vshell"),
            )));
            
            match StyleExt::from_path(&style_path) {
                Some(ext) => {
                    if style_path.is_dir() {
                        style_path = style_path.join(format!("style.{ext}"))
                    };
                    let pretty_path = style_path.parent().expect("to have parent directory")
                        .canonicalize()
                        .map_err(Report::new)
                        .unwrap_or_else(|_| env::current_dir().expect("to have current directory"));
                    debug!("Using style.{} from: {}/", ext, pretty_path.display());
                    load_styles(style_path, ext, app.clone());
                }
                None => {
                    error!(
                        "Failed to find style.sass, style.scss, or style.css in config directory"
                    );
                }
            }

            let (tx, rx) = mpsc::channel();

            spawn_blocking(move || {
                rx.recv().expect("to receive signal on channel");

                info!("Shutting down vshell...");

                // shutdown IPC
                exit(0);
            });

            ctrlc::set_handler(move || tx.send(()).expect("to send signal on channel"))
                .expect("to set ctrl-c handler");

            let hold = app.hold();
            send!(activate_tx, hold);
        });

        {
            let instance = instance2.clone();
            let app = app.clone();

            // Load layers
        }

        app.run_with_args(&Vec::<&str>::new());
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

pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    VShell::runtime().spawn_blocking(f)
}
