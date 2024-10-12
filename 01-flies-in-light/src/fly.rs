use web_sys::CanvasRenderingContext2d;

use crate::utils::Point;

struct Fly {
    pub loc: Point::<f64>
}

impl Fly {
    pub fn new(x: f64, y: f64) -> Self {
        Fly {
            loc: Point::new(x,y)
        }
    }

    pub fn update(&mut self) {
        // Do movement
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        // let _ = ctx.set_fill_style(&JsValue::from(self.color.as_str()));
        // let _ = ctx.fill_rect(self.loc.x - 5.0, self.loc.y - 5.0, 10.0, 10.0);
    }
}