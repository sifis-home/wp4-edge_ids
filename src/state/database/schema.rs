// @generated automatically by Diesel CLI.

diesel::table! {
    alarms (id) {
        id -> Integer,
        time -> BigInt,
        message -> Text,
    }
}

diesel::table! {
    configurations (id) {
        id -> Integer,
        config -> Text,
    }
}

diesel::table! {
    data (id) {
        id -> Integer,
        time -> BigInt,
        message -> Text,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Integer,
        config -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(alarms, configurations, data, webhooks,);
