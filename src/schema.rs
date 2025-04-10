// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 250]
        name -> Varchar,
        #[max_length = 250]
        code -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 250]
        name -> Varchar,
        #[max_length = 250]
        email -> Varchar,
        password -> Text,
        role_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
