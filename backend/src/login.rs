use actix_web::{post, web, HttpResponse, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{remove_file, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub fn remove_old_token(filename: &String) -> std::io::Result<()> {
    let file_exists = Path::new(&filename).exists();

    if !file_exists {
        return Ok(());
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

    let filename_token = format!("authorized/{}.txt", contents);
    println!("{}", filename_token);
    remove_file(filename_token)?;
    Ok(())
}

pub fn write_file(filename: String, contents: &String) {
    let mut file = match File::create(&filename) {
        Err(why) => panic!("Couldn't open {}: {}", &filename, why),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => panic!("Couldn't write \"{}\" to {}: {}", contents, filename, why),
        Ok(_) => println!("write finished"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    password: String,
}

/*
    1. users.jsonからUser型としてデータを読み込む
    2. id, passwordが正しければトークンを生成
    3. トークン->idの対応を表すファイルを authorized/ に保存
    4. 古いトークンは消去し、id->トークンの対応を表すファイルを更新

    リクエストボディの形式
    { "id":"hoge", "password":"fuga" }

    Todo:
    ガバガバセキュリティだからクライアント側の実装をしたら
    もっと堅牢な方式を取る
*/
#[post("/login")]
pub async fn login(req_body: web::Json<User>) -> impl Responder {
    let filename_users = "users.json";
    let file_users = File::open(filename_users).unwrap();
    let reader = BufReader::new(file_users);

    let deserialized: Vec<User> = serde_json::from_reader(reader).unwrap();

    let user_found = deserialized
        .iter()
        .any(|x| x.id == req_body.id && x.password == req_body.password);

    if user_found {
        let mut rng = rand::thread_rng();
        let token: u128 = rng.gen();

        let filename_token = format!("authorized/{}.txt", token);
        let contents_token = &req_body.id;

        write_file(filename_token, contents_token);

        let filename_token_rev = format!("{}.txt", req_body.id);
        let contents_token_rev = &token.to_string();

        remove_old_token(&filename_token_rev).unwrap_or(());
        write_file(filename_token_rev, contents_token_rev);

        return HttpResponse::Ok().body(token.to_string());
    }

    HttpResponse::Ok().body("user not found")
}
