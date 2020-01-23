
table! {
    characters (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    paths (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    characters,
    paths,
);
