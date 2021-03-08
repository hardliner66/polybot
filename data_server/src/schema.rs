table! {
    channel_points (streamer_id, viewer_id) {
        streamer_id -> Int4,
        viewer_id -> Int4,
        points -> Nullable<Int4>,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    channel_points,
    users,
);
