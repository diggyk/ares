use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::grid::Coords;
use crate::grid::CoordsKind;
use crate::grid::Dir;

pub fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(n).collect()
}

/// Get the angle between a line and the verticle
pub fn get_angle(x1: f64, y1: f64) -> f64 {
    let x2 = 100.0;
    let y2 = 0.0;

    let ang1 = ((x1 * x2) + (y1 * y2))
        / ((x1.powi(2) + y1.powi(2)).sqrt() * (x2.powi(2) + y2.powi(2)).sqrt());

    let ang1 = (ang1.acos() * 180.0 / std::f64::consts::PI).round() - 90.0;

    if x1 == 0.0 {
        if y1 > 0.0 {
            0.0
        } else {
            180.0
        }
    } else if y1 == 0.0 {
        if x1 > 0.0 {
            90.0
        } else {
            270.0
        }
    // positive, positive
    } else if (x1 > 0.0) && (y1 > 0.0) {
        ang1.abs()
    // positive, negative
    } else if (x1 > 0.0) && (y1 < 0.0) {
        ang1.abs() + 90.0
    // negative, negative
    } else if (x1 < 0.0) && (y1 < 0.0) {
        ang1 + 180.0
    // negative, positive
    } else {
        90.0 - ang1 + 270.0
    }
}

#[cfg(test)]
#[test]
fn test_angles() {
    assert_eq!(45.0, get_angle(100.0, 100.0));
    assert_eq!(315.0, get_angle(-100.0, 100.0));
    // assert_eq!(225.0, get_angle(-100.0, -100.0));
    assert_eq!(135.0, get_angle(100.0, -100.0));
    assert_eq!(30.0, get_angle(10.0, 17.0));
    assert_eq!(150.0, get_angle(8.66, -5.0));
    assert_eq!(265.0, get_angle(-9.9619, -0.871557));

    assert_eq!(300.0, get_angle(-3.4641016151377553, 2.0));
    assert_eq!(0.0, get_angle(0.0, 10.0));
    assert_eq!(90.0, get_angle(5.0, 0.0));
    assert_eq!(180.0, get_angle(0.0, -3.0));
    assert_eq!(270.0, get_angle(-4.0, 0.0));
}

/// Get the bearing between the origin (coords1) and target(coords2)
pub fn get_bearing(dir: Dir, coords1: Coords, coords2: Coords) -> Option<i32> {
    let x1: f64;
    let y1: f64;
    let mut x2: f64;
    let mut y2: f64;

    if let CoordsKind::Flat2D { x: cx, y: cy } = coords1.to_flat2d() {
        x1 = cx;
        y1 = cy;
    } else {
        return None;
    }

    if let CoordsKind::Flat2D { x: cx, y: cy } = coords2.to_flat2d() {
        x2 = cx;
        y2 = cy;
    } else {
        return None;
    }

    // make x1, y1 relative to 0, 0
    x2 -= x1;
    y2 -= y1;

    let a1: i32 = dir.into();
    let a2 = get_angle(x2, y2) as i32;

    let mut bearing = a2 - a1;
    if bearing > 180 {
        bearing -= 360;
    } else if bearing < -180 {
        bearing += 360;
    }

    Some(bearing)
}

#[test]
fn test_bearings() {
    assert_eq!(
        Some(0),
        get_bearing(Dir::Orient0, Coords { q: 0, r: 0 }, Coords { q: 0, r: 5 })
    );

    assert_eq!(
        Some(-60),
        get_bearing(Dir::Orient0, Coords { q: 0, r: 0 }, Coords { q: -2, r: 2 })
    );

    assert_eq!(
        Some(0),
        get_bearing(
            Dir::Orient180,
            Coords { q: -2, r: 3 },
            Coords { q: -2, r: 0 }
        )
    );

    assert_eq!(
        Some(-120),
        get_bearing(
            Dir::Orient120,
            Coords { q: -3, r: 4 },
            Coords { q: -3, r: 6 }
        )
    );

    assert_eq!(
        Some(-180),
        get_bearing(
            Dir::Orient240,
            Coords { q: 1, r: -2 },
            Coords { q: 3, r: -2 }
        )
    );
}
