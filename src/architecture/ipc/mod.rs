mod server;
pub(crate) mod request;
pub(crate) mod response;
mod client;

use std::path::{Path, PathBuf};
use tracing::warn;
use crate::fl;

#[derive(Debug)]
pub struct Ipc {
    path: PathBuf
}

impl Ipc {
    pub fn new() -> Self {
        let ipc_socket = std::env::var("XDG_RUNTIME_DIR")
            .map_or_else(|_| PathBuf::from("/tmp"), PathBuf::from)
            .join("vshell.sock");
        
        if format!("{}", ipc_socket.display()).len() > 100 {
            warn!("{}", fl!("architecture-ipc_warn_ipc-socket-length"));
        }
        
        Self {
            path: ipc_socket
        }
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
}