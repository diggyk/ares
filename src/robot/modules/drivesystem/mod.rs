pub struct DriveSystemModule {}

impl DriveSystemModule {
    pub fn get_steps(_name: &str) -> i32 {
        1
    }

    pub fn get_power_usage(_name: &str) -> i32 {
        100
    }
}
