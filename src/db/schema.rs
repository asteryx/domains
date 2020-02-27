table! {
    domain (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        state -> Int4,
        author -> Int4,
    }
}

table! {
    domain_status (id) {
        id -> Int4,
        date -> Timestamp,
        loading_time -> Int4,
        status_code -> Int4,
        headers -> Varchar,
        filename -> Nullable<Varchar>,
        domain_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
    }
}

joinable!(domain -> users (author));
joinable!(domain_status -> domain (domain_id));

allow_tables_to_appear_in_same_query!(
    domain,
    domain_status,
    users,
);
