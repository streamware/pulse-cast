// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Int4,
        owner -> Int4,
        device_name -> Text,
        device_type -> Text,
        device_token -> Text,
        os_version -> Text,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(devices -> users (owner));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    users,
);
