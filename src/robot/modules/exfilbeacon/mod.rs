pub struct ExfilBeaconModule {}

impl ExfilBeaconModule {
    pub fn get_delay(_name: &str) -> i32 {
        5
    }

    pub fn get_power_usage(_name: &str) -> i32 {
        1000
    }
}
