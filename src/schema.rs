// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        published_by_id -> Int4,
        metadata -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 25]
        first_name -> Varchar,
        #[max_length = 25]
        last_name -> Varchar,
        #[max_length = 25]
        email -> Varchar,
        #[max_length = 50]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(posts -> users (published_by_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
