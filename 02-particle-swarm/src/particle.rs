use web_sys::CanvasRenderingContext2d;
use rand::Rng;
use gloo_console::log;

use crate::utils::{Color, Point, normalize_angle, get_angle_between_points,dist_between_points};

pub struct Particle {
    start_loc: Point::<f64>,
    dest_loc: Point::<f64>,
    speed: f64,
    rot_speed: f64,
    loc: Point::<f64>,
    color: Color,
    dest_color: Color,
    update_cooldown: f64,
    radius: f64,
    angle: f64,
    prev_points: Vec::<Point::<f64>>
}

const PARTICLE_BORDER: i32 = 10;
const FRAME_TIME: f64 = 0.01;
const MAX_SPEED: f64 = 9.0;
const MIN_SPEED: f64 = 6.0;
const MIN_RADIUS: f64 = 20.0;
const MAX_RADIUS: f64 = 100.0;
const MAX_TRAIL: usize = 20;
const PARTICLE_SIZE: f64 = 1.0;

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
        let r2: u8 = 105 + (150-col_val);
        let g2: u8 = 155 + rng.gen_range(0..85);
        let b2: u8 = 105 + (col_val);
        
        let speed = rng.gen_range(MIN_SPEED..MAX_SPEED);

        Particle {
            dest_loc: Point::new(loc_x as f64, loc_y as f64),
            loc: Point::new(loc_x as f64, loc_y as f64),
            start_loc: Point::new(loc_x as f64, loc_y as f64),
            color: Color::new(r,g,b),
            dest_color: Color::new(r2,g2,b2),
            speed: speed * 90.0,
            rot_speed: speed,
            update_cooldown: FRAME_TIME,
            radius: rng.gen_range(MIN_RADIUS..MAX_RADIUS),
            angle: rng.gen_range(0.0..std::f64::consts::PI),
            prev_points: Vec::new()
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.update_cooldown -= delta;
        if self.update_cooldown > 0.0 {
            self.update_color();
            self.move_particle(delta);
        } else {
            self.update_cooldown += FRAME_TIME;
        }
    }

    pub fn move_particle(&mut self, delta: f64) {
        if self.is_at_loc() {
            self.angle = normalize_angle( self.angle + (self.rot_speed * delta));
        } else {
            log!("Moving to dest");
            let ang = get_angle_between_points(&self.loc, &self.dest_loc);
            self.loc.x += ang.sin() * (self.speed * delta);
            self.loc.y -= ang.cos() * (self.speed * delta);
            
            if dist_between_points(&self.loc, &self.dest_loc) < self.radius {
                self.angle = get_angle_between_points(&self.dest_loc, &self.loc);
                self.loc.x = self.dest_loc.x;
                self.loc.y = self.dest_loc.y;
            }
        }
    }

    fn is_at_loc(&self) -> bool {
        self.loc.x == self.dest_loc.x && self.loc.y == self.dest_loc.y
    }

    pub fn set_dest_loc(&mut self, pt: Point::<f64>) {
        if self.is_at_loc() {
            self.loc.x = self.loc.x + (self.angle.sin() * self.radius);
            self.loc.y = self.loc.y - (self.angle.cos() * self.radius); 
        }
        
        self.dest_loc = pt;
    }

    pub fn reset_loc(&mut self) {
        if self.is_at_loc() {
            self.loc.x = self.loc.x + (self.angle.sin() * self.radius);
            self.loc.y = self.loc.y - (self.angle.cos() * self.radius); 
        }
        
        self.dest_loc.x = self.start_loc.x;
        self.dest_loc.y = self.start_loc.y;
    }

    pub fn update_color(&mut self) {
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
            let mut rng = rand::thread_rng();
            let col_val = rng.gen_range(0..150);
            let r: u8 = 105 + (150-col_val);
            let g: u8 = 155 + rng.gen_range(0..85);
            let b: u8 = 105 + (col_val);
            self.dest_color = Color::new(r,g,b);
        }
    }
    
    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        for (idx,pp) in self.prev_points.iter().enumerate() {
            let str_rep = self.color.get_str_opac( idx as f64/self.prev_points.len() as f64 );
            // log!(str_rep.clone(), idx, self.prev_points.len());
            let _ = ctx.set_fill_style_str(&str_rep);

            let _ = ctx.fill_rect(pp.x - PARTICLE_SIZE, pp.y - PARTICLE_SIZE, PARTICLE_SIZE * 2.0, PARTICLE_SIZE * 2.0);
        }
        
        let _ = ctx.set_fill_style_str(&self.color.get_str_opac(1.0));
        let (x,y) = if self.is_at_loc() {
            let x = self.loc.x + (self.angle.sin() * self.radius);
            let y = self.loc.y - (self.angle.cos() * self.radius); 
            (x,y)
        } else {
            (self.loc.x, self.loc.y)
        };

        self.prev_points.push(Point::new(x, y));
        if self.prev_points.len() > MAX_TRAIL {
            self.prev_points.remove(0);
        }
        let _ = ctx.fill_rect(x - PARTICLE_SIZE, y - PARTICLE_SIZE, PARTICLE_SIZE * 2.0, PARTICLE_SIZE * 2.0);
    }
}