use leptos::*;
//import websys html element
use wasm_bindgen::JsCast;
use web_sys::{HtmlDialogElement, HtmlElement};
//import get_bounding_client_rect

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}

fn get_xy(id: &str) -> (f64, f64) {
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

#[component]
fn App(cx: Scope) -> impl IntoView {
    //let text = "best dislike discrue net will aboung the occase who some and name been disgust what pass ver been antic she gree receive strust";
    let lesson = "hello world";
    let (text, set_text) = create_signal(cx, lesson.to_string());
    let (index, set_index) = create_signal(cx, 0);
    let (correct_input, set_correct_input) = create_signal(cx, true);
    let (x, set_x) = create_signal(cx, 0.0);
    let (y, set_y) = create_signal(cx, 0.0);

    view! {
        cx,
        <input
            id = "input"
            style = "opacity:0; position:absolute; top:0; left:0; height:0; width:0;"
            on:keydown=move |e| {
                let key = &e.key();
                // get rid of modifier keys
                if key.len() != 1 {
                  return;
                }
                let typed_char = &key.chars().next().unwrap();
                let expected_char = &text().chars().nth(index()).unwrap();
                if typed_char == expected_char {
                    set_correct_input(true);
                    set_index.update(|i| *i += 1);
                }
                else {
                    set_correct_input(false);
                }

            }
            on: keyup = move |_| {

                if index() == text().len() {
                        set_index(0);
                        set_text(lesson.to_string());
                }
            }

            on:blur=move |_| {
                let dialog = document().get_element_by_id("typeDialog").unwrap().dyn_into::<HtmlDialogElement>().unwrap();
                dialog.show();
            }
        >
        </input>


        <div
            style = "font-size: 2rem; width:100%; height:auto; word-break: break-all; font-family: monospace; font-weight: 400; color:black;"
        >
            <span
                style = "color:gray;"
            >
                {move || (&text()[..index()]).replace(" ", "␣")}
            </span>
            <span
                id = "current"
                class:red = move || !correct_input()
                //call get_xy on mount

            >
                {move || {

                    if index() == text().len() {"".to_string()} else {(&text()[index()..index()+1]).replace(" ", "␣")}
                }}
            </span>
            <span
                id = "to_write"
            >
                {move || {
                    // cursor needs to be updated here, so current is already updated
                    let (pos_x,pos_y) = get_xy("current");
                    set_x(pos_x);
                    set_y(pos_y);



                    if index() < text().len(){
                        (&text()[index()+1..]).replace(" ", "␣")
                    }
                    else {"".to_string()}


                }}

            </span>
        </div>
        <div
            id = "cursor"
            //style = "position: absolute; top:14px; left: 7px; width: 2px; height: 2rem; background-color: black;"
            style = move || {format!("position: absolute; top:{}px; left:{}px; width: 2px; height: 2rem; background-color: black;", y().to_string(),x().to_string())}
            //easy way to hide to cursor until the first key is pressed
            style = ""
            // figure out a way to change only the position of the cursor, probably with a class

        >
        </div>


        <dialog
            open
            id="typeDialog"
            // move focus to input
            on:click=move |_| {
              //move cursor to end of input
                let input = document().get_element_by_id("input").unwrap().dyn_into::<HtmlElement>().unwrap();
                let dialog = document().get_element_by_id("typeDialog").unwrap().dyn_into::<HtmlDialogElement>().unwrap();
                        {
                let(pos_x,pos_y) = get_xy("current");
                set_x(pos_x);
                set_y(pos_y);
        }
                dialog.close();
                input.focus().unwrap();
            }
        >
            <div>
                "click to start typing"
            </div>
        </dialog>

    }
}
