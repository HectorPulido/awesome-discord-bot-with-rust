table! {
    channels (id) {
        id -> Int4,
        channel_id -> Varchar,
        channel_type -> Int4,
    }
}

table! {
    resources (id) {
        id -> Int4,
        user_id -> Varchar,
        channel_id -> Varchar,
        url -> Varchar,
        description -> Text,
        resources_type -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    types (id) {
        id -> Int4,
        type_description -> Varchar,
    }
}

joinable!(channels -> types (channel_type));
joinable!(resources -> types (resources_type));

allow_tables_to_appear_in_same_query!(
    channels,
    resources,
    types,
);
