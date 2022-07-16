use bytes::BufMut;
use futures::stream::TryStreamExt;
use reqwest::header::HeaderMap;
use std::{convert::Infallible, env, net::Ipv4Addr};
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};

#[tokio::main]
async fn main() {
    let headers = warp::header::headers_cloned();
    let upload_route = warp::post()
        .and(warp::path("api"))
        .and(warp::path("uploadfile"))
        .and(warp::multipart::form().max_length(1024 * 1024 * 1000))
        .and(headers)
        .and_then(upload);

    let router = upload_route.recover(handle_rejection);
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(router).run((Ipv4Addr::LOCALHOST, port)).await
}

async fn upload(form: FormData, headers: HeaderMap) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    let x_ms_blob_account = headers.get("x-ms-blob-account").unwrap();
    let x_ms_blob_sv = headers.get("x-ms-blob-sv").unwrap();
    let x_ms_blob_container = headers.get("x-ms-blob-container").unwrap();

    for p in parts {
        let url = format!(
            "https://{}.blob.core.windows.net/{}/{}{}",
            x_ms_blob_account.to_str().unwrap(),
            x_ms_blob_container.to_str().unwrap(),
            p.filename().unwrap(),
            x_ms_blob_sv.to_str().unwrap()
        );

        let value = p
            .stream()
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await
            .map_err(|e| {
                eprintln!("reading file error: {}", e);
                warp::reject::reject()
            })?;

        let part = reqwest::multipart::Part::bytes(value);
        let file = reqwest::multipart::Form::new().part("part_bytes", part);

        let mut headers = HeaderMap::new();
        headers.insert("x-ms-blob-type", "BlockBlob".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        client.put(url).multipart(file).send().await.unwrap();
    }

    Ok("success")
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
