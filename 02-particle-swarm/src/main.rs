use yew::prelude::*;
use yew_router::prelude::*;

mod canvas_control;
mod particle;
mod utils;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/scenes/particle-swarm")]
    Home,
}

fn switch(routes: Route) -> Html {    
    match routes {
        Route::Home => html!{
            <canvas_control::CanvasControl />
        }
    }
}

#[function_component(App)]
fn app_body() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
