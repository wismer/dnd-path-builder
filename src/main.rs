use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use actix_files as actix_fs;
use std::fs;
use std::path::PathBuf;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[post("/")]
async fn graphql() -> impl Responder {
    format!("ASDKL:SDLF POST")
}

#[get("/")]
async fn home() -> HttpResponse {
    let file = fs::read_to_string("./static/index.html");
    match file {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(&s),
        Err(_) => panic!("DALSKJHDKAJSHD")
    }
}

#[get("/static/js/{filename}")]
async fn js(req: actix_web::HttpRequest) -> HttpResponse {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut full_path = String::new();
    full_path.push_str("./static/js/");
    full_path.push_str(&path.to_str().unwrap());
    let file = fs::read_to_string(&full_path);

    let contents = match file {
        Ok(c) => c,
        Err(e) => panic!("{}, {:?}", e, &path)
    };

    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(&contents)
}

#[get("/static/css/{filename}")]
async fn css(req: actix_web::HttpRequest) -> HttpResponse {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut full_path = String::new();
    full_path.push_str("./static/css/");
    full_path.push_str(&path.to_str().unwrap());
    let file = fs::read_to_string(&full_path);
    
    let contents = match file {
        Ok(c) => c,
        Err(e) => panic!("{} for file: {:?}", e, &path.as_os_str())
    };

    HttpResponse::Ok()
        .content_type("text/css")
        .body(&contents)
}

#[get("/{filename}")]
async fn manifest(req: actix_web::HttpRequest) -> HttpResponse {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut full_path = String::new();
    full_path.push_str("./static/");
    full_path.push_str(&path.to_str().unwrap());
    let file = fs::read_to_string(&full_path);
    let contents = match file {
        Ok(c) => c,
        Err(_) => "".to_owned()
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(&contents)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(graphql)
            .service(home)
            .service(js)
            .service(manifest)
            .service(css)
            .service(
                actix_fs::Files::new("/static", ".")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}