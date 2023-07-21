use leptos::*;

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
        >
        </input>


        <div
            style = "font-size: 2em; width:100%; height:auto; word-break: break-all;"
        >
            {move || (&text[..index()]).replace(" ", "␣")}
            <span
                class:red = move || !correct_input()
                style = "font-weight:bold;"
            >{move || if index() == text.len() {"".to_string()} else {(&text[index()..index()+1]).replace(" ", "␣")}}</span>
            {move || (&text[index()+1..]).replace(" ", "␣")}
        </div>

        <dialog
            open = move || index() == text.len()
        >
            <div>
                "click to start typing"
            </div>
        </dialog>

    }
}
