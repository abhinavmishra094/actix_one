table! {
    users (id) {
        id -> Int8,
        uid -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}
