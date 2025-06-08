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

#[derive(diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = backend::schema::lets_teach_user_concept)]
#[diesel(check_for_backend(ft_sdk::Sqlite))]
pub struct UC {
    pub id: i64,
    pub user_id: i64,
    pub concept_url: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
