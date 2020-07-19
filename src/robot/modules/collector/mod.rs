pub struct CollectorModule {}

impl CollectorModule {
    pub fn get_power_usage(name: &str) -> i32 {
        500
    }

    pub fn get_collection_rate(name: &str) -> i32 {
        10
    }

    pub fn get_collection_max(name: &str) -> i32 {
        100
    }
}
