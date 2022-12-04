use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let closure = Closure::<dyn Fn()>::new(|| {
        let document = web_sys::window().unwrap().document().unwrap();
        let val = document.query_selector("p").unwrap().unwrap();
        val.set_inner_html("Clicked")
    });

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    val.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    body.append_child(&val)?;
    Ok(())
}
