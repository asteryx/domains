table! {
    domain (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        state -> Integer,
        author -> Integer,
    }
}

table! {
    domain_status (id) {
        id -> Integer,
        date -> Timestamp,
        loading_time -> Integer,
        status_code -> Integer,
        headers -> Text,
        filename -> Text,
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

joinable!(domain -> users (author));
joinable!(domain_status -> domain (domain_id));

allow_tables_to_appear_in_same_query!(
    domain,
    domain_status,
    users,
);
