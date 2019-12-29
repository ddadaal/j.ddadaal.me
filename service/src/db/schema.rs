table! {
    links (id) {
        id -> Integer,
        short_link -> Text,
        full_link -> Text,
        create_time -> Timestamp,
    }
}

table! {
    stats (id) {
        id -> Integer,
        ip -> Text,
        short_link -> Text,
        access_url -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    links,
    stats,
);
