use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// holds the Coordinates as either Axial or Cube coords
#[derive(Debug)]
pub enum CoordsKind {
    Axial {q: i32, r: i32},
    Cube {x: i32, y: i32, z: i32},
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Dir {
    Orient0,
    Orient60,
    Orient120,
    Orient180,
    Orient240,
    Orient300,
}

impl From<i32> for Dir {
   fn from(item: i32) -> Dir {
       match item {
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


impl From<i16> for Dir {
    fn from(item: i16) -> Dir {
        match item {
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
        CoordsKind::Axial{q: self.q, r: self.r}
    }

    pub fn to_cube(&self) -> CoordsKind {
        CoordsKind::Cube{x: self.q, y: self.r, z: 0 - self.q - self.r}
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
            },
            Dir::Orient180 => r -= dist,
            Dir::Orient240 => q -= dist,
            Dir::Orient300 => {
                q -= dist;
                r += dist;
            }
        }

        Coords{q, r}
    }
}

#[cfg(test)]
#[test]
fn convert_to_cube() {
    let coords = Coords {q: 0, r: 0};
    if let CoordsKind::Cube {x, y, z} = coords.to_cube() {
        assert_eq!(x, 0);
        assert_eq!(y, 0);
        assert_eq!(z, 0);
    } else {
        panic!("Got the wrong type back from to_axial")
    }

    let coords = Coords {q: -2, r: 0};
    if let CoordsKind::Cube {x, y, z} = coords.to_cube() {
        assert_eq!(x, -2);
        assert_eq!(y, 0);
        assert_eq!(z, 2);
    }

    let coords = Coords {q: 1, r: 1};
    if let CoordsKind::Cube {x, y, z} = coords.to_cube() {
        assert_eq!(x, 1);
        assert_eq!(y, 1);
        assert_eq!(z, -2);
    }
}

#[test]
fn test_to() {
    let coords = Coords {q: 0, r: 0};

    let coords1 = coords.to(&Dir::Orient0, 5);
    let coords2 = coords.to(&Dir::Orient60, 24);
    let coords3 = coords.to(&Dir::Orient120, 4);
    let coords4 = coords.to(&Dir::Orient180, 934);
    let coords5 = coords.to(&Dir::Orient240, 2);
    let coords6 = coords.to(&Dir::Orient300, 32);

    assert_eq!(coords1, Coords{q: 0, r: 5});
    assert_eq!(coords2, Coords{q: 24, r: 0});
    assert_eq!(coords3, Coords{q: 4, r: -4});
    assert_eq!(coords4, Coords{q: 0, r: -934});
    assert_eq!(coords5, Coords{q: -2, r: 0});
    assert_eq!(coords6, Coords{q: -32, r: 32});
}