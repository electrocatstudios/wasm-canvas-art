use js_sys::Math::atan2;

pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b
        }
    }

    pub fn _get_str(&self) -> String {
        format!("rgba({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn get_str_opac(&self, opac: f64) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, opac)
    }
}


pub fn dist_between_points(pt1: &Point<f64>, pt2: &Point<f64>) -> f64 {
    let diff_x = pt1.x - pt2.x;
    let diff_y = pt1.y - pt2.y;
    ((diff_x*diff_x) + (diff_y*diff_y)).sqrt()
}

pub fn get_angle_between_points(start: &Point::<f64>, end: &Point::<f64> ) -> f64 {
    let diff_x = end.x - start.x;
    let diff_y = start.y - end.y; // Invert because axis inverted
    normalize_angle((std::f64::consts::PI/2.0) - atan2(diff_y, diff_x))
}


const TWO_PI: f64 = std::f64::consts::PI * 2.0;

// Return angle between 0 and 2 * PI
pub fn normalize_angle(angle: f64) -> f64 {
    // Make sure angle is between 0 and 2*pi
    if angle < 0.0 {
        angle + TWO_PI
    } else if angle > TWO_PI {
        angle - TWO_PI
    } else {
        angle
    }
}