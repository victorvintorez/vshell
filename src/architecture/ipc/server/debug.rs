use gtk4::Application;
use crate::architecture::ipc::request::DebugRequest;
use crate::architecture::ipc::response::Response;
use crate::fl;

pub fn handle_request(request: DebugRequest, application: &Application) -> Response {
    match request {
        DebugRequest::Ping => Response::Ok { message: Some(fl!("architecture-ipc-server-debug_string_pong")) },
        DebugRequest::Inspector => {
            gtk4::Window::set_interactive_debugging(true);
            Response::Ok { message: None }
        }
    }
}