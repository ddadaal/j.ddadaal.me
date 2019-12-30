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
        from -> Text,
        url -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    jumps,
    stats,
);
