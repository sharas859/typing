use crate::common::structs::Counts;
use leptos::*;
use linked_hash_map::LinkedHashMap;

#[component]
pub fn CharDisplay(
    cx: Scope,
    counts_map: ReadSignal<LinkedHashMap<char, Counts>>,
    to_train: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! { cx,
        <div // horizontal
        style="display: flex; flex-direction: row; flex-wrap: wrap; justify-content: center; align-items: center; height: 1rem; width: 100%;">
            <For
                // should probably do this with with sometime
                each=move || counts_map.get()
                key=|(key, _)| *key as i32
                view=move |cx, (symbol, counts)| {
                    let (clicked, set_clicked) = create_signal(cx, false);
                    // let counts = create_memo(cx, move |_| counts_map.with(|map| {*map.get(&symbol).unwrap()}));
                    view! { cx,
                        <div
                            style="width:1rem; height=10px; solid black;"
                            style:background-color=move || {
                                if counts.total.get() == 0 {
                                    return "hsl(204, 8%, 76%);".to_string();
                                }
                                let total = counts.total.get() as f32;
                                let missed = counts.missed.get() as f32;
                                let hit_rate = if counts.total.get() == 0 {
                                    0.0
                                } else {
                                    1.0 - missed / total
                                };
                                format!("hsl({}, 78%, 63%)", hit_rate * 120.0)
                            }
                            style:border = move || {
                                if clicked() {
                                    "0.1rem solid red"
                                } else {
                                    "0.1rem solid black"
                                }
                            }
                            on:click=move |_| {
                                if clicked() {
                                    set_clicked(false);
                                    to_train.update(|vec| {
                                        vec.retain(|s| s != &symbol.to_string());
                                    });
                                } else {
                                    set_clicked(true);
                                    to_train.update(|vec| {
                                        vec.push(symbol.to_string());
                                    });
                                }
                            }
                        >

                            {symbol}

                        </div>
                    }
                }
            />

        </div>
    }
}
