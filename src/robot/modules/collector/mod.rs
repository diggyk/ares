pub struct CollectorModule {}

impl CollectorModule {
    pub fn get_power_usage(_name: &str) -> i32 {
        500
    }

    pub fn get_collection_rate(_name: &str) -> i32 {
        10
    }

    pub fn get_collection_max(_name: &str) -> i32 {
        100
    }
}
