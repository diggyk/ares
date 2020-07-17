pub mod power;

pub trait CollectorModule {
    fn get_name(&self) -> String;
    fn get_power_usage(&self) -> i32;

    // how many units can we collect in a tick
    fn get_collection_rate(&self) -> i32;

    fn get_max_collection(&self) -> i32;
}

pub trait DriveSystemModule {
    fn get_name(&self) -> String;
    fn get_steps(&self) -> i32;
    fn get_power_usage(&self) -> i32;
}

pub trait ExfilBeaconModule {
    fn get_name(&self) -> String;
    fn get_delay(&self) -> i32;
    fn get_power_usage(&self) -> i32;
}

pub trait HullModule {
    fn get_name(&self) -> String;
    fn get_max_strength(&self) -> i32;
}

pub trait MemoryModule {
    fn get_name(&self) -> String;
    fn get_memory_size(&self) -> i32;
}

pub trait PowerModule {
    fn get_name(&self) -> String;
    fn get_max_power(&self) -> i32;
    fn get_recharge_rate(&self) -> i32;
}

pub trait ScannerModule {
    fn get_name(&self) -> String;
    fn get_pov(&self) -> i32;
    fn get_range(&self) -> i32;
    fn get_power_usage(&self) -> i32;
    fn get_accuracy(&self) -> i32;
}

pub trait WeaponModule {
    fn get_name(&self) -> String;
    fn get_range(&self) -> i32;
    fn get_max_damage(&self) -> i32;
    fn get_min_damage(&self) -> i32;
    fn get_cool_down(&self) -> i32;
    fn get_power_usage(&self) -> i32;
}
