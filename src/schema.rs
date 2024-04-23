// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        firstname -> Varchar,
        #[max_length = 255]
        lastname -> Varchar,
    }
}
