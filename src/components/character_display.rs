use crate::common::structs::Counts;
use leptos::html::Div;
use leptos::*;
use linked_hash_map::LinkedHashMap;

enum HitRate {
    Zero,
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl HitRate {
    fn from_rate(rate: f32) -> Self {
        if rate == 0.0 {
            HitRate::Zero
        } else if rate < 0.2 {
            HitRate::VeryLow
        } else if rate < 0.4 {
            HitRate::Low
        } else if rate < 0.6 {
            HitRate::Medium
        } else if rate < 0.8 {
            HitRate::High
        } else if rate <= 1.0 {
            HitRate::VeryHigh
        } else {
            // Default case
            HitRate::Zero
        }
    }

    fn get_color(&self) -> String {
        match self {
            HitRate::Zero => "hsl(204, 8%, 76%);".to_string(),
            HitRate::VeryLow => "#F7768E;".to_string(),
            HitRate::Low => "#8C8587".to_string(),
            HitRate::Medium => "#E18C85".to_string(),
            HitRate::High => "#B4B873".to_string(),
            HitRate::VeryHigh => "#9ECE6A".to_string(),
        }
    }
}

#[component]
pub fn CharDisplay(
    cx: Scope,
    counts_map: ReadSignal<LinkedHashMap<String, Counts>>,
    to_train: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! { cx,
        // horizontal
        <div style="display: flex; flex-direction: row; flex-wrap: wrap; justify-content: center; align-items: center; height: auto; width: 100%;">
            <For
                // should probably do this with with sometime
                each=move || counts_map.get()
                key=|(key, _)| key.clone()
                view=move |cx, (symbol, counts)| {
                    let (clicked, set_clicked) = create_signal(cx, false);
                    let sym = symbol.clone();
                    let handle = create_node_ref::<Div>(cx);
                    // let counts = create_memo(cx, move |_| counts_map.with(|map| {*map.get(&symbol).unwrap()}));
                    view! { cx,
                        <div
                            _ref=handle
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
                                HitRate::from_rate(hit_rate).get_color()
                            }

                            style:border=move || {
                                if clicked() { "0.1rem solid red" } else { "0.1rem solid black" }
                            }

                            style:cursor="pointer"

                            on:click=move |e| {
                                e.stop_propagation();
                                if clicked() {
                                    set_clicked(false);
                                    to_train
                                        .update(|vec| {
                                            vec.retain(|s| s != &symbol.to_string());
                                        });
                                } else {
                                    set_clicked(true);
                                    to_train
                                        .update(|vec| {
                                            vec.push(symbol.to_string());
                                        });
                                }
                            }



                        >

                            {sym}

                        </div>
                    }
                }
            />

        </div>
    }
}
