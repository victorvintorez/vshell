mod debug;

use std::path::Path;
use gtk4::glib;
use std::rc::Rc;
use tokio::sync::mpsc;
use gtk4::Application;
use color_eyre::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, error, info, warn};
use crate::{glib_recv_mpsc, send_async, spawn, try_send, VShell};
use crate::architecture::ipc::request::Request;
use crate::architecture::ipc::response::Response;
use crate::architecture::ipc::server::debug::handle_request;
use super::Ipc;

impl Ipc {
    pub fn start(&self, application: &Application, vshell: Rc<VShell>) {
        let (req_tx, req_rx) = mpsc::channel(32);
        let (res_tx, mut res_rx) = mpsc::channel(32);

        let path = self.path.clone();

        if path.exists() {
            warn!("The IPC socket file already exists. Removing it.");
            Self::shutdown(&path);
        }

        spawn(async move {
            info!("Starting IPC on {}", path.display());

            let listener = match UnixListener::bind(&path) {
                Ok(listener) => listener,
                Err(e) => {
                    error!("Failed to bind IPC socket: {:?}", e);
                    return;
                }
            };

            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => {
                        if let Err(e) = Self::handle_connection(stream, &req_tx, &mut res_rx).await {
                            error!("{:?}", e);
                        }
                    }
                    Err(e) => error!("{:?}", e)
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
        res_rx: &mut Receiver<Response>
    ) -> Result<()> {
        let (mut stream_read, mut stream_write) = stream.split();

        let mut read_buffer = vec![0; 1024];

        let bytes = stream_read.read(&mut read_buffer).await?;

        let request = bincode::deserialize::<Request>(&read_buffer[..bytes])?;

        debug!("Received request: {:?}", request);

        send_async!(req_tx, request);

        let response = res_rx.recv().await.unwrap_or(Response::Error { message: None });

        let response = bincode::serialize(&response)?;

        stream_write.write_all(&response).await?;
        stream_write.shutdown().await?;

        Ok(())
    }

    fn handle_request(
        request: Request,
        application: &Application,
        vshell: &Rc<VShell>
    ) -> Response {
        match request {
            Request::Debug(request) => handle_request(request, application),
        }
    }
    
    pub fn shutdown<P: AsRef<Path>>(path: P) {
        if let Err(e) = std::fs::remove_file(path) {
            error!("Failed to remove IPC socket file: {:?}", e);
        }
    }
}