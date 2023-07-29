use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn get_xy(id: &str) -> (f64, f64) {
    let maybe_el = document().get_element_by_id(id);
    if let Some(el) = maybe_el {
        let el = el.dyn_into::<HtmlElement>().unwrap();
        let rect = el.get_bounding_client_rect();
        let pos_x = rect.x();
        let pos_y = rect.y();
        return (pos_x, pos_y);
    } else {
        return (0.0, 0.0);
    }
}
