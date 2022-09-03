pub mod get_req;
pub mod post_req;
pub mod models;
pub mod schema;
pub mod utils;
pub mod sql;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use tera::Tera;

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let ctx = tera::Context::new();
    let view = tmpl
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // とりあえずすべてのオリジンからのリクエストを許可
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        let templates = Tera::new("templates/**/*").unwrap();

        App::new()
            .wrap(cors)
            .service(get_req::hello)
            .service(post_req::echo)
            .service(get_req::get_message)
            .service(post_req::send_messsage)
            .route("/hey", web::get().to(get_req::manual_hello))
            .app_data(templates)
            .service(web::resource("/home").to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
