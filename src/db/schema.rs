table! {
    domains (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        status -> Integer,
        author -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        name -> Text,
    }
}

joinable!(domains -> users (author));

allow_tables_to_appear_in_same_query!(
    domains,
    users,
);
