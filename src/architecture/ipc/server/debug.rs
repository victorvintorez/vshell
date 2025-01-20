use gtk4::Application;
use crate::architecture::ipc::request::DebugRequest;
use crate::architecture::ipc::response::Response;

pub fn handle_request(request: DebugRequest, application: &Application) -> Response {
    match request {
        DebugRequest::Ping => Response::Ok { message: Some("Pong".to_string()) },
        DebugRequest::Inspector => {
            gtk4::Window::set_interactive_debugging(true);
            Response::Ok { message: None }
        }
    }
}