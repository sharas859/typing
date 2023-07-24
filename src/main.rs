use leptos::*;
//import websys html element
use linked_hash_map::LinkedHashMap;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Counts {
    total: i32,
    missed: i32,
}

trait IncrCounts {
    fn incr_counts(&mut self, c: char, missed: bool);
}

impl IncrCounts for LinkedHashMap<char, Counts> {
    fn incr_counts(&mut self, c: char, missed: bool) {
        if let Some(entry) = self.get_mut(&c) {
            entry.total += 1;
            if missed {
                entry.missed += 1;
            }
        }
    }
}

#[component]
fn CharDisplay(cx: Scope, counts_map: ReadSignal<LinkedHashMap<char, Counts>>) -> impl IntoView {
    view! {
        cx,
        <div
            //horizontal
            style = "display: flex; flex-direction: row; flex-wrap: wrap; justify-content: center; align-items: center; height: 1rem; width: 100%;"
        >
            <For
                // should probably do this with with sometime
                each = move || counts_map.get()
                key = |key| *key
                view = move |cx, (symbol,_)| {
                let counts = create_memo(cx, move |_| counts_map.with(|map| {map.get(&symbol).unwrap().clone()}));
                let hit_rate = if counts().total == 0 {0.0} else {1.0 - (counts().missed as f32 / counts().total as f32)};
                view! {
                    cx,
                    <div
                       style = move || format!("width:1rem; height=10px; border:0.1rem solid black; background-color: hsl({}, 78%, 63%);", hit_rate * 120.0)
                    >
                        {
                           symbol
                        }
                    </div>
                }
            }
            />
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let lesson = "best dislike discrue net will aboung the occase who some and name been disgust what pass ver been antic she gree receive strust";
    //let lesson = "hello world";
    let (text, set_text) = create_signal(cx, lesson.to_string());
    let (index, set_index) = create_signal(cx, 0);
    let (missed, set_missed) = create_signal(cx, false);
    let (x, set_x) = create_signal(cx, 0.0);
    let (y, set_y) = create_signal(cx, 0.0);
    let symbols: Vec<char> = vec![
        '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '~', '!', '@', '#', '$',
        '%', '^', '&', '*', '(', ')', '_', '+', '[', ']', '{', '}', '\\', '|', ';', ':', '\'', '"',
        ',', '.', '<', '>', '/', '?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z',
    ];
    //make every value in symbols a ref cell
    let map: LinkedHashMap<char, Counts> = symbols
        .iter()
        .map(|c| {
            (
                *c,
                Counts {
                    total: 0,
                    missed: 0,
                },
            )
        })
        .collect();
    let (counts, set_counts) = create_signal(cx, map);

    view! {


        cx,
    <div
        //make this the whole screen, ignoring parent padding
        style = "position: absolute; top:0; left:0; height:100%; width:100%; padding:0; margin:0; display: flex; flex-direction: column;  align-items: center; background-color: #1a1b26;"
    >
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
                    set_counts.update(|counts| counts.incr_counts(*typed_char, missed()));
                    set_missed(false);
                    set_index.update(|i| *i += 1);


                }
                else {
                    set_missed(true);
                }

            }
            on: keyup = move |_| {

                if index() == text().len() {
                        counts.with(|map| {
                            for (key, val) in map.iter() {
                                //log!("{}: {}/{}", key, val.missed, val.count);
                            }
                        });
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

        <CharDisplay counts_map = counts/>


        <div
            style = "font-size: 2rem; width:100%; height:auto; word-break: break-all; font-family: monospace; font-weight: 400; color:#c0caf5;"
        >
            <span
                style = "color:#444b6a;"
            >
                {move || (&text()[..index()]).replace(" ", "␣")}
            </span>
            <span
                id = "current"
                class:red = move || missed()
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
            style = move || {format!("position: absolute; top:{}px; left:{}px; width: 2px; height: 2rem; background-color:#89ddf3;", y().to_string(),x().to_string())}
            //easy way to hide to cursor until the first key is pressed
            style = ""
            // figure out a way to change only the position of the cursor, probably with a class

        >
        </div>
        <dialog
            open
            style = "top: 30%"
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
    </div>



    }
}
