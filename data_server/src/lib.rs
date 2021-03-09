#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{NewUser, NewUserData, User, UserData};

pub fn create_user<'a>(conn: &PgConnection, name: &'a str) -> QueryResult<User> {
    use schema::users;

    let new_user = NewUser {
        name: &name.to_lowercase(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user(conn: &PgConnection, user_name: &str) -> QueryResult<User> {
    use self::schema::users::dsl::*;

    users
        .filter(name.eq(&user_name.to_lowercase()))
        .get_result::<User>(conn)
}

pub fn get_user_data(conn: &PgConnection, streamer: &User, viewer: &User) -> QueryResult<UserData> {
    use self::schema::user_data::dsl::*;

    user_data
        .filter(streamer_id.eq(streamer.id))
        .filter(viewer_id.eq(viewer.id))
        .get_result::<UserData>(conn)
}

pub fn create_user_data(
    conn: &PgConnection,
    streamer: &User,
    viewer: &User,
) -> QueryResult<UserData> {
    use self::schema::user_data;

    let new_user_data = NewUserData {
        streamer_id: streamer.id,
        viewer_id: viewer.id,
        points: 0,
    };

    diesel::insert_into(user_data::table)
        .values(&new_user_data)
        .get_result(conn)
}

pub fn set_points(
    conn: &PgConnection,
    streamer: &User,
    viewer: &User,
    new_points: i32,
) -> QueryResult<()> {
    use self::schema::user_data;
    use self::schema::user_data::dsl::*;
    conn.transaction::<_, _, _>(|| {
        if let Err(_) = get_user_data(conn, streamer, viewer) {
            let new_user_data = NewUserData {
                streamer_id: streamer.id,
                viewer_id: viewer.id,
                points: new_points,
            };

            diesel::insert_into(user_data::table)
                .values(&new_user_data)
                .get_result(conn)
        } else {
            diesel::update(user_data.find((streamer.id, viewer.id)))
                .set(points.eq(new_points))
                .get_result::<UserData>(conn)
        }
    })?;

    Ok(())
}

pub fn add_points(
    conn: &PgConnection,
    streamer: &User,
    viewer: &User,
    additional_points: i32,
) -> QueryResult<i32> {
    use self::schema::user_data;
    use self::schema::user_data::dsl::*;
    let data = conn.transaction::<_, _, _>(|| match get_user_data(conn, streamer, viewer) {
        Err(_) => {
            let new_user_data = NewUserData {
                streamer_id: streamer.id,
                viewer_id: viewer.id,
                points: additional_points,
            };

            diesel::insert_into(user_data::table)
                .values(&new_user_data)
                .get_result(conn)
        }
        Ok(data) => diesel::update(user_data.find((streamer.id, viewer.id)))
            .set(points.eq(data.points + additional_points))
            .get_result::<UserData>(conn),
    })?;

    Ok(data.points)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
