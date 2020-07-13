table! {
    gridcells (id) {
        id -> Int4,
        q -> Int4,
        r -> Int4,
        edge0 -> Int2,
        edge60 -> Int2,
        edge120 -> Int2,
        edge180 -> Int2,
        edge240 -> Int2,
        edge300 -> Int2,
    }
}

table! {
    robots (id) {
        id -> Int8,
        name -> Varchar,
        owner -> Nullable<Int4>,
        affiliation -> Nullable<Int4>,
        q -> Int4,
        r -> Int4,
        orientation -> Int2,
        gridcell -> Nullable<Int4>,
        components -> Nullable<Json>,
        configs -> Nullable<Json>,
    }
}

table! {
    valuables (id) {
        id -> Int8,
        q -> Int4,
        r -> Int4,
        gridcell -> Int4,
        #[sql_name = "type"]
        type_ -> Int2,
        amount -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    gridcells,
    robots,
    valuables,
);
