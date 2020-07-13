use crate::utils;
use crate::grid::*;

#[derive(Debug)]
pub struct Robot {
    pub id: i64,
    pub name: String,

    pub coords: Coords,
    pub orientation: Dir,
    pub known: Vec<GridCell>,
}

impl Robot {
    pub fn new(coords: Coords, orientation: Dir) -> Robot {
        Robot {
            id: 0,
            name: utils::random_string(8),
            coords,
            orientation,
            known: Vec::new(),
        }
    }

    fn register(&mut self) {
    }
}

#[cfg(test)]
#[test]
fn basic_robot_new() {
    let coords = Coords{ q: -2, r: 5};
    let dir = Dir::Orient120;

    let robot = Robot::new(coords, dir);

    assert_eq!(robot.coords.q, -2);
    assert_eq!(robot.coords.r, 5);
    assert_eq!(robot.orientation, Dir::Orient120);
}