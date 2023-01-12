use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{classes, function_component, html, use_effect_with_deps, use_node_ref, Html};

/// canvas简单使用
#[function_component(Rect2)]
pub fn rect_2() -> Html {
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
                canvas.set_width(400);
                canvas.set_height(300);
                let context = CanvasRenderingContext2d::from(JsValue::from(
                    canvas.get_context("2d").unwrap().unwrap(),
                ));
                // 绘制矩形
                context.stroke_rect(0., 0., 200., 200.);
                // 设置绘笔样式
                // 值可以是color: 任意的css颜色值，gradient: 一个CanvasGradient对象, pattern: CanvasPattern 对象
                context.set_stroke_style(&JsValue::from_str("#FF0000"));
                context.stroke_rect(0., 0., 100., 100.);
                context.stroke_rect(50., 50., 100., 100.);
                context.stroke_rect(250., 250., 100., 100.);
                // 填充矩形: fill_rect
                // 设置填充样式: set_fill_style
            },
            (),
        );
    }
    html! {
      <canvas class={classes!("canvas")} ref={canvas_node_ref}></canvas>
    }
}
