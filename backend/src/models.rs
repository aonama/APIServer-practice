use crate::schema::users;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}