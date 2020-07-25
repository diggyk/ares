use rand::seq::SliceRandom;

pub struct WeaponModule {}

impl WeaponModule {
    pub fn get_random() -> String {
        let list = vec!["none", "blaster", "supreme_blaster"];

        let mut rng = rand::thread_rng();

        list.choose(&mut rng).unwrap().to_string()
    }

    pub fn get_range(_name: &str) -> i32 {
        match _name {
            "none" => 0,
            "blaster" => 1,
            "supreme_blaster" => 2,
            _ => 0,
        }
    }
    pub fn get_max_damage(_name: &str) -> i32 {
        match _name {
            "none" => 0,
            "blaster" => 250,
            "supreme_blaster" => 500,
            _ => 0,
        }
    }

    pub fn get_min_damage(_name: &str) -> i32 {
        match _name {
            "none" => 0,
            "blaster" => 100,
            "supreme_blaster" => 250,
            _ => 0,
        }
    }

    pub fn get_cool_down(_name: &str) -> i32 {
        match _name {
            "none" => 0,
            "blaster" => 0,
            "supreme_blaster" => 0,
            _ => 0,
        }
    }

    pub fn get_power_usage(_name: &str) -> i32 {
        match _name {
            "none" => 0,
            "blaster" => 500,
            "supreme_blaster" => 1000,
            _ => 0,
        }
    }
}
