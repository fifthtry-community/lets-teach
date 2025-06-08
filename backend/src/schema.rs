diesel::table! {
    lets_teach_user_concept (id) {
        id -> BigInt,
        user_id -> BigInt,
        concept_url -> Text,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}


diesel::table! {
    lets_teach_user_concept_event (id) {
        id -> BigInt,
        user_id -> BigInt,
        concept_url -> Text,
        event_type -> Text,
        event_data -> Text,
        created_at -> Timestamptz,
    }
}
