#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};

type HttpClient = Client<hyper::client::HttpConnector>;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));

    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(_client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // println!("req: {:?}", req);

    let headers = req.headers().clone();
    println!("headers: {:?}", headers);


    let path = req.uri().path().to_string();
    if path.starts_with("/hello") {
        let target_url = "http://127.0.0.1:8000".to_owned();
        let resp = get_response(_client, req, &target_url, &path).await?;
        return Ok(resp);
    }

    let resp = Response::new(Body::from("sorry! no route found"));
    Ok(resp)
}

async fn get_response(client: HttpClient, req: Request<Body>, target_url: &str, path: &str) -> Result<Response<Body>, hyper::Error> {
    let target_url = format!("{}{}", target_url, path);
    let headers = req.headers().clone();
    let mut request_builder = Request::builder()
        .method(req.method())
        .uri(target_url)
        .body(req.into_body())
        .unwrap();

    *request_builder.headers_mut() = headers;
    let response = client.request(request_builder).await?;
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec()).unwrap();

    let mut resp = Response::new(Body::from(body));
    *resp.status_mut() = http::StatusCode::OK;
    Ok(resp)
}
