use rand::seq::SliceRandom;

pub struct ScannerModule {}

impl ScannerModule {
    pub fn get_random() -> String {
        let list = vec![
            "basic",
            "plus",
            "triscan",
            "triscan_advanced",
            "triscan_ultra",
            "boxium_starter",
            "boxium_advanced",
            "boxium_ultra",
            "omni_basic",
            "omni_ultra",
        ];

        let mut rng = rand::thread_rng();

        list.choose(&mut rng).unwrap().to_string()
    }

    pub fn get_fov(name: &str) -> i32 {
        match name {
            "basic" => 0,
            "plus" => 0,
            "triscan" => 120,
            "triscan_advanced" => 120,
            "triscan_ultra" => 120,
            "boxium_starter" => 240,
            "boxium_advanced" => 240,
            "boxium_ultra" => 240,
            "omni_basic" => 360,
            "omni_ultra" => 360,
            _ => 0,
        }
    }

    pub fn get_range(name: &str) -> i32 {
        match name {
            "basic" => 1,
            "plus" => 2,
            "triscan" => 1,
            "triscan_advanced" => 2,
            "triscan_ultra" => 3,
            "boxium_starter" => 1,
            "boxium_advanced" => 2,
            "boxium_ultra" => 3,
            "omni_basic" => 2,
            "omni_ultra" => 4,
            _ => 0,
        }
    }

    pub fn get_power_usage(name: &str) -> i32 {
        match name {
            "basic" => 20,
            "plus" => 30,
            "triscan" => 60,
            "triscan_advanced" => 80,
            "triscan_ultra" => 120,
            "boxium_starter" => 250,
            "boxium_advanced" => 350,
            "boxium_ultra" => 1000,
            "omni_basic" => 500,
            "omni_ultra" => 2000,
            _ => 0,
        }
    }

    pub fn get_accuracy(name: &str) -> i32 {
        match name {
            "basic" => 100,
            "plus" => 100,
            "triscan" => 75,
            "triscan_advanced" => 75,
            "triscan_ultra" => 50,
            "boxium_starter" => 80,
            "boxium_advanced" => 80,
            "boxium_ultra" => 80,
            "omni_basic" => 0,
            "omni_ultra" => 0,
            _ => 0,
        }
    }
}
