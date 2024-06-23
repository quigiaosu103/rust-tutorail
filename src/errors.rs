use axum::{http::StatusCode, response::{IntoResponse, Response}};



#[derive(Debug)]
pub enum Error {
    LoginFailed,
    DeleteFailIdNotFound {
        id: u32
    }
}
pub  type Result<T> = core::result::Result<T, Error>;


impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED CLIENT ERROR").into_response()
            },
            Error::DeleteFailIdNotFound {id} => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("DELETE FAIL ID {id} NOT FOUND")).into_response()
            }
        }
    }
}

