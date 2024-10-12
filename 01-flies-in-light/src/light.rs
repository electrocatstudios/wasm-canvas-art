use web_sys::CanvasRenderingContext2d;
// use gloo_console::log;

use crate::utils::{Color, Point};

pub struct Light {
    pub loc: Point::<f64>,
    pub color: Color
}

const LIGHT_SIZE: f64 = 20.0;

impl Light {
    pub fn new(loc: Point::<f64>, color: Color) -> Self {
        Light {
            loc: loc,
            color: color
        }
    }

    pub fn update(&mut self, _delta: f64) {
        // Nothing to do here for the moment
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {

        let grad = ctx.create_radial_gradient(
            self.loc.x,
            self.loc.y,
            LIGHT_SIZE,
            self.loc.x,
            self.loc.y,
            LIGHT_SIZE * 10.0,
        ).unwrap();

        // Add three color stops
        let _ = grad.add_color_stop(0.0, self.color.get_str_opac(0.9).as_str());
        let _ = grad.add_color_stop(0.2, self.color.get_str_opac(0.9).as_str());
        let _ = grad.add_color_stop(0.22, self.color.get_str_opac(0.5).as_str());
        let _ = grad.add_color_stop(0.8, self.color.get_str_opac(0.1).as_str());
        let _ = grad.add_color_stop(1.0, self.color.get_str_opac(0.0).as_str());

        ctx.set_fill_style_canvas_gradient(&grad);
        let _ = ctx.arc(
                    self.loc.x,
                    self.loc.y,
                    LIGHT_SIZE * 10.0, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();

    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}