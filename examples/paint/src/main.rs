use geo_booleanop::boolean::BooleanOp;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm::clone;
use wasm::dom;
use web_sys;

//TODO precalculate circle, and then just add x and y in a loop
fn circle(x: f64, y: f64) -> geo::MultiPolygon {
    let segments = 10;
    let radius = 20;
    let d_theta = std::f64::consts::PI * 2.0 / segments as f64;
    let mut polygon = Vec::new();

    for i in 0..segments {
        let theta = i as f64 * d_theta;
        let new_point = (x + (radius as f64) * theta.cos(), y + (radius as f64) * theta.sin());
        polygon.push(new_point);
    }
    let theta = 0 as f64 * d_theta;
    let new_point = (x + (radius as f64) * theta.cos(), y + (radius as f64) * theta.sin());
    println!();
    polygon.push(new_point);
    geo::MultiPolygon(vec![geo::Polygon::new(geo::LineString::from(polygon), Vec::new())])
}

//TODO replace map collect with push_str and time them
fn d_for_multy_polygon(multy_polygon: &geo::MultiPolygon) -> String {
    let mut buffer = String::new();

    for polygon in multy_polygon.iter() {
        buffer.push_str("M ");
        let line = polygon
            .exterior()
            .0
            .iter()
            .map(|point| format!("{} {}", point.x, point.y))
            .collect::<Vec<String>>()
            .join(" L ");

        buffer.push_str(&line);

        for path in polygon.interiors() {
            buffer.push_str("M ");
            let line = path
                .0
                .iter()
                .map(|point| format!("{} {}", point.x, point.y))
                .collect::<Vec<String>>()
                .join(" L ");

            buffer.push_str(&line);
        }
    }

    buffer
}

fn perf_to_system(amt: f64) -> std::time::Duration {
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
    std::time::Duration::new(secs, nanos)
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = dom::document();
    let body = document.body();

    let svg = document.create_element_ns("http://www.w3.org/2000/svg", "svg");
    svg.set_attribute("style", "border: solid 1px;");

    let path = document.create_element_ns("http://www.w3.org/2000/svg", "path");
    path.set_attribute("stroke", "green");
    path.set_attribute("fill", "red");
    path.set_attribute("fill-opacity", "100%");
    path.set_attribute("fill-rule", "evenodd");
    svg.set_attribute("width", "800");
    svg.set_attribute("height", "600");
    svg.append_child(&path);
    body.append_child(&svg);

    let polygon = geo::MultiPolygon(vec![geo::Polygon::new(geo::LineString(vec![]), vec![])]);
    let polygon = RefCell::new(polygon);

    let pressed = Rc::new(Cell::new(false));
    svg.add_event_listener::<_, web_sys::MouseEvent>("mousedown", clone!(pressed ; |_event| { pressed.set(true);}));
    svg.add_event_listener::<_, web_sys::MouseEvent>("mouseup", clone!(pressed ; |_event| { pressed.set(false);}));

    svg.add_event_listener::<_, web_sys::MouseEvent>(
        "mousemove",
        clone!( pressed;  |event| {
            if pressed.get() {
                let point = circle(event.offset_x() as f64, event.offset_y() as f64);
                let mut polygon = polygon.borrow_mut();
                *polygon = polygon.union(&point);
                let now = web_sys::window().unwrap().performance().unwrap().now();
                let d = d_for_multy_polygon(&polygon);
                path.set_attribute("d", &d);
                let later = web_sys::window().unwrap().performance().unwrap().now();
                web_sys::console::log_1(&format!("{:?} {:?}", perf_to_system(later - now), later - now).into());
            }
        }),
    );
}
