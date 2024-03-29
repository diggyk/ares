use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use rand::seq::SliceRandom;
use serde_repr;
use std::io::Write;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// holds the Coordinates as either Axial or Cube coords
#[derive(Debug)]
pub enum CoordsKind {
    Axial { q: i32, r: i32 },
    Cube { x: i32, y: i32, z: i32 },
    Flat2D { x: f64, y: f64 },
}

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    AsExpression,
    FromSqlRow,
    PartialOrd,
    serde_repr::Serialize_repr,
)]
#[repr(i16)]
#[sql_type = "SmallInt"]
pub enum Dir {
    Orient0 = 0,
    Orient60 = 60,
    Orient120 = 120,
    Orient180 = 180,
    Orient240 = 240,
    Orient300 = 300,
}

impl<DB> ToSql<SmallInt, DB> for Dir
where
    DB: Backend,
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i16).to_sql(out)
    }
}

impl<DB> FromSql<SmallInt, DB> for Dir
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> Result<Self> {
        match i16::from_sql(bytes)? {
            0 => Ok(Dir::Orient0),
            60 => Ok(Dir::Orient60),
            120 => Ok(Dir::Orient120),
            180 => Ok(Dir::Orient180),
            240 => Ok(Dir::Orient240),
            300 => Ok(Dir::Orient300),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

impl From<i16> for Dir {
    fn from(item: i16) -> Dir {
        let i = if item < 0 {
            item + 360
        } else if item > 300 {
            item - 360
        } else {
            item
        };
        match i {
            0 => Dir::Orient0,
            60 => Dir::Orient60,
            120 => Dir::Orient120,
            180 => Dir::Orient180,
            240 => Dir::Orient240,
            300 => Dir::Orient300,
            _ => Dir::Orient0,
        }
    }
}

impl From<Dir> for i16 {
    fn from(item: Dir) -> i16 {
        match item {
            Dir::Orient0 => 0,
            Dir::Orient60 => 60,
            Dir::Orient120 => 120,
            Dir::Orient180 => 180,
            Dir::Orient240 => 240,
            Dir::Orient300 => 300,
        }
    }
}

impl From<i32> for Dir {
    fn from(item: i32) -> Dir {
        let i = if item < 0 {
            item + 360
        } else if item > 300 {
            item - 360
        } else {
            item
        };
        match i {
            0 => Dir::Orient0,
            60 => Dir::Orient60,
            120 => Dir::Orient120,
            180 => Dir::Orient180,
            240 => Dir::Orient240,
            300 => Dir::Orient300,
            _ => Dir::Orient0,
        }
    }
}

impl From<Dir> for i32 {
    fn from(item: Dir) -> i32 {
        match item {
            Dir::Orient0 => 0,
            Dir::Orient60 => 60,
            Dir::Orient120 => 120,
            Dir::Orient180 => 180,
            Dir::Orient240 => 240,
            Dir::Orient300 => 300,
        }
    }
}

impl From<&Dir> for i32 {
    fn from(item: &Dir) -> i32 {
        match item {
            Dir::Orient0 => 0,
            Dir::Orient60 => 60,
            Dir::Orient120 => 120,
            Dir::Orient180 => 180,
            Dir::Orient240 => 240,
            Dir::Orient300 => 300,
        }
    }
}

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        match rng.gen_range(0, 6) {
            0 => Dir::Orient0,
            1 => Dir::Orient60,
            2 => Dir::Orient120,
            3 => Dir::Orient180,
            4 => Dir::Orient240,
            5 => Dir::Orient300,
            _ => Dir::Orient0,
        }
    }
}

impl Dir {
    pub fn left(&self, angle: i32) -> Dir {
        let a1: i32 = self.clone().into();
        let mut a1 = a1 - angle;

        if a1 < 0 {
            a1 = a1 + 360;
        }

        a1.into()
    }

    pub fn right(&self, angle: i32) -> Dir {
        let a1: i32 = self.clone().into();
        let mut a1 = a1 + angle;

        if a1 > 360 {
            a1 = a1 - 360;
        }

        a1.into()
    }

    pub fn get_vec() -> Vec<Dir> {
        vec![
            Dir::Orient0,
            Dir::Orient60,
            Dir::Orient120,
            Dir::Orient180,
            Dir::Orient240,
            Dir::Orient300,
        ]
    }

    pub fn get_random() -> Dir {
        let dirs = Dir::get_vec();
        let mut rng = rand::thread_rng();
        *dirs.choose(&mut rng).unwrap()
    }

    // starting with the given orientation, make a vector of each other
    // orientation, switching from left to right
    pub fn get_side_scan_iter(start: Dir) -> Vec<Dir> {
        let start: i16 = start.into();
        let mut dirs = Vec::new();

        dirs.push(start.into());

        for x in &[60, 120] {
            let mut temp = start - x;
            if temp < 0 {
                temp += 360;
            }
            dirs.push(temp.into());

            temp = start + x;
            if temp > 360 {
                temp -= 360;
            }
            dirs.push(temp.into());
        }

        let mut temp = start - 180;
        if temp < 0 {
            temp += 360;
        }
        dirs.push(temp.into());

        dirs
    }

    // Get the opposite side
    pub fn get_opposite(&self) -> Dir {
        let mut angle: i16 = self.clone().into();
        angle += 180;
        if angle > 360 {
            angle -= 360;
        }
        angle.into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Coords {
    pub q: i32,
    pub r: i32,
}

#[cfg(test)]
#[test]
fn test_dir_conversions() {
    let d1: Dir = 60.into();
    let d2: i32 = Dir::Orient300.into();
    let d3: Dir = 55.into();

    assert_eq!(d1, Dir::Orient60);
    assert_eq!(d2, 300);
    assert_eq!(d3, Dir::Orient0);
}

impl Coords {
    pub fn to_axial(&self) -> CoordsKind {
        CoordsKind::Axial {
            q: self.q,
            r: self.r,
        }
    }

    pub fn to_cube(&self) -> CoordsKind {
        CoordsKind::Cube {
            x: self.q,
            y: self.r,
            z: 0 - self.q - self.r,
        }
    }

    // this conversion is to map the center of the cells onto a standard
    // flat plane with an x and y axis
    pub fn to_flat2d(&self) -> CoordsKind {
        let q = self.q as f64;
        let r = self.r as f64;

        let x = (2.0 / 3f64.sqrt() + 1.0 / 3f64.sqrt()) * q;
        let y = q + r * 2.0;

        CoordsKind::Flat2D { x, y }
    }

    pub fn to(&self, dir: &Dir, dist: i32) -> Coords {
        let mut q = self.q;
        let mut r = self.r;

        match dir {
            Dir::Orient0 => r += dist,
            Dir::Orient60 => q += dist,
            Dir::Orient120 => {
                q += dist;
                r -= dist;
            }
            Dir::Orient180 => r -= dist,
            Dir::Orient240 => q -= dist,
            Dir::Orient300 => {
                q -= dist;
                r += dist;
            }
        }

        Coords { q, r }
    }

    /// Get the distance between these coords and a target
    pub fn distance_to(&self, target: &Coords) -> i32 {
        let a = self.to_cube();
        let b = target.to_cube();

        if let CoordsKind::Cube {
            x: ax,
            y: ay,
            z: az,
        } = a
        {
            if let CoordsKind::Cube {
                x: bx,
                y: by,
                z: bz,
            } = b
            {
                return ((ax - bx).abs() + (ay - by).abs() + (az - bz).abs()) / 2;
            }
        }

        -1
    }
}

#[cfg(test)]
#[test]
fn convert_to_cube() {
    let coords = Coords { q: 0, r: 0 };
    if let CoordsKind::Cube { x, y, z } = coords.to_cube() {
        assert_eq!(x, 0);
        assert_eq!(y, 0);
        assert_eq!(z, 0);
    } else {
        panic!("Got the wrong type back from to_axial")
    }

    let coords = Coords { q: -2, r: 0 };
    if let CoordsKind::Cube { x, y, z } = coords.to_cube() {
        assert_eq!(x, -2);
        assert_eq!(y, 0);
        assert_eq!(z, 2);
    }

    let coords = Coords { q: 1, r: 1 };
    if let CoordsKind::Cube { x, y, z } = coords.to_cube() {
        assert_eq!(x, 1);
        assert_eq!(y, 1);
        assert_eq!(z, -2);
    }
}

#[test]
fn test_distance() {
    let coords = Coords { q: 0, r: 0 };
    assert_eq!(coords.distance_to(&Coords { q: 0, r: 4 }), 4);
    assert_eq!(coords.distance_to(&Coords { q: -2, r: 3 }), 3);
}

#[test]
fn test_to() {
    let coords = Coords { q: 0, r: 0 };

    let coords1 = coords.to(&Dir::Orient0, 5);
    let coords2 = coords.to(&Dir::Orient60, 24);
    let coords3 = coords.to(&Dir::Orient120, 4);
    let coords4 = coords.to(&Dir::Orient180, 934);
    let coords5 = coords.to(&Dir::Orient240, 2);
    let coords6 = coords.to(&Dir::Orient300, 32);

    assert_eq!(coords1, Coords { q: 0, r: 5 });
    assert_eq!(coords2, Coords { q: 24, r: 0 });
    assert_eq!(coords3, Coords { q: 4, r: -4 });
    assert_eq!(coords4, Coords { q: 0, r: -934 });
    assert_eq!(coords5, Coords { q: -2, r: 0 });
    assert_eq!(coords6, Coords { q: -32, r: 32 });
}

#[derive(Debug)]
pub struct CoordsAndDir {
    pub coords: Coords,
    pub dir: Dir,
}
