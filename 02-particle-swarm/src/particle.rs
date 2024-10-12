use web_sys::CanvasRenderingContext2d;
use rand::Rng;
use gloo_console::log;

use crate::utils::{Color, Point};

pub struct Particle {
    bounds: Point::<f64>,
    loc: Point::<f64>,
    color: Color,
    dest_color: Color,
    update_cooldown: f64
}

const PARTICLE_BORDER: i32 = 10;
const FRAME_TIME: f64 = 0.01;

impl Particle {
    pub fn new(bound_x: f64, bound_y: f64) -> Self {
        let mut rng = rand::thread_rng();

        let loc_x = rng.gen_range(PARTICLE_BORDER..bound_x as i32-PARTICLE_BORDER);
        let loc_y = rng.gen_range(PARTICLE_BORDER..bound_y as i32-PARTICLE_BORDER);

        let col_val = rng.gen_range(0..100);
        let r: u8 = 155 + (100-col_val);
        let g: u8 = 155 + rng.gen_range(0..85);
        let b: u8 = 155 + (col_val);

        let col_val = rng.gen_range(0..150);
        let r2: u8 = 105 + (100-col_val);
        let g2: u8 = 155 + rng.gen_range(0..85);
        let b2: u8 = 105 + (col_val);

        Particle {
            bounds: Point::new(bound_x, bound_y),
            loc: Point::new(loc_x as f64, loc_y as f64),
            color: Color::new(r,g,b),
            dest_color: Color::new(r2,g2,b2),
            update_cooldown: FRAME_TIME
        }
    }

    pub fn update(&mut self, delta: f64) {
        // log!(self.update_cooldown);
        self.update_cooldown -= delta;
        if self.update_cooldown > 0.0 {
            return;
        }
        // log!("Update");
        self.update_cooldown += FRAME_TIME;

        let mut is_equal = true;
        if self.dest_color.r > self.color.r {
            self.color.r += 1;
            is_equal = false;
        } else if self.dest_color.r < self.color.r {
            self.color.r -= 1;
            is_equal = false;
        }
        if self.dest_color.g > self.color.g {
            self.color.g += 1;
            is_equal = false;
        } else if self.dest_color.g < self.color.g {
            self.color.g -= 1;
            is_equal = false;
        }
        if self.dest_color.b > self.color.b {
            self.color.b += 1;
            is_equal = false;
        } else if self.dest_color.b < self.color.b {
            self.color.b -= 1;
            is_equal = false;
        }

        if is_equal {
            log!("Picking new color");
            let mut rng = rand::thread_rng();
            let col_val = rng.gen_range(0..150);
            let r: u8 = 105 + (100-col_val);
            let g: u8 = 155 + rng.gen_range(0..85);
            let b: u8 = 105 + (col_val);
            self.dest_color = Color::new(r,g,b);
        }
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let _ = ctx.set_fill_style_str(&self.color.get_str_opac(1.0));
        let _ = ctx.fill_rect(self.loc.x - 5.0, self.loc.y - 5.0, 10.0, 10.0);
    }
}