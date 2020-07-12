use crate::utils;
use crate::db::DbClient;
use crate::grid::*;

pub struct Robot<'a> {
    dbclient: Option<&'a DbClient>,

    pub id: u32,
    pub name: String,

    pub coords: Coords,
    pub orientation: Dir,
    pub known: Vec<GridCell>,
}

impl<'a> Robot<'a> {
    pub fn new(coords: Coords, orientation: Dir) -> Robot<'a> {
        Robot {
            dbclient: None,
            id: 0,
            name: utils::random_string(8),
            coords,
            orientation,
            known: Vec::new(),
        }
    }

    pub fn attach_db(&mut self, dbclient: &'a DbClient) {
        self.dbclient = Some(dbclient);

        self.register();
    }

    fn register(&self) {
        
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

#[test]
fn test_db_attaching() {
    let mut robot = Robot::new(
        Coords{q: -1, r: 1},
        Dir::Orient60,
    );

    let dbclient = DbClient::new(
        "testuser",
        "testpw",
        "testhost",
        "testdb",
    );

    robot.attach_db(&dbclient);
}