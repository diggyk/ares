use super::*;

#[derive(Debug)]
enum PowerModules {
    Basic,
    Large,
    FastCharge,
}

impl PowerModules {
    pub fn to_module(name: &str) -> Box<dyn PowerModule> {
        match name {
            "basic" => Box::new(BasicPowerModule {}),
            "large" => Box::new(LargePowerModule {}),
            "fastcharge" => Box::new(FastChargePowerModule {}),
            &_ => Box::new(BasicPowerModule {}),
        }
    }
}

pub struct BasicPowerModule {}

impl PowerModule for BasicPowerModule {
    fn get_name(&self) -> String {
        String::from("basic")
    }

    fn get_max_power(&self) -> i32 {
        1000
    }

    fn get_recharge_rate(&self) -> i32 {
        200
    }
}

pub struct LargePowerModule {}

impl PowerModule for LargePowerModule {
    fn get_name(&self) -> String {
        String::from("large")
    }

    fn get_max_power(&self) -> i32 {
        10000
    }

    fn get_recharge_rate(&self) -> i32 {
        500
    }
}

pub struct FastChargePowerModule {}

impl PowerModule for FastChargePowerModule {
    fn get_name(&self) -> String {
        String::from("fastcharge")
    }

    fn get_max_power(&self) -> i32 {
        3000
    }

    fn get_recharge_rate(&self) -> i32 {
        1000
    }
}
