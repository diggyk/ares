use rand::seq::SliceRandom;

use crate::grid::Coords;
use crate::grid::Dir;
use crate::utils::get_bearing;

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

    pub fn get_fov(_name: &str) -> i32 {
        match _name {
            "none" => 0,
            "blaster" => 0,
            "supreme_blaster" => 0,
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

    /// see if a target @ coords2 is in range of the weapon (_name) given bearing
    /// from coords1 facing dir
    pub fn in_range(_name: &str, coords1: &Coords, dir: &Dir, coords2: &Coords) -> bool {
        let fov = Self::get_fov(_name);
        let range = Self::get_range(_name);

        // out of range
        if coords1.distance_to(coords2) > range {
            println!("Too far");
            return false;
        }

        let bearing = get_bearing(dir, coords1, coords2);
        if bearing.is_none() {
            println!("Can't find bearings");
            return false;
        }

        // can only shoot straight and bearing is not straight ahead
        if fov == 0 && bearing.unwrap() != 0 {
            println!("from: {:?} facing {:?} to: {:?}", coords1, dir, coords2);
            println!("Bearing mismatch: {}", bearing.unwrap());
            return false;
        } else if bearing.unwrap().abs() <= fov / 2 {
            return true;
        } else {
            println!("Some other issue");
            return false;
        }
    }
}
