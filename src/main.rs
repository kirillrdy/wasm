use wasm_bindgen::prelude::*;

mod dom {

    use wasm_bindgen::JsCast;

    pub fn document() -> Document {
        Document {
            document: web_sys::window().unwrap().document().unwrap(),
        }
    }

    pub struct Document {
        document: web_sys::Document,
    }

    impl Document {
        pub fn create_element(self, tag: &str) -> Element {
            let element = self.document.create_element(tag).unwrap();
            Element { element }
        }

        pub fn body(&self) -> Element {
            let element = self.document.body().unwrap();
            let element = element.dyn_into::<web_sys::Element>().unwrap();
            Element { element }
        }
    }

    impl Element {
        pub fn set_inner_html(&self, text: &str) {
            self.element.set_inner_html(text)
        }
        pub fn append_child(&self, element: &Element) {
            self.element.append_child(&element.element).unwrap();
        }

        pub fn add_event_listener<F>(&self, handler: F)
        where
            F: Fn() + 'static,
        {
            let closure = wasm_bindgen::closure::Closure::<dyn Fn()>::new(handler);
            self.element
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }

    #[derive(Clone)]
    pub struct Element {
        element: web_sys::Element,
    }
}

pub fn main() -> Result<(), JsValue> {
    let document = dom::document();
    let body = document.body();
    let p = document.create_element("p");

    p.set_inner_html("Hello from Rust!");
    let p2 = p.clone();
    p.add_event_listener(move || p2.set_inner_html("foo"));

    body.append_child(&p);
    Ok(())
}
