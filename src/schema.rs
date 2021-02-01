table! {
    login_history (id) {
        id -> Integer,
        user_id -> Integer,
        login_timestamp -> Datetime,
    }
}

table! {
    people (id) {
        id -> Integer,
        name -> Varchar,
        gender -> Bool,
        age -> Integer,
        address -> Varchar,
        phone -> Varchar,
        email -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    login_history,
    people,
    users,
);
