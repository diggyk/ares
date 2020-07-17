pub trait CollectorModule {
    fn get_name() -> String;
    fn get_power_usage() -> i32;

    // how many units can we collect in a tick
    fn get_collection_rate() -> i32;

    fn get_max_collection() -> i32;
}

pub trait DriveSystemModule {
    fn get_name() -> String;
    fn get_steps() -> i32;
    fn get_power_usage() -> i32;
}

pub trait ExfilBeaconModule {
    fn get_name() -> String;
    fn get_delay() -> i32;
    fn get_power_usage() -> i32;
}

pub trait HullModule {
    fn get_name() -> String;
    fn get_max_strength() -> i32;
}

pub trait MemoryModule {
    fn get_name() -> String;
    fn get_memory_size() -> i32;
}

pub trait PowerModule {
    fn get_name() -> String;
    fn get_max_power() -> i32;
    fn get_recharge_rate() -> i32;
}

pub trait ScannerModule {
    fn get_name() -> String;
    fn get_pov() -> i32;
    fn get_range() -> i32;
    fn get_power_usage() -> i32;
    fn get_accuracy() -> i32;
}

pub trait WeaponModule {
    fn get_name() -> String;
    fn get_range() -> i32;
    fn get_max_damage() -> i32;
    fn get_min_damage() -> i32;
    fn get_cool_down() -> i32;
    fn get_power_usage() -> i32;
}
