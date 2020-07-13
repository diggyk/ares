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
    robot_known_cells (robot_id, gridcell_id) {
        robot_id -> Int8,
        gridcell_id -> Int4,
        discovery_time -> Timestamp,
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

joinable!(robot_known_cells -> gridcells (gridcell_id));
joinable!(robot_known_cells -> robots (robot_id));

allow_tables_to_appear_in_same_query!(
    gridcells,
    robot_known_cells,
    robots,
    valuables,
);
