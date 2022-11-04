// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> Integer,
        completed -> Bool,
        message -> Text,
    }
}
