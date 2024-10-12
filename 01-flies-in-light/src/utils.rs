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

    pub fn get_str(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}


pub fn dist_between_points(pt1: Point<f64>, pt2: Point<f64>) -> f64 {
    let diff_x = pt1.x - pt2.x;
    let diff_y = pt1.y - pt2.y;
    ((diff_x*diff_x) + (diff_y*diff_y)).sqrt()
}