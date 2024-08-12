
use axum::{
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, Multipart, Query, Request},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use librsvg_rebind::prelude::*;
use rustc_version_runtime::version;
use serde::{Deserialize, Serialize};
use tower::util::ServiceExt;
use tower_http::{limit::RequestBodyLimitLayer, services::ServeFile};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route_service(
            "/",
            get(|request: Request| async {
                let service = ServeFile::new("static/index.html");
                let result = service.oneshot(request).await;
                result
            })
            .post(process_upload),
        )
        .route_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .route_service("/favicon.svg", ServeFile::new("static/favicon.svg"))
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        .route_service("/status.json", get(get_status))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024 /* 10mb */));

    // run our app with hyper, listening globally on port 3000

    // get address from environment variable
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

    // get port from environment variable
    let port = std::env::var("PORT").unwrap_or_else(|_| "4000".to_string());

    let listen = format!("{}:{}", address, port);

    let listener = tokio::net::TcpListener::bind(listen).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct StatusParams {
    callback: Option<String>,
}

#[derive(Serialize)]
struct StatusInfo {
    success: bool,
    message: String,
    tech: String,
    timestamp: String,
    lastmod: String,
    commit: String,
}

async fn get_status(Query(params): Query<StatusParams>) -> Response {
    let tech = format!("Rust {}", version());
    let timestamp = chrono::Utc::now().to_rfc3339();
    let lastmod = std::env::var("LASTMOD").unwrap_or_else(|_| "(local)".to_string());
    let commit = std::env::var("COMMIT").unwrap_or_else(|_| "(local)".to_string());

    let status = StatusInfo {
        success: true,
        message: "OK".to_string(),
        tech: tech.to_string(),
        timestamp: timestamp.to_string(),
        lastmod: lastmod.to_string(),
        commit: commit.to_string(),
    };

    if params.callback.is_some() {
        let jsonp = format!(
            "{}({})",
            params.callback.unwrap(),
            serde_json::to_string(&status).unwrap()
        );
        return jsonp.into_response();
    }
    return Json(status).into_response();
}

async fn process_upload(mut multipart: Multipart) -> Response {
    while let Some(field) = multipart.next_field().await.unwrap() {
        //let name = field.name().unwrap().to_string();
        //let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let response = Response::builder()
            .header("Content-Type", "text/html")
            .body(Body::from(process_bytes(content_type, data)))
            //.body(Body::from_stream(process_bytes(data)))
            .unwrap();
        return response;

        //return Body::from_stream(process_bytes(data));
    }
    return "No file uploaded".into_response();
}


fn process_bytes (content_type:String, data: Bytes) -> String {
    let mut buf = String::with_capacity(20 * 1024);

    buf.push_str(ABOVE);
    buf.push_str(format!("Content-Type    : {}\n", content_type).as_str());
    buf.push_str(format!("Upload size     : {} bytes\n", data.len()).as_str());
    buf.push_str(format!("Image           : <img class=\"preview\" src=\"{}\" alt=\"original image\" />\n", make_data_url(content_type, &data)).as_str());

    let sizes: [i32; 4] = [16, 32, 64, 128];
    let mut pngs: Vec<Bytes> = vec![];

    for (_i, size) in sizes.iter().enumerate() {
        let png = render_png(size, &data);
        buf.push_str(format!("Generating      : {}x{} image\n", size, size).as_str());
        buf.push_str(format!("Image           : <img class=\"preview\" src=\"{}\" alt=\"original image\" />\n", make_data_url("image/png".to_string(), &png)).as_str());
        pngs.push(png);
    }

    buf.push_str(format!("Generating      : ICO file\n").as_str());
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for (_i, png) in pngs.iter().enumerate() {
        let mut reader = std::io::Cursor::new(&png);
        let image = ico::IconImage::read_png(&mut reader).unwrap();
        icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
    }
    let mut ico = Vec::new();
    icon_dir.write(&mut ico).unwrap();
    let ico_bytes = Bytes::from(ico);

    buf.push_str(format!("Icon            : <img class=\"preview\" src=\"{}\" alt=\"original image\" />\n", make_data_url("image/ico".to_string(), &ico_bytes)).as_str());
    buf.push_str(format!("                  <a href=\"{}\" download=\"favicon.ico\">Download</a>\n", make_data_url("image/ico".to_string(), &ico_bytes)).as_str());

    buf.push_str("Complete!\n");
    buf.push_str("<a href=\"/\">Make another</a>");

    buf.push_str(BELOW);

    return buf;
}

fn make_data_url(content_type: String, data: &Bytes) -> String {
    let mut buf = String::with_capacity(20 * 1024);

    buf.push_str("data:");
    buf.push_str(content_type.as_str());
    buf.push_str(";base64,");
    buf.push_str(&BASE64.encode(data));

    return buf;
}

const ABOVE: &str = "<html><head><style>img.preview {max-width:128px;max-height:128px;vertical-align:top;border:1px solid black;background-color:eee; }</style><title>Result</title></head><body><pre>";
const BELOW: &str = "</pre></body></html>";

fn render_png(imgsize:&i32, data:&Bytes) -> Bytes {

    let handle = librsvg_rebind::Handle::from_data(&data)
        .unwrap()
        .unwrap();

    let surface =
        cairo::ImageSurface::create(cairo::Format::ARgb32, *imgsize as i32, *imgsize as i32).unwrap();
    let context = cairo::Context::new(&surface).unwrap();

    let viewport = librsvg_rebind::Rectangle::new(0., 0., *imgsize as f64, *imgsize as f64);

    handle.render_document(&context, &viewport).unwrap();

    let mut output_file = Vec::new();

    //let mut output_file = std::fs::File::create("/dev/null").unwrap();
    surface.write_to_png(&mut output_file).unwrap();

    return Bytes::from(output_file);
}