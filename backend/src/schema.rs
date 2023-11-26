// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user,
);
