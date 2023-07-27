use hyper::http::response;
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => *response.body_mut() = req.into_body(),
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http;//{}", addr);

    server.await?;

    Ok(())
}
