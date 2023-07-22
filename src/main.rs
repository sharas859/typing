use leptos::*;
//import websys html element
use wasm_bindgen::JsCast;
use web_sys::{HtmlDialogElement, HtmlElement};

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let text = "best dislike discrue net will aboung the occase who some and name been disgust what pass ver been antic she gree receive strust";
    let (index, set_index) = create_signal(cx, 0);
    let (correct_input, set_correct_input) = create_signal(cx, true);

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
                let expected_char = &text.chars().nth(index()).unwrap();
                if typed_char == expected_char {
                    set_correct_input(true);
                    set_index.update(|i| *i += 1);
                }
                else {
                    set_correct_input(false);
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
                {move || (&text[..index()]).replace(" ", "␣")}
            </span>
            <span
                class:red = move || !correct_input()
            >
            {move || if index() == text.len() {"".to_string()} else {(&text[index()..index()+1]).replace(" ", "␣")}}</span>
            {move || (&text[index()+1..]).replace(" ", "␣")}
        </div>

        <dialog
            open
            id="typeDialog"
            // move focus to input
            on:click=move |_| {
              //move cursor to end of input
                let input = document().get_element_by_id("input").unwrap().dyn_into::<HtmlElement>().unwrap();
                // hide this dialog, make sure it also closes when you click on the text
                let dialog = document().get_element_by_id("typeDialog").unwrap().dyn_into::<HtmlDialogElement>().unwrap();
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
