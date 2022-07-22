use std::{env, net::Ipv4Addr};
use warp::Filter;

#[tokio::main]
async fn main() {
    let headers = warp::header::headers_cloned();
    let put_blob_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("putblob"))
        .and(warp::multipart::form().max_length(1024 * 1024 * 10240))
        .and(headers)
        .and_then(handler::put_blob);

    let router = put_blob_route.recover(handler::handle_rejection);
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(router).run((Ipv4Addr::LOCALHOST, port)).await
}
