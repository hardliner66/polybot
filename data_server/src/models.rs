use super::schema::{user_data, users};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "user_data"]
pub struct NewUserData {
    pub streamer_id: i32,
    pub viewer_id: i32,
    pub points: i32,
}

#[derive(Queryable)]
pub struct UserData {
    pub streamer_id: i32,
    pub viewer_id: i32,
    pub points: i32,
}
