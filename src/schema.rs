// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Integer,
        owner -> Text,
        device_name -> Text,
        device_type -> Text,
        device_token -> Text,
        os_version -> Text,
        enabled -> Bool,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(devices -> users (owner));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    users,
);
