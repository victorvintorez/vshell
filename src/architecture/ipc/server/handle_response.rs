use std::cell::RefMut;

use crate::architecture::ipc::request::{DebugRequest, SchemeRequest, WallpaperRequest};
use crate::architecture::ipc::response::Response;
use crate::architecture::theme::color::SchemesEnum;
use crate::architecture::theme::ThemeManager;
use crate::fl;
use gtk4::Application;

pub fn handle_debug(request: DebugRequest, application: &Application) -> Response {
    match request {
        DebugRequest::Ping => Response::Ok {
            message: Some(fl!("architecture-ipc-server-debug_string_pong")),
        },
        DebugRequest::Inspector => {
            gtk4::Window::set_interactive_debugging(true);
            Response::Ok { message: None }
        }
    }
}

pub fn handle_wallpaper(
    request: WallpaperRequest,
    mut theme_manager: RefMut<'_, ThemeManager>,
) -> Response {
    match request {
        WallpaperRequest::Set { path } => match theme_manager.update_wallpaper(path) {
            Ok(res) => Response::Ok {
                message: Some("TODO: i18n".to_string()),
            },
            Err(e) => Response::Error {
                message: Some("TODO: i18n".to_string()),
            },
        },
        WallpaperRequest::Default => Response::Error {
            message: Some("Not Implemented".to_string()),
        },
        WallpaperRequest::Show => Response::Error {
            message: Some("Not Implemented".to_string()),
        },
    }
}

pub fn handle_scheme(
    request: SchemeRequest,
    mut theme_manager: RefMut<'_, ThemeManager>,
) -> Response {
    match request {
        SchemeRequest::Set { scheme } => match SchemesEnum::try_from(scheme) {
            Ok(scheme) => match theme_manager.update_scheme(scheme) {
                Ok(res) => Response::Ok {
                    message: Some("TODO: i18n".to_string()),
                },
                Err(e) => Response::Error {
                    message: Some("TODO: i18n".to_string()),
                },
            },
            Err(e) => Response::Error {
                message: Some("TODO: i18n".to_string()),
            },
        },
        SchemeRequest::Default => Response::Error {
            message: Some("Not Implemented".to_string()),
        },
        SchemeRequest::Show => Response::Error {
            message: Some("Not Implemented".to_string()),
        },
    }
}
