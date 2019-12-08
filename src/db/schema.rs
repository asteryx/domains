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
    domains_status (id) {
        id -> Integer,
        date -> Text,
        loading_time -> Integer,
        headers -> Text,
        domain_id -> Integer,
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
joinable!(domains_status -> domains (domain_id));

allow_tables_to_appear_in_same_query!(
    domains,
    domains_status,
    users,
);
