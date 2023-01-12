use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{classes, function_component, html, use_effect_with_deps, use_node_ref, Html};

/// canvas简单使用
#[function_component(Base1)]
pub fn base_1() -> Html {
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
                canvas.set_width(200);
                canvas.set_height(100);
                // 获得canvas的绘图上下文
                // 对于2d绘图上下文，左上角是(0, 0)
                let context = CanvasRenderingContext2d::from(JsValue::from(
                    canvas.get_context("2d").unwrap().unwrap(),
                ));
                context.set_fill_style(&JsValue::from_str("#FF0000"));
                context.fill_rect(0.0, 0.0, 150., 75.);
            },
            (),
        );
    }
    html! {
      <canvas class={classes!("canvas")} ref={canvas_node_ref}></canvas>
    }
}