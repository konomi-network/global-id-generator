use std::collections::HashMap;
use crate::response::AppResponse;
use hyper::service::Service;
use hyper::{http, Body, Request, Response, Uri};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::{mpsc, oneshot};

pub struct Svc {
    sender: Arc<mpsc::Sender<service::id::Request>>,
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let sender = Arc::clone(&self.sender);
        Box::pin(async move {
            match (req.uri().path(), req.method()) {
                ("/id", &http::method::Method::POST) => {
                    if let Some((sharding_id, num)) = parse_query_str(req.uri()) {
                        let (tx, rx) = oneshot::channel();
                        let r = service::id::Request::NewKey { sharding_id, num, tx };

                        match sender.send(r).await {
                            Err(e) => {
                                log::error!("cannot send: {:}", e);
                                Ok(AppResponse::internal_server_error().into())
                            }
                            _ => match rx.await {
                                Ok(id) => Ok(AppResponse::success(id).into()),
                                Err(e) => {
                                    log::error!("cannot receive: {:}", e);
                                    Ok(AppResponse::internal_server_error().into())
                                }
                            },
                        }
                    } else {
                        Ok(AppResponse::invalid_parameters().into())
                    }
                }
                _ => Ok(AppResponse::not_found().into()),
            }
        })
    }
}

pub struct MakeSvc {
    sender: Arc<mpsc::Sender<service::id::Request>>,
}

impl MakeSvc {
    pub fn new(sender: Arc<mpsc::Sender<service::id::Request>>) -> Self {
        MakeSvc { sender }
    }
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let sender = Arc::clone(&self.sender);
        let fut = async move { Ok(Svc { sender }) };
        Box::pin(fut)
    }
}

fn parse_query_str(query_str: &Uri) -> Option<(u64, usize)> {
    let params = query_str_to_map(query_str);
    let sharding_id = match params.get("sharding_id").map(|u| u64::from_str(u)) {
        None => return None,
        Some(u) => match u {
            Ok(r) => r,
            Err(_) => return None
        }
    };
    let num = parse_num(&params);
    Some((sharding_id, num))
}

fn query_str_to_map(query_str: &Uri) -> HashMap<String, String> {
    query_str
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new)
}

fn parse_num(params: &HashMap<String, String>) -> usize {
    let num = match params.get("num").map(|u| usize::from_str(u)) {
        None => 1, // default to 1
        Some(u) => match u {
            Ok(r) => r,
            Err(_) => 1
        }
    };
    num
}
