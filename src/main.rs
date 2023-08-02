use leptos::*;
//import websys html element
//use gloo_storage::{LocalStorage, Storage};
use instant::{Duration, Instant};
use leptos_use::storage::use_storage;
use ringbuf::{Rb, StaticRb};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDialogElement, HtmlElement};
mod common;
use common::structs::*;
use common::traits::*;
use common::utils::*;
mod components;
use components::character_display::CharDisplay;
//import get_bounding_client_rect
mod word_index;
fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let mut word_index = word_index::WordIndex::new();
    let word_list = include_str!("../res/pseudowords.txt");
    word_index.read_words(word_list);
    let (wi, _) = create_signal(cx, word_index);

    //let lesson = "best dislike discrue net will aboung the occase who some and name been disgust what pass ver been antic she gree receive strust";
    //let lesson = "hello world";
    let to_train = create_rw_signal(cx, Vec::<String>::new());
    let (text, set_text) = create_signal(
        cx,
        wi.with_untracked(|wi| wi.generate_lesson_from_n_grams(50, &to_train.get_untracked())),
    );
    set_text(wi.with_untracked(|wi| wi.generate_random_lesson(50)));
    let (index, set_index) = create_signal(cx, 0);
    let (missed, set_missed) = create_signal(cx, false);
    let (x, set_x) = create_signal(cx, 0.0);
    let (y, set_y) = create_signal(cx, 0.0);

    const LAST_N_CHARS: usize = 40;
    let input_buffer = StaticRb::<Duration, LAST_N_CHARS>::default();
    let (rb_sig, set_rb_sig) = create_signal(cx, input_buffer);
    let (timer, set_timer) = create_signal(cx, Instant::now());

    let symbols: Vec<char> = vec![
        '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '~', '!', '@', '#', '$',
        '%', '^', '&', '*', '(', ')', '_', '+', '[', ']', '{', '}', '\\', '|', ';', ':', '\'', '"',
        ',', '.', '<', '>', '/', '?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z',
    ];
    //make every value in symbols a ref cell

    let map: CountsMap = symbols.iter().map(|c| (*c, Counts::new(cx))).collect();

    let (state, set_state, _) = use_storage(cx, "counts", CountsVec::from_map(map));

    //let cv = LocalStorage::get("counts_vec").unwrap_or(CountsVec::from_map(map));
    let cv = state();
    let cm = cv.into_map(cx);
    // check if map and map2 are equal

    let (counts, set_counts) = create_signal(cx, cm);

    view! { cx,
        <div // make this the whole screen, ignoring parent padding
        style="position: absolute; top:0; left:0; height:100%; width:100%; padding:0; margin:0; display: flex; flex-direction: column;  align-items: center; background-color: #1a1b26;">
            <input
                id="input"
                style="opacity:0; position:absolute; top:0; left:0; height:0; width:0;"
                on:keydown=move |e| {
                    let key = &e.key();
                    if key.len() != 1 {
                        return;
                    }
                    let typed_char = &key.chars().next().unwrap();
                    let expected_char = &text().chars().nth(index()).unwrap();
                    if index() == 0 {
                        set_timer(Instant::now());
                    }
                    if typed_char == expected_char {
                        if index() != 0 {
                            set_rb_sig
                                .update(|rb| {
                                    rb.push_overwrite(timer().elapsed());
                                });
                            set_timer(Instant::now());
                        }
                        set_counts.update(|counts| counts.incr_counts(*typed_char, missed()));
                        set_missed(false);
                        set_index.update(|i| *i += 1);
                    } else {
                        set_missed(true);
                    }
                }

                on:keyup=move |_| {
                    if index() == text().len() {
                        let cv = CountsVec::from_map(counts());
                        set_state(cv);
                        set_index(0);
                        set_text(wi.with(|wi| wi.generate_lesson_from_n_grams(50, &to_train.get_untracked())));
                    }
                }

                on:blur=move |_| {
                    let dialog = document()
                        .get_element_by_id("typeDialog")
                        .unwrap()
                        .dyn_into::<HtmlDialogElement>()
                        .unwrap();
                    dialog.show();
                }
            />

            <CharDisplay counts_map=counts to_train = to_train/>

            <div style="font-size: 2rem; width:100%; height:auto; word-break: break-all; font-family: monospace; font-weight: 400; color:#959CBD;">
                <span style="color:#414868;">
                    {move || (text()[..index()]).replace(' ', "␣")}
                </span>
                <span id="current" class:red=move || missed()>
                    // call get_xy on mount

                    {move || {
                        if index() == text().len() {
                            "".to_string()
                        } else {
                            (text()[index()..index() + 1]).replace(' ', "␣")
                        }
                    }}

                </span>
                <span id="to_write">
                    {move || {
                        let (pos_x, pos_y) = get_xy("current");
                        set_x(pos_x);
                        set_y(pos_y);
                        if index() < text().len() {
                            (text()[index() + 1..]).replace(' ', "␣")
                        } else {
                            "".to_string()
                        }
                    }}

                </span>
            </div>
            <div>
                {move || {
                    let time = rb_sig.with(|rb| rb.iter().sum::<Duration>());
                    let avg_time = time.as_secs_f32() / rb_sig.with_untracked(|rb| rb.len() as f32);
                    const MINUTE: f32 = 60.0;
                    const LETTERS_PER_WORD: f32 = 4.5;

                    format!("wpm: {}", MINUTE / avg_time / LETTERS_PER_WORD)
                }}

            </div>
            <div
                id="cursor"
                // style = "position: absolute; top:14px; left: 7px; width: 2px; height: 2rem; background-color: black;"
                style=move || {
                    format!(
                        "position: absolute; top:{}px; left:{}px; width: 2px; height: 2rem; background-color:#7dcfff;",
                        y().to_string(), x().to_string()
                    )
                }
                // easy way to hide to cursor until the first key is pressed
                style=""
            >// figure out a way to change only the position of the cursor, probably with a class

            </div>
            <dialog
                open
                style="top: 30%"
                id="typeDialog"
                // move focus to input
                on:click=move |_| {
                    let input = document()
                        .get_element_by_id("input")
                        .unwrap()
                        .dyn_into::<HtmlElement>()
                        .unwrap();
                    let dialog = document()
                        .get_element_by_id("typeDialog")
                        .unwrap()
                        .dyn_into::<HtmlDialogElement>()
                        .unwrap();
                    {
                        let (pos_x, pos_y) = get_xy("current");
                        set_x(pos_x);
                        set_y(pos_y);
                    }
                    dialog.close();
                    input.focus().unwrap();
                }
            >

                <div>"click to start typing"</div>
            </dialog>
        </div>
    }
}
