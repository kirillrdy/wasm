use wasm_bindgen::prelude::*;
#[macro_export]

macro_rules! clone {
    ($($n:ident),+; || $body:block) => (
        {
            $( let $n = $n.clone(); )+
            move || { $body }
        }
    );
    ($($n:ident),+; |$($p:ident),+| $body:block) => (
        {
            $( let $n = $n.clone(); )+
            move |$($p),+| { $body }
        }
    );
}

pub mod dom {

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
        pub fn create_element(&self, tag: &str) -> Element {
            let element = self.document.create_element(tag).unwrap();
            Element { element }
        }
        pub fn create_element_ns(&self, ns: &str, tag: &str) -> Element {
            let element = self.document.create_element_ns(Some(ns), tag).unwrap();
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
        pub fn set_attribute(&self, name: &str, value: &str) {
            self.element.set_attribute(name, value).unwrap();
        }

        pub fn get_value(&self) -> String {
            self.element
                .clone()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value()
        }

        pub fn add_event_listener<Handler, ClosureArg>(&self, event_type: &str, handler: Handler)
        where
            Handler: Fn(ClosureArg) + 'static,
            ClosureArg: wasm_bindgen::convert::FromWasmAbi + 'static,
        {
            let closure = wasm_bindgen::closure::Closure::<dyn Fn(_)>::new(handler);
            self.element
                .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }

    #[derive(Clone)]
    pub struct Element {
        pub element: web_sys::Element,
    }
}

pub fn main() -> Result<(), JsValue> {
    let document = dom::document();

    let p = document.create_element("p");
    let input = document.create_element("input");

    p.set_inner_html("Hello from Rust!");
    p.add_event_listener::<_, web_sys::MouseEvent>(
        "click",
        clone!(p,input; |_event | {
            p.set_inner_html(&input.get_value())
        }),
    );

    let body = document.body();
    body.append_child(&input);
    body.append_child(&p);
    Ok(())
}
