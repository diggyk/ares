
// holds the Coordinates as either Axial or Cube coords
#[derive(Debug)]
pub enum CoordsKind {
    Axial {q: i32, r: i32},
    Cube {x: i32, y: i32, z: i32},
}

pub struct Coords {
    pub q: i32,
    pub r: i32,
}

impl Coords {
    pub fn to_axial(&self) -> CoordsKind {
        CoordsKind::Axial{q: self.q, r: self.r}
    }

    pub fn to_cube(&self) -> CoordsKind {
        CoordsKind::Cube{x: self.q, z: self.r, y: 0 - self.q - self.r}
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
        assert_eq!(y, 2);
        assert_eq!(z, 0);
    }

    let coords = Coords {q: 1, r: 1};
    if let CoordsKind::Cube {x, y, z} = coords.to_cube() {
        assert_eq!(x, 1);
        assert_eq!(y, -2);
        assert_eq!(z, 1);
    }
}