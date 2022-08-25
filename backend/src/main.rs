pub mod get_req;
pub mod login;
pub mod post_req;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_req::hello)
            .service(post_req::echo)
            .service(get_req::get_message)
            .service(post_req::send_messsage)
            .service(login::login)
            .route("/hey", web::get().to(get_req::manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
