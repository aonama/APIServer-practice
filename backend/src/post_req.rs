use actix_web::{post, web, HttpResponse, Responder};
use chrono::Local;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ReqBody {
    message: String,
    token: u128,
}

pub async fn check_login(token: u128) -> String {
    let filename = format!("authorized/{}.txt", token);
    let file_exists = Path::new(&filename).exists();

    if !file_exists {
        return "".to_string();
    }

    let mut file = match File::open(&filename) {
        Err(why) => panic!("Couldn't read {}: {}", &filename, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", &filename, why),
        Ok(_) => (),
    }

    contents
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/*
    リクエストボディの形式
    { "message":"hoge", "token":"fuga" }
*/
#[post("/send_message")]
pub async fn send_messsage(req_body: web::Json<ReqBody>) -> impl Responder {
    let user_id = check_login(req_body.token).await;

    if user_id.is_empty() {
        return HttpResponse::Ok().body("please login!!\n");
    }

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

    let dt = Local::now().format("%Y/%m/%d %H:%M:%S\n").to_string();
    let contents = format!("{1}{0} {2}\n", &user_id, dt, req_body.message);

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write \"{}\" to {}: {}", contents, filename, why),
        Ok(_) => println!("finished"),
    }

    HttpResponse::Ok().body("post successed!\n")
}
