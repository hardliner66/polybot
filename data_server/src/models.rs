use super::schema::{users, channel_points};

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="channel_points"]
pub struct NewChannelPoints {
    pub streamer_id: i32,
    pub viewer_id: i32,
}

#[derive(Queryable)]
pub struct ChannelPoints {
    pub streamer_id: i32,
    pub viewer_id: i32,
    pub points: i32,
}

