// @generated automatically by Diesel CLI.

diesel::table! {
    instances (id) {
        id -> Int8,
        name -> Text,
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

diesel::joinable!(metrics -> instances (instance_id));

diesel::allow_tables_to_appear_in_same_query!(
    instances,
    metrics,
);
