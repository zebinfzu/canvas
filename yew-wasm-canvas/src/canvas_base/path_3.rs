use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{classes, function_component, html, use_effect_with_deps, use_node_ref, Html};

/// 绘笔
#[function_component(Path3)]
pub fn path_3() -> Html {
    let canvas_node_ref = use_node_ref();
    {
        let canvas_node_ref = canvas_node_ref.clone();
        use_effect_with_deps(
            move |_| {
                let canvas = canvas_node_ref.cast::<HtmlCanvasElement>();
                let canvas = if let Some(canvas) = canvas {
                    canvas
                } else {
                    println!("canvas not attach");
                    return;
                };
                canvas.set_width(600);
                canvas.set_height(400);
                let context = CanvasRenderingContext2d::from(JsValue::from(
                    canvas.get_context("2d").unwrap().unwrap(),
                ));
                // begin_path, move, line_to, close_path, stoke
                context.begin_path();
                context.move_to(20., 20.);
                context.line_to(200., 20.);
                context.line_to(120., 120.);
                context.close_path();
                context.stroke();
            },
            (),
        );
    }
    html! {
      <canvas class={classes!("canvas")} ref={canvas_node_ref}></canvas>
    }
}