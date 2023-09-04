// @generated automatically by Diesel CLI.

diesel::table! {
    access_rules (id) {
        id -> Int8,
        user_id -> Int8,
        use_case_id -> Int8,
        resource_type -> Text,
        resource_id -> Nullable<Int8>,
    }
}

diesel::table! {
    instances (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    jobs (name) {
        name -> Text,
        last_invocation -> Timestamp,
        sleep_time -> Interval,
    }
}

diesel::table! {
    metrics (id) {
        id -> Int8,
        instance_id -> Int8,
        metric_type -> Text,
        creating_date -> Timestamp,
        metric_value -> Json,
    }
}

diesel::table! {
    use_cases (id) {
        id -> Int8,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Text,
        email -> Text,
        salt -> Text,
        salted_password -> Text,
    }
}

diesel::joinable!(access_rules -> use_cases (use_case_id));
diesel::joinable!(access_rules -> users (user_id));
diesel::joinable!(metrics -> instances (instance_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_rules,
    instances,
    jobs,
    metrics,
    use_cases,
    users,
);
