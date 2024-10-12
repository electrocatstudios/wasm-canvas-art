use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use gloo_console::log;

use crate::{light::Light, utils::{Color, Point}};

pub struct CanvasControl {
    callback: Closure<dyn FnMut()>,
    canvas: NodeRef,
    light: Light
}

pub enum CanvasControlMsg {
    MouseDown((f64, f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    TouchStart((f64, f64)),
    TouchEnd((f64, f64)),
    TouchMove((f64, f64)),
    Render,
    Null
}


#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct CanvasControlProps;

impl Component for CanvasControl {
    type Message = CanvasControlMsg;
    type Properties = CanvasControlProps;

    fn create(ctx: &Context<Self>) -> Self {
        // log!("HGello world");
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(CanvasControlMsg::Render)) as Box<dyn FnMut()>);

        ctx.link().send_message(CanvasControlMsg::Render);

        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();
        // log!(width, height);

        CanvasControl{
            callback: callback,
            canvas: NodeRef::default(),
            light: Light::new(Point::new(width / 2.0, height / 2.0), Color::new(155, 255, 155))
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            CanvasControlMsg::MouseDown(evt) => {
                
                true
            },
            CanvasControlMsg::MouseUp(_evt) => {
                true
            },
            CanvasControlMsg::MouseMove(evt) => {
                // log!("Event here => ", self.mousehandler.offset_x, self.mousehandler.offset_y);
                true
            },
            CanvasControlMsg::TouchStart(evt) => {
                // log!("Event here TouchStart => ", evt.0, evt.1);
                true
            },
            CanvasControlMsg::TouchEnd(_evt) => {
                // log!("Event here TouchEnd => ", evt.0, evt.1);
                true
            },
            CanvasControlMsg::TouchMove(evt) => {
                // log!("Event here TouchMove => ", evt.0, evt.1);
                true
            },
            CanvasControlMsg::Render => {
                // log!("Render");
                self.render();
                true
            },
            CanvasControlMsg::Null => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousedown = ctx.link().callback(move |evt: MouseEvent| {
            CanvasControlMsg::MouseDown((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmousemove = ctx.link().callback(move |evt: MouseEvent| {
            CanvasControlMsg::MouseMove((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmouseup = ctx.link().callback(move |evt: MouseEvent| {
            CanvasControlMsg::MouseUp((evt.page_x() as f64, evt.page_y() as f64))
        });
        let ontouchstart = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => CanvasControlMsg::TouchStart((touch.page_x() as f64, touch.page_y() as f64)),
                None => CanvasControlMsg::Null,
            }
        });
        let ontouchend = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => CanvasControlMsg::TouchEnd((touch.page_x() as f64, touch.page_y() as f64)),
                None => CanvasControlMsg::Null,
            }
        });
        let ontouchmove = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => CanvasControlMsg::TouchMove((touch.page_x() as f64, touch.page_y() as f64)),
                None => CanvasControlMsg::Null,
            }
        });

        html! { 
            <div class="game_canvas">
                <canvas id="canvas"
                    style={"margin: 0px; width: 100vw; height: 100vh, left:0px; top:0px;"}
                    onmousedown={onmousedown}
                    onmousemove={onmousemove}
                    onmouseup={onmouseup}
                    ontouchstart={ontouchstart}
                    ontouchend={ontouchend}
                    ontouchmove={ontouchmove}
                    ref={self.canvas.clone()}
                    tabindex = "1"
                ></canvas>
            </div>
        }
    }
}

impl CanvasControl {
    fn canvas_update(&mut self) {
        self.light.update();
    }

    fn render(&mut self) {
        self.canvas_update();

        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        
        let width = canvas.client_width() as f64;
        let height = canvas.client_height() as f64;
        
        // let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        // let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();
        // log!(width, height);

        // Make sure the we reset the draw surface to prevent stretching
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let mut ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();

        ctx.set_fill_style_str("rgb(55, 155, 55)");
        ctx.fill_rect(0.0, 0.0, width, height);
        ctx.stroke();

        self.light.render(&mut ctx);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}