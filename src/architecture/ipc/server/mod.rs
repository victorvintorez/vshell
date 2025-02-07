mod debug;

use super::Ipc;
use crate::architecture::ipc::request::Request;
use crate::architecture::ipc::response::Response;
use crate::architecture::ipc::server::debug::handle_request;
use crate::{fl, glib_recv_mpsc, send_async, spawn, try_send, VShell};
use color_eyre::Result;
use gtk4::glib;
use gtk4::Application;
use std::path::Path;
use std::rc::Rc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, error, info, warn};

impl Ipc {
    pub fn start(&self, application: &Application, vshell: Rc<&VShell>) {
        let (req_tx, req_rx) = mpsc::channel(32);
        let (res_tx, mut res_rx) = mpsc::channel(32);

        let ipc_path = self.path.clone();

        if ipc_path.exists() {
            warn!("{}", fl!("architecture-ipc-server_warn_ipc-socket-exists"));
            Self::shutdown(&ipc_path);
        }

        spawn(async move {
            info!(
                "{}",
                fl!(
                    "architecture-ipc-server_info_ipc-socket-starting",
                    path = format!("{:?}", ipc_path)
                )
            );

            let listener = match UnixListener::bind(&ipc_path) {
                Ok(listener) => listener,
                Err(e) => {
                    error!(
                        "{}",
                        fl!(
                            "architecture-ipc-server_error_ipc-socket-bind-fail",
                            error = format!("{:?}", e)
                        )
                    );
                    return;
                }
            };

            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => {
                        if let Err(e) = Self::handle_connection(stream, &req_tx, &mut res_rx).await
                        {
                            error!(
                                "{}",
                                fl!(
                                    "architecture-ipc-server_error_handle-connection-fail",
                                    error = format!("{:?}", e)
                                )
                            );
                        }
                    }
                    Err(e) => error!(
                        "{}",
                        fl!(
                            "architecture-ipc-server_error_ipc-stream-accept-fail",
                            error = format!("{:?}", e)
                        )
                    ),
                }
            }
        });

        let application = application.clone();

        glib_recv_mpsc!(req_rx, request => {
            let res = Self::handle_request(request, &application, &vshell);
            try_send!(res_tx, res);
        });
    }

    async fn handle_connection(
        mut stream: UnixStream,
        req_tx: &Sender<Request>,
        res_rx: &mut Receiver<Response>,
    ) -> Result<()> {
        let (mut stream_read, mut stream_write) = stream.split();

        let mut read_buffer = vec![0; 1024];

        let bytes = stream_read.read(&mut read_buffer).await?;

        let req = rmp_serde::from_slice::<Request>(&read_buffer[..bytes])?;

        debug!(
            "{}",
            fl!(
                "architecture-ipc-server_debug_ipc-received-request",
                request = format!("{:?}", req)
            )
        );

        send_async!(req_tx, req);

        let response = res_rx
            .recv()
            .await
            .unwrap_or(Response::Error { message: None });

        let response = rmp_serde::to_vec(&response)?;

        stream_write.write_all(&response).await?;
        stream_write.shutdown().await?;

        Ok(())
    }

    fn handle_request(
        request: Request,
        application: &Application,
        vshell: &Rc<&VShell>,
    ) -> Response {
        match request {
            Request::Debug(request) => handle_request(request, application),
        }
    }

    pub fn shutdown<P: AsRef<Path>>(path: P) {
        if let Err(e) = std::fs::remove_file(path) {
            error!(
                "{}",
                fl!(
                    "architecture-ipc-server_error_ipc-shutdown-fail",
                    error = format!("{:?}", e)
                )
            );
        }
    }
}
