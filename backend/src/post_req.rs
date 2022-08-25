use actix_web::{post, HttpResponse, Responder};
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/send_message")]
pub async fn send_messsage(req_body: String) -> impl Responder {
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
