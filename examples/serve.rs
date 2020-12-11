//! A serve example to run actix web based on: https://github.com/actix/examples/blob/master/basics/src/main.rs
//! Form processing based on: https://github.com/actix/examples/blob/master/form/src/main.rs
//! To run: cargo run --example serve
//! To build release: cargo build --example serve --release
//! Copy the templates & static folder with the exe bin file for the web to be runnable.
//! Added tera as template & templates dir, logger, static files, 
//! Added form & serde with App state & configure
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, error, get, guard, middleware, Result};
use tera::{Tera, Context};
use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use serde::{Deserialize, Serialize};

// create a data struct to save all the data for the App Context
struct AppData {
    tmpl: Tera
}

struct AppState {
    foo: String
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    name: String,
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("[Alice, Bob]")
}

async fn render_tmpl(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

// Start the main web app processing
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set env and log
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        //setup tera instance: https://tera.netlify.app/docs/#usage
        // match = C switch: https://doc.rust-lang.org/rust-by-example/flow_control/match.html?highlight=match#match
        let tera = match Tera::new( concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*") ){
            Ok(t) => t,
            Err(e) => {
                println!("Parsing templates errors: {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .data(AppData {tmpl: tera})
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register favicon
            .service(favicon)
            .service(logo)
            .service(uikitcss)
            .service(uikitjs1)
            .service(uikitjs2)
            // register default service
            .default_service(
                // 404
                web::resource("")
                    .route(web::get().to(p404))
                    // enable guard
                    .route(
                        web::route().guard(guard::Not(guard::Get())).to(HttpResponse::MethodNotAllowed),
                    )
            )
            // register normal services
            .route("/", web::get().to(greet))
            .route("/greet/{name}", web::get().to(greet))
            .service(
                web::resource("/users")
                    .route(web::get().to(get_users))
            )
            .service(
                web::resource("/tmpl/{name}")
                    .route(web::get().to(render_tmpl))
            )
            // Use app_config as another way to handle URLs with a fn outside the main. If you place configure before normal inline route, only configure accepted.
            // -> must put configure at the end of the HTTP
            .configure(app_config)
    })
    .bind("127.0.0.1:8081")?
    // .workers(4);
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig){
    config.service(
        web::scope("")
            .data(AppState{
                foo: "bar".to_string(),
            })
            .service(
                web::resource("/post1")
                    .route(web::get().to(render_forms))
                    .route(web::post().to(handle_post1))
            ),
    );
}

async fn render_forms(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {
    // let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    // ctx.insert("name", name);
    let rendered = data.tmpl.render("form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn handle_post1(
    state: web::Data<AppState>,
    params: web::Form<MyParams> 
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!("Yourname is {}. Appstate.foo is {}", params.name, state.foo)))
}

// Processing favicon
#[get("/favicon.ico")]
async fn favicon() -> Result<fs::NamedFile>{
    Ok( fs::NamedFile::open( "static/favicon.ico" )? )
}

// Processing logo
#[get("/logo.png")]
async fn logo() -> Result<fs::NamedFile>{
    Ok( fs::NamedFile::open( "static/logo.png" )? )
}

// Processing uikit css
#[get("/static/css/uikit.min.css")]
async fn uikitcss() -> Result<fs::NamedFile>{
    Ok( fs::NamedFile::open( "static/css/uikit.min.css" )? )
}

// Processing uikit js1
#[get("/static/js/uikit.min.js")]
async fn uikitjs1() -> Result<fs::NamedFile>{
    Ok( fs::NamedFile::open( "static/js/uikit.min.js" )? )
}

// Processing uikit js2
#[get("/static/js/uikit-icons.min.js")]
async fn uikitjs2() -> Result<fs::NamedFile>{
    Ok( fs::NamedFile::open( "static/js/uikit-icons.min.js" )? )
}

// handle 404
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("templates/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().route("/", web::get().to(render_tmpl))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_post() {
        let mut app = test::init_service(App::new().route("/", web::get().to(render_tmpl))).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }
}