// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        subject -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
    }
}
