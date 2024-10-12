use web_sys::CanvasRenderingContext2d;
use rand::Rng;
// use gloo_console::log;

use crate::utils::{dist_between_points, get_angle_between_points, Point};

pub struct Fly {
    pub loc: Point::<f64>,
    goal: Point::<f64>,
    dest_loc: Point::<f64>,
    dist: f64,
    speed: f64
}

const FLY_SPEED: f64 = 150.0;
const FLY_LIGHT_OFFSET: i32 = 350;
const LIGHT_RANGE: i32 = 240;

impl Fly {
    pub fn new(light: Point::<f64> ) -> Self {
        let mut rng = rand::thread_rng();
        let start = Point::new(
            light.x - rng.gen_range(-FLY_LIGHT_OFFSET..FLY_LIGHT_OFFSET) as f64,
            light.y - rng.gen_range(-FLY_LIGHT_OFFSET..FLY_LIGHT_OFFSET) as f64
        );
        let dist = dist_between_points(&start, &light);
        let speed = (rng.gen_range(0..20) as f64) + FLY_SPEED;
        Fly {
            loc: start,
            goal: Point::new(light.x, light.y),
            dest_loc: Point::new(
                light.x - rng.gen_range(-FLY_LIGHT_OFFSET..FLY_LIGHT_OFFSET) as f64,
                light.y - rng.gen_range(-FLY_LIGHT_OFFSET..FLY_LIGHT_OFFSET) as f64
            ),
            dist: dist,
            speed: speed
        }
    }

    pub fn update(&mut self, delta: f64) {
        // Do movement
        let mut rng = rand::thread_rng();
        let mut angle = get_angle_between_points(&self.loc, &self.dest_loc);
        angle += (50.0 - (rng.gen_range(0..100) as f64)) / 100.0; // Add some wiggle to the movement
        
        self.loc.x += (delta * self.speed) * angle.sin();
        self.loc.y -= (delta * self.speed) * angle.cos();

        if dist_between_points(&self.loc, &self.dest_loc) < self.speed {
            self.dest_loc.x = self.goal.x - rng.gen_range(-FLY_LIGHT_OFFSET..FLY_LIGHT_OFFSET) as f64;
            self.dest_loc.y = self.goal.y - rng.gen_range(-FLY_LIGHT_OFFSET..FLY_LIGHT_OFFSET) as f64;
        }
        self.dist = dist_between_points(&self.loc, &self.goal);
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        // Calculate distance from light
        let perc = if self.dist < LIGHT_RANGE as f64 {
            1.0 - (self.dist / LIGHT_RANGE as f64)
        } else {
            0.0
        };
        
        let r = 15 + (perc * 85.0) as i32;
        let g = 15 + (perc * 85.0) as i32;
        let b = 5 + (perc * 55.0) as i32;

        let col_str = format!("rgb({}, {}, {})", r, g, b);
        
        let _ = ctx.set_fill_style_str(&col_str);
        let _ = ctx.fill_rect(self.loc.x - 2.0, self.loc.y - 2.0, 4.0, 4.0);
    }
}