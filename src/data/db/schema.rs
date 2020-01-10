table! {
    jumps (id) {
        id -> Integer,
        from -> Text,
        to -> Text,
        create_time -> Timestamp,
    }
}

table! {
    stats (id) {
        id -> Integer,
        ip -> Text,
        jump_from -> Text,
        url -> Text,
        time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    jumps,
    stats,
);
