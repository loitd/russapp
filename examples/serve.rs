// A serve example to run actix web based on: https://github.com/actix/examples/blob/master/basics/src/main.rs
// To run: cargo run --example serve
// To build release: cargo build --example serve --release
// Copy the templates & static folder with the exe bin file for the web to be runnable.
// Added tera as template & templates dir, logger, static files, 
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, error, get, guard, middleware, Result};
use tera::{Tera, Context};
use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};

// create a data struct to save all the data for the App Context
struct AppData {
    tmpl: Tera
}

// Processing favicon
#[get("/favicon.ico")]
async fn favicon() -> Result<fs::NamedFile>{
    Ok( fs::NamedFile::open( "static/favicon.ico" )? )
}

// handle 404
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("templates/404.html")?.set_status_code(StatusCode::NOT_FOUND))
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
    })
    .bind("127.0.0.1:8081")?
    // .workers(4);
    .run()
    .await
}