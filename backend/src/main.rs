pub mod get_req;
pub mod post_req;

use actix_web::{web, App, HttpServer, HttpResponse, Error, error};
use tera::Tera;

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let view = 
        tmpl.render("index.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();

        App::new()
            .service(get_req::hello)
            .service(post_req::echo)
            .service(get_req::get_message)
            .service(post_req::send_messsage)
            .route("/hey", web::get().to(get_req::manual_hello))
            .data(templates)
            .service(web::resource("/home").to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
