// @generated automatically by Diesel CLI.

diesel::table! {
    expenses (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        amount -> Int8,
        date -> Timestamp,
    }
}

diesel::table! {
    payments (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        amount -> Int8,
        name -> Text,
        email -> Text,
        payment_type -> Text,
        date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(expenses, payments,);
