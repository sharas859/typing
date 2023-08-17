use leptos::*;
//import websys html element
use gloo_storage::{LocalStorage, Storage};
use instant::{Duration, Instant};
//use leptos_use::storage::use_storage;
use leptos::html::{Dialog, Input};
use ringbuf::{Rb, StaticRb};
mod common;
use common::structs::*;
use common::traits::*;
use common::utils::*;
mod components;
use components::character_display::CharDisplay;
use components::display_drawer::Drawer;
//import get_bounding_client_rect
mod word_index;
fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

enum RegenType {
    Regenerate,
    Reset,
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
    let symbols_to_train = create_rw_signal(cx, Vec::<String>::new());
    let (text, set_text) = create_signal(
        cx,
        wi.with_untracked(|wi| wi.generate_lesson_from_n_grams(50, &to_train.get_untracked())),
    );
    set_text(wi.with_untracked(|wi| wi.generate_random_lesson_string(50)));
    let (index, set_index) = create_signal(cx, 0);
    let (missed, set_missed) = create_signal(cx, false);
    let (x, set_x) = create_signal(cx, 0.0);
    let (y, set_y) = create_signal(cx, 0.0);

    const LAST_N_CHARS: usize = 40;
    let input_buffer = StaticRb::<Duration, LAST_N_CHARS>::default();
    let (rb_sig, set_rb_sig) = create_signal(cx, input_buffer);
    let (timer, set_timer) = create_signal(cx, Instant::now());
    let (is_typing, set_is_typing) = create_signal(cx, false);

    let dialog_ref = create_node_ref::<Dialog>(cx);
    let input_ref = create_node_ref::<Input>(cx);

    // todo make this a static struct
    let symbols: Vec<&str> = vec![
        "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "~", "!", "@", "#", "$",
        "%", "^", "&", "*", "(", ")", "_", "+", "[", "]", "{", "}", "\\", "|", ";", ":", "\'",
        "\"", ",", ".", "<", ">", "/", "?", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
        "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C",
        "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
        "V", "W", "X", "Y", "Z",
    ];

    let letters: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let capitals: Vec<&str> = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    let numbers: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

    let symbols: Vec<&str> = vec![
        "`", "~", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+", "[", "]", "{", "}",
        "\\", "|", ";", ":", "\'", "\"", ",", ".", "<", ">", "/", "?",
    ];

    //    let bigrams = symbols
    //        .iter()
    //        .flat_map(|&c1| symbols.iter().map(move |&c2| format!("{}{}", c1, c2)))
    //        .collect::<Vec<String>>();
    //make every value in symbols a ref cell
    // combine symbols and bigrams
    let bigrams = letters
        .iter()
        .flat_map(|&c1| letters.iter().map(move |&c2| format!("{}{}", c1, c2)))
        .collect::<Vec<String>>();

    let bigram_map: CountsMap = bigrams
        .iter()
        .map(|bigram| (bigram.clone(), Counts::new(cx)))
        .collect();

    let symbols_map: CountsMap = symbols
        .iter()
        .map(|&c| (c.to_string(), Counts::new(cx)))
        .collect();

    let map: CountsMap = letters
        .iter()
        .map(|&c| (c.to_string(), Counts::new(cx)))
        .collect();

    //    let (state, set_state, _) = use_storage(cx, "counts", CountsVec::from_map(map));
    //    let (bigram_state, set_bigram_state, _) =
    //        use_storage(cx, "bigram_counts", CountsVec::from_map(bigram_map));
    //    let (symbols_state, set_symbols_state, _) =
    //        use_storage(cx, "symbols_counts", CountsVec::from_map(symbols_map));

    let state = LocalStorage::get("counts").unwrap_or(CountsVec::from_map(map));
    let bigram_state =
        LocalStorage::get("bigram_counts").unwrap_or(CountsVec::from_map(bigram_map));
    let symbols_state =
        LocalStorage::get("symbols_counts").unwrap_or(CountsVec::from_map(symbols_map));

    //let cv = LocalStorage::get("counts_vec").unwrap_or(CountsVec::from_map(map));
    //    let cm = state.get_untracked().into_map(cx);
    //    let bm = bigram_state.get_untracked().into_map(cx);
    //    let sm = symbols_state.get_untracked().into_map(cx);
    let cm = state.into_map(cx);
    let bm = bigram_state.into_map(cx);
    let sm = symbols_state.into_map(cx);

    // check if map and map2 are equal

    let (counts, set_counts) = create_signal(cx, cm);
    let (bigram_counts, set_bigram_counts) = create_signal(cx, bm);
    let (symbols_counts, set_symbols_counts) = create_signal(cx, sm);
    let last_char = store_value(cx, "".to_string());

    let reset_lesson = move |regen: RegenType| {
        set_index(0);
        if let regen = RegenType::Regenerate {
            set_text(wi.with_untracked(|wi| {
                wi.generate_lesson_string_from_ngrams_with_special_chars(
                    50,
                    &to_train.get_untracked(),
                    &symbols_to_train.get_untracked(),
                )
            }));
        }

        set_missed(false);
        let (pos_x, pos_y) = get_xy("current", false);
        set_x(pos_x);
        set_y(pos_y);
        last_char.set_value("".to_string());
    };

    let handle_popup = move || {
        let input = input_ref.get().expect("input ref not set");
        let dialog = dialog_ref.get().expect("dialog ref not set");
        {
            let (pos_x, pos_y) = get_xy("current", false);
            set_x(pos_x);
            set_y(pos_y);
        }

        dialog.close();
        set_is_typing(true);

        input.focus().unwrap();
    };

    create_effect(cx, move |_| {
        to_train();
        symbols_to_train();
        reset_lesson(RegenType::Regenerate);
    });

    view! { cx,
            <div // make this the whole screen, ignoring parent padding
            style="position: absolute; top:0; left:0; height:100%; width:100%; padding:0; margin:0; display: flex; flex-direction: column;  align-items: center; background-color: #1a1b26;">
                <input
                    _ref = input_ref
                    id="input"
                    style="opacity:0; position:absolute; top:0; left:0; height:0; width:0;"
                    on:keydown=move |e| {
                        let key = &e.key();
                        if key == "Escape" {
                            reset_lesson(RegenType::Reset);
                            return;
                        }
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

                            set_counts.update(|counts| counts.incr_counts(typed_char.to_string(), missed()));
                            set_symbols_counts.update(|counts| counts.incr_counts(typed_char.to_string(), missed()));

                            if !last_char().is_empty() {
                                let bigram = format!("{}{}", last_char(), typed_char);
                                set_bigram_counts.update(|counts| counts.incr_counts(bigram, missed()));
                            }
                            last_char.set_value(typed_char.to_string());

                            set_missed(false);
                            set_index.update(|i| *i += 1);



                        } else {
                            set_missed(true);
                        }

                    }

                    on:keyup=move |_| {
                        if index() == text().len() {
                            let cv = CountsVec::from_map(counts());
                            //set_state(cv);
                            if LocalStorage::set("counts", cv).is_err() {
                                log!("error writing to storage");
                            }
                            let bv = CountsVec::from_map(bigram_counts());
                            if LocalStorage::set("bigram_counts", bv).is_err() {
                                log!("error writing to storage");
                            }
                            //set_bigram_state.set_untracked(bv);
                            let sv = CountsVec::from_map(symbols_counts());
                            if LocalStorage::set("symbols_counts", sv).is_err() {
                                log!("error writing to storage");
                            }
                            //set_symbols_state(sv);
                            log!("wrote to storage");


                            reset_lesson(RegenType::Regenerate);
    //                        set_index(0);
    //                        set_text(wi.with_untracked(|wi| {
    //                            wi.generate_lesson_string_from_ngrams_with_special_chars(
    //                                50,
    //                                &to_train.get_untracked(),
    //                                &symbols_to_train.get_untracked(),
    //                            )
    //                        }));
    //                        let (pos_x,u pos_y) = get_xy("current", false);
    //                        set_x(pos_x);
    //                        set_y(pos_y);
                        }
                    }

                    on:blur=move |_| {
                        let dialog = dialog_ref.get().expect("dialog ref not set");

                        reset_lesson(RegenType::Reset);
                        set_is_typing(false);
                        dialog.show();
                        dialog.focus().unwrap();
                    }
                />

                //<CharDisplay counts_map=counts to_train = to_train/>

                <Drawer
                    render_prop = || view! {cx, <CharDisplay counts_map=counts to_train = to_train/>}
                >
                    <CharDisplay counts_map=bigram_counts to_train = to_train />
                </Drawer>
                //<CharDisplay counts_map=symbols_counts, to_train=symbols_to_train/>
                <CharDisplay counts_map=symbols_counts to_train = symbols_to_train/>

                <div style="font-size: 2rem; width:100%; height:auto; word-break: break-all; font-family: monospace; font-weight: 400; color:#959CBD; text-align: center">
                    <span style="color:#414868;">
                        {move || (text()[..index()]).replace(' ', "␣")}
                    </span>
                    <span id="current" class:red=missed>
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
                            let (pos_x, pos_y) = get_xy("current", true);
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
                        const LETTERS_PER_WORD: f32 = 5.0;

                        format!("wpm: {}", MINUTE / avg_time / LETTERS_PER_WORD)
                    }}

                </div>
                <div
                    id="cursor"
                    // style = "position: absolute; top:14px; left: 7px; width: 2px; height: 2rem; background-color: black;"
                    style=move || {
                        if !is_typing() {
                            return "display: none;".to_string();
                        }
                        format!(
                            "position: absolute ; top:{}px; left:{}px; width: 2px; height: 2rem; background-color:#7dcfff; transition: left 0.1s ease-in-out;",
                            y().to_string(), x().to_string()
                        )
                    }
                >// figure out a way to change only the position of the cursor, probably with a class

                </div>
                <dialog
                    _ref = dialog_ref
                    open
                    style="top: 30%"
                    id="typeDialog"
                    // move focus to input
                    on:click= move |_| {
                        handle_popup();
                    }
                >

                    <div>"Click to start typing"</div>
                </dialog>
            </div>
        }
}
