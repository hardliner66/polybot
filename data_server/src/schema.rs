table! {
    user_data (streamer_id, viewer_id) {
        streamer_id -> Int4,
        viewer_id -> Int4,
        points -> Int8,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    user_data,
    users,
);
