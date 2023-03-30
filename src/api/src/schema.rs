// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
    }
}

diesel::table! {
    products (id) {
        id -> Char,
        store_id -> Nullable<Char>,
        seller_id -> Nullable<Char>,
        category_id -> Nullable<Integer>,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        price -> Nullable<Integer>,
        active -> Nullable<Bool>,
    }
}

diesel::table! {
    stores (id) {
        id -> Char,
        owner_id -> Nullable<Char>,
        description -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        status -> Nullable<Bool>,
        location -> Nullable<Varchar>,
        active -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Char,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        cpf -> Nullable<Unsigned<Bigint>>,
        description -> Nullable<Varchar>,
        phone_number -> Nullable<Unsigned<Bigint>>,
        is_admin -> Nullable<Bool>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::joinable!(products -> categories (category_id));
diesel::joinable!(products -> stores (store_id));
diesel::joinable!(products -> users (seller_id));
diesel::joinable!(stores -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    products,
    stores,
    users,
);
