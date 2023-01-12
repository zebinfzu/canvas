use yew::{function_component, html, Html};

mod canvas_base;
#[allow(unused)]
use canvas_base::{Base1, Rect2, Path3};

#[function_component(App)]
fn app() -> Html {
    html! {
      <>
        // <Base1 />
        // <Rect2 />
        <Path3 />
      </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
