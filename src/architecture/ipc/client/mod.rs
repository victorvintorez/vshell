use super::Ipc;
use crate::architecture::ipc::request::Request;
use crate::architecture::ipc::response::Response;
use crate::fl;
use color_eyre::eyre::ErrReport;
use color_eyre::{Help, Report, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tracing::debug;

impl Ipc {
    pub async fn send(&self, req: Request, debug: bool) -> Result<Response> {
        let mut stream = match UnixStream::connect(&self.path).await {
            Ok(stream) => Ok::<UnixStream, ErrReport>(stream),
            Err(e) => {
                return Err(Report::new(e)
                    .wrap_err(fl!("architecture-ipc-client_error_ipc-socket-connect-fail"))
                    .suggestion(fl!(
                        "architecture-ipc-client_error_ipc-socket-connect-suggestion"
                    )))
            }
        }?;

        if debug {
            debug!(
                "{}",
                fl!(
                    "architecture-ipc-client_debug_ipc-sent-request",
                    request = format!("{:?}", req)
                )
            );
        }

        let write_buffer = rmp_serde::to_vec(&req)?;

        stream.write_all(&write_buffer).await?;

        let mut read_buffer = vec![0; 1024];
        let bytes = stream.read(&mut read_buffer).await?;

        let response = rmp_serde::from_slice(&read_buffer[..bytes])?;

        Ok(response)
    }
}
