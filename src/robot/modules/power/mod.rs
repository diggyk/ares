pub struct PowerModule {}

impl PowerModule {
    pub fn get_max_power(name: &str) -> i32 {
        match name {
            "basic" => 1000,
            "plus" => 1500,
            "foxline" => 3000,
            _ => 1000,
        }
    }

    pub fn get_recharge_rate(name: &str) -> i32 {
        match name {
            "basic" => 150,
            "plus" => 300,
            "foxline" => 1000,
            _ => 150,
        }
    }
}
