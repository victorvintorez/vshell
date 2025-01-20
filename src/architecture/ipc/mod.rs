mod server;
mod request;
mod response;

use std::path::{Path, PathBuf};
use tracing::warn;

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
            warn!("The IPC socket file's absolute path is too long. This may cause issues with some systems.");
        }
        
        Self {
            path: ipc_socket
        }
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
}