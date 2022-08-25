use actix_web::{get, HttpResponse, Responder};
use std::fs::File;
use std::io::Read;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/get_message")]
pub async fn get_message() -> impl Responder {
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

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
