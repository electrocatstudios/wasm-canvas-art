use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use gloo_console::log;

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

    pub fn update(&mut self) {
        // Do movement
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        // let _ = ctx.set_fill_style(&JsValue::from(self.color.as_str()));
        // let _ = ctx.fill_rect(self.loc.x - 5.0, self.loc.y - 5.0, 10.0, 10.0);

        // let _ = ctx.set_fill_style(&JsValue::from(self.color.get_str()));
        let grad = ctx.create_radial_gradient(
            self.loc.x,
            self.loc.y,
            LIGHT_SIZE,
            self.loc.x,
            self.loc.y,
            LIGHT_SIZE * 4.0,
        ).unwrap();

        let _ = grad.add_color_stop(0.0,"white");
        let _ = grad.add_color_stop(0.5, "pink");
        let _ = grad.add_color_stop(1.0,"black" );
        
        // gradient.addColorStop(0, "pink");
        // gradient.addColorStop(0.9, "white");
        // gradient.addColorStop(1, "green");

        let _ = ctx.set_fill_style_canvas_gradient(&grad);

        // let grad = ctx.create_radial_gradient(
        //     self.loc.x,
        //     self.loc.y,
        //     LIGHT_SIZE,
        //     self.loc.x,
        //     self.loc.y,
        //     LIGHT_SIZE * 2.0,
        // ).unwrap();

        // log!(self.color.get_str());
        // let _ = ctx.set_fill_style(&JsValue::from(self.color.get_str()));
        // ctx.set_fill_style_str(self.color.get_str().as_str());
        // ctx.set_stroke_style_str(self.color.get_str().as_str());
        ctx.fill_rect(20.0, 20.0, 860.0, 660.0);
        // let _ = ctx.begin_path();
        // let _ = ctx.arc(
        //             self.loc.x,
        //             self.loc.y,
        //             LIGHT_SIZE * 2.0, 
        //             0.0, 
        //             std::f64::consts::PI * 2.0
        //         );
        let _ = ctx.fill();

        // ctx.set_stroke_style(&JsValue::from("rgb(255, 255, 0)"));
        // ctx.move_to(30.0, 30.0);
        // ctx.line_to(100.0, 100.0);
        // // ctx.line_to(GAME_WIDTH, GAME_HEIGHT);
        // // ctx.line_to(0.0, GAME_HEIGHT);
        // // ctx.line_to(0.0, 0.0);
        // ctx.stroke();
    }
}