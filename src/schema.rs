table! {
    students (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        class -> Text,
        phone_number -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        login -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    students,
    users,
);
