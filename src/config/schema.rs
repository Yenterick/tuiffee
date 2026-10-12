// @generated automatically by Diesel CLI.

diesel::table! {
    blooming (id) {
        id -> Nullable<Integer>,
        water_g -> Float,
        time_s -> Integer,
        sequency -> Integer,
        preparation_technique_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    coffee (id) {
        id -> Nullable<Integer>,
        brand -> Text,
        name -> Nullable<Text>,
        roasting_level -> Text,
        process_type -> Text,
        descriptive_notes -> Nullable<Text>,
        location -> Nullable<Text>,
        variety_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    cupping (id) {
        id -> Nullable<Integer>,
        author -> Text,
        aroma -> Nullable<Integer>,
        flavor -> Nullable<Integer>,
        acidity -> Nullable<Integer>,
        sweetness -> Nullable<Integer>,
        body -> Nullable<Integer>,
        aftertaste -> Nullable<Integer>,
        balance -> Nullable<Integer>,
        observations -> Nullable<Text>,
        preparation_record_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    method (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pouring (id) {
        id -> Nullable<Integer>,
        water_g -> Float,
        time_s -> Integer,
        sequency -> Integer,
        preparation_technique_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    preparation_record (id) {
        id -> Nullable<Integer>,
        method_id -> Integer,
        coffee_id -> Integer,
        preparation_technique_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    preparation_technique (id) {
        id -> Nullable<Integer>,
        name -> Text,
        grind_level -> Text,
        ratio -> Integer,
        people_target -> Integer,
        coffee_g -> Float,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    variety (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(blooming -> preparation_technique (preparation_technique_id));
diesel::joinable!(coffee -> variety (variety_id));
diesel::joinable!(cupping -> preparation_record (preparation_record_id));
diesel::joinable!(pouring -> preparation_technique (preparation_technique_id));
diesel::joinable!(preparation_record -> coffee (coffee_id));
diesel::joinable!(preparation_record -> method (method_id));
diesel::joinable!(preparation_record -> preparation_technique (preparation_technique_id));

diesel::allow_tables_to_appear_in_same_query!(
    blooming,
    coffee,
    cupping,
    method,
    pouring,
    preparation_record,
    preparation_technique,
    variety,
);
