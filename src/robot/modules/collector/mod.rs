use rand::seq::SliceRandom;

pub struct CollectorModule {}

impl CollectorModule {
    pub fn get_random() -> String {
        let list = vec!["basic", "foxterra", "ultratech"];

        let mut rng = rand::thread_rng();

        list.choose(&mut rng).unwrap().to_string()
    }

    pub fn get_power_usage(_name: &str) -> i32 {
        match _name {
            "basic" => 500,
            "foxterra" => 1000,
            "ultratech" => 1500,
            _ => 500,
        }
    }

    pub fn get_collection_rate(_name: &str) -> i32 {
        match _name {
            "basic" => 10,
            "foxterra" => 25,
            "ultratech" => 50,
            _ => 10,
        }
    }

    pub fn get_collection_max(_name: &str) -> i32 {
        match _name {
            "basic" => 200,
            "foxterra" => 500,
            "ultratech" => 1000,
            _ => 200,
        }
    }
}
