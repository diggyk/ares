use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(n).collect()
}

/// Get the angle between a line and the verticle
pub fn get_angle(x1: f64, y1: f64) -> f64 {
    let x2 = 100.0;
    let y2 = 0.0;

    let ang1 = ((x1*x2) + (y1*y2))/((x1.powi(2) + y1.powi(2)).sqrt()*(x2.powi(2) + y2.powi(2)).sqrt());

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
    } else if (x1 > 0.0) && (y1 > 0.0) {
        ang1.abs()
    } else if (x1 > 0.0) && (y1 < 0.0) {
        ang1.abs() + 90.0
    } else if (x1 < 0.0) && (y1 < 0.0) {
        ang1.abs() + 180.0
    } else {
        ang1.abs() + 270.0
    }
}

#[cfg(test)]
#[test]
fn test_angles() {
    assert_eq!(45.0, get_angle(100.0, 100.0));
    assert_eq!(315.0, get_angle(-100.0, 100.0));
    assert_eq!(225.0, get_angle(-100.0, -100.0));
    assert_eq!(135.0, get_angle(100.0, -100.0));
    assert_eq!(30.0, get_angle(10.0, 17.0));
    assert_eq!(150.0, get_angle(8.66, -5.0));
    assert_eq!(265.0, get_angle(-9.9619, -0.871557));

    assert_eq!(0.0, get_angle(0.0, 10.0));
    assert_eq!(90.0, get_angle(5.0, 0.0));
    assert_eq!(180.0, get_angle(0.0, -3.0));
    assert_eq!(270.0, get_angle(-4.0, 0.0));
}