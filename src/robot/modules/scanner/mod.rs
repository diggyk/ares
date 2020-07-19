pub struct ScannerModule {}

impl ScannerModule {
    fn get_pov(name: &str) -> i32 {
        match name {
            "basic" => 120,
            _ => 120,
        }
    }

    fn get_range(name: &str) -> i32 {
        match name {
            "basic" => 1,
            _ => 1,
        }
    }

    fn get_power_usage(name: &str) -> i32 {
        match name {
            "basic" => 50,
            _ => 50,
        }
    }

    fn get_accuracy(name: &str) -> i32 {
        match name {
            "basic" => 100,
            _ => 100,
        }
    }
}
