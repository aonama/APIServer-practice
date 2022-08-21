use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/get_message")]
async fn get_message() -> impl Responder {
    let filename = "log.txt";
    
    let mut file = match File::open(filename) {
        Err(_why) => File::create(filename).unwrap(),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", filename, why),
        Ok(_) => (),
    }

    HttpResponse::Ok().body(contents)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/send_message")]
async fn send_messsage(req_body: String) -> impl Responder {
    let filename = "log.txt";

    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)
    {
        Err(why) => panic!("Couldn't open {}: {}", filename, why),
        Ok(file) => file,
    };

    let dt = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let contents = format!("{0} {1}\n", dt, req_body);

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write \"{}\" to {}: {}", contents, filename, why),
        Ok(_) => println!("finished"),
    }

    HttpResponse::Ok().body("post successed!\n")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_message)
            .service(send_messsage)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}