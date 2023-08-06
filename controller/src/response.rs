use hyper::{Body, Response};
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct AppResponse<T: Debug + Serialize> {
    data: Option<T>,
    code: u8,
    #[serde(skip_serializing)]
    http_code: u16,
}

impl<T: Serialize + Debug> AppResponse<T> {
    pub fn success(data: T) -> Self {
        AppResponse {
            data: Some(data),
            code: 0,
            http_code: 200,
        }
    }

    // pub fn failed(code: u8, message: String, http_code: u16) -> Self {
    //     AppResponse { data: None, code, message: Some(message), http_code }
    // }

    pub fn not_found() -> Self {
        AppResponse {
            data: None,
            code: 1,
            http_code: 404,
        }
    }

    pub fn internal_server_error() -> Self {
        AppResponse {
            data: None,
            code: 2,
            http_code: 500,
        }
    }

    pub fn invalid_parameters() -> Self {
        AppResponse {
            data: None,
            code: 3,
            http_code: 400,
        }
    }
}

impl From<AppResponse<Vec<u64>>> for Response<Body> {
    fn from(r: AppResponse<Vec<u64>>) -> Self {
        // if cannot convert to string, something is wrong
        let mut r = r;
        let json = match serde_json::to_string(&r) {
            Ok(str) => str,
            Err(_) => {
                log::error!("cannot serialize json response: {:?}", r);
                r = AppResponse::internal_server_error();
                // if this goes wrong, sth is really wrong
                serde_json::to_string(&r).unwrap()
            }
        };
        Response::builder()
            .header("content-type", "application/json")
            .status(r.http_code)
            .body(Body::from(json))
            .unwrap()
    }
}
