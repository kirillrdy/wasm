use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn add_event_listener(element: &web_sys::Element, handler: &'static dyn Fn()) {
    let closure = Closure::<dyn Fn()>::new(handler);
    element
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    add_event_listener(&val, &move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let val = document.query_selector("p").unwrap().unwrap();
        val.set_inner_html("Clicked")
    });

    body.append_child(&val)?;
    Ok(())
}
