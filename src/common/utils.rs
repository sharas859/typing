use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn get_xy(id: &str, centered: bool) -> (f64, f64) {
    let maybe_el = document().get_element_by_id(id);
    if let Some(el) = maybe_el {
        let el = el.dyn_into::<HtmlElement>().unwrap();
        let rect = el.get_bounding_client_rect();
        //offset for centered text
        let mut pos_x = rect.x();
        if centered {
            pos_x += rect.width() / 2.0;
        }
        let pos_y = rect.y();
        (pos_x, pos_y)
    } else {
        (0.0, 0.0)
    }
}
