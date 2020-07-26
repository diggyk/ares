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
        q -> Int4,
        r -> Int4,
    }
}

table! {
    robot_modules (robot_id) {
        robot_id -> Int8,
        m_collector -> Varchar,
        m_drivesystem -> Varchar,
        m_exfilbeacon -> Varchar,
        m_hull -> Varchar,
        m_memory -> Varchar,
        m_power -> Varchar,
        m_scanner -> Varchar,
        m_weapons -> Varchar,
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
        power -> Int4,
        max_power -> Int4,
        recharge_rate -> Int4,
        hull_strength -> Int4,
        max_hull_strength -> Int4,
        mined_amount -> Int4,
        val_inventory -> Int4,
        max_val_inventory -> Int4,
        exfil_countdown -> Int4,
        hibernate_countdown -> Int4,
        status_text -> Varchar,
    }
}

table! {
    valuables (id) {
        id -> Int8,
        q -> Int4,
        r -> Int4,
        kind -> Varchar,
        amount -> Int4,
    }
}

joinable!(robot_known_cells -> gridcells (gridcell_id));
joinable!(robot_known_cells -> robots (robot_id));
joinable!(robot_modules -> robots (robot_id));

allow_tables_to_appear_in_same_query!(
    gridcells,
    robot_known_cells,
    robot_modules,
    robots,
    valuables,
);
