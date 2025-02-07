mod architecture;
mod config;
mod macros;

use crate::architecture::cli;
use crate::architecture::ipc::Ipc;
use crate::architecture::logging;
use crate::architecture::theme::css::{load_styles, StyleExt};
use crate::architecture::theme::md3::ThemeManager;
use clap::Parser;
use color_eyre::Report;
use config::Config;
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

    let vshell = VShell::new(args.config_path);

    match args.command {
        Some(request) => {
            let rt = create_runtime();
            rt.block_on(async move {
                let ipc = Ipc::new();
                match ipc.send(request, args.debug).await {
                    Ok(res) => {
                        if args.debug {
                            debug!("Response: {:?}", res);
                        }

                        cli::handle_response(res);
                    }
                    Err(e) => error!("{:?}", e),
                };
            });
        }
        None => vshell.start(args.style_path),
    }
}

pub struct VShell {
    config: Config,
    config_dir: PathBuf,
    theme: ThemeManager,
}

impl VShell {
    fn new(config_dir: Option<PathBuf>) -> Self {
        let (config, config_dir) = config::load_config(config_dir);

        let templates = config.templates.clone();
        let theme = ThemeManager::new(templates);

        Self {
            config,
            config_dir,
            theme,
        }
    }

    fn start(&self, style_path: Option<PathBuf>) {
        info!(
            "{}",
            fl!(
                "main_info_starting-vshell",
                pkgversion = env!("CARGO_PKG_VERSION")
            )
        );

        let app = gtk4::Application::builder()
            .application_id("dev.vintorez.vshell")
            .build();

        let running = AtomicBool::new(false);

        let (activate_tx, activate_rx) = mpsc::channel();

        let instance = Rc::new(self);
        let instance2 = instance.clone();

        // start wayland client

        app.connect_activate(move |app| {
            if running.load(Ordering::Relaxed) {
                info!("{}", fl!("main_info_vshell-running"));
                return;
            }

            running.store(true, Ordering::Relaxed);

            let ipc = Ipc::new();
            ipc.start(app, instance.clone());

            let mut style_path = style_path.clone().unwrap_or_else(|| {
                config_dir().map_or_else(
                    || {
                        error!("{}", fl!("main_err_style-path-fail"));
                        exit(1);
                    },
                    |dir| dir.join("vshell"),
                )
            });

            match StyleExt::try_from(style_path.as_path()) {
                Ok(ext) => {
                    if style_path.is_dir() {
                        style_path = style_path.join(format!("style.{ext}"))
                    };
                    let pretty_path = style_path
                        .parent()
                        .expect(&fl!("main_expect_style-parent-dir"))
                        .canonicalize()
                        .map_err(Report::new)
                        .unwrap_or_else(|_| {
                            env::current_dir().expect(&fl!("main_expect_style-current-dir"))
                        });
                    debug!("Using style.{} from: {}/", ext, pretty_path.display());
                    load_styles(style_path, ext, app.clone());
                }
                Err(_e) => {
                    error!(
                        "{}",
                        fl!(
                            "main_err_style-file-fail",
                            path = format!("{:?}", style_path)
                        )
                    );
                }
            }

            let (tx, rx) = mpsc::channel();

            let ipc_path = ipc.path().to_path_buf();

            spawn_blocking(move || {
                rx.recv().expect(&fl!("main_expect_rx-receive-signal"));

                info!("{}", fl!("main_info_vshell-shutdown"));

                Ipc::shutdown(ipc_path);
                exit(0);
            });

            ctrlc::set_handler(move || tx.send(()).expect(&fl!("main_expect_tx-send-signal")))
                .expect(&fl!("main_expect_ctrl-c-handler"));

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
        .expect(&fl!("main_expect_tokio-runtime"))
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
