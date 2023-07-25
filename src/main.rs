use std::convert::Infallible;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello World!".into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| 
        async {Ok::<_, Infallible>(service_fn(handle_request))}
    );

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http;//{}", addr);

    server.await?;

    Ok(())
}
