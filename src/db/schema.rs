table! {
    users (id) {
        id -> Text,
        email -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        password_hash -> Text,
        email_verified -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_suspended -> Bool,
        is_deleted -> Bool,
        user_role -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
);
