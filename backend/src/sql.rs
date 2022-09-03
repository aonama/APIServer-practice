use diesel::prelude::*;
use crate::models::NewUser;
use crate::schema::users as users_schema;
use crate::utils::establish_connection;

pub fn insert_user() {
    let mut connection = establish_connection();
    let new_user = NewUser {
        name: String::from("new_user"),
    };
    diesel::insert_into(users_schema::dsl::users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");
    println!("INSERT successed!");
}
