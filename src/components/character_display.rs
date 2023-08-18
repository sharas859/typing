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

    fn get_class(&self) -> String {
        match self {
            HitRate::Zero => "zero",
            HitRate::VeryLow => "veryLow",
            HitRate::Low => "low",
            HitRate::Medium => "medium",
            HitRate::High => "high",
            HitRate::VeryHigh => "veryHigh",
        }
        .to_string()
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
                    let hit_rate = move || if counts.total.get() == 0 {
                        0.0
                    } else {
                        1.0 - counts.missed.get() as f32 / counts.total.get() as f32
                    };
                    let hitrate_class = move || HitRate::from_rate(hit_rate()).get_class();
                    // let counts = create_memo(cx, move |_| counts_map.with(|map| {*map.get(&symbol).unwrap()}));
                    view! { cx,
                        <div
                            class="counter"
                            style="width:1rem; height=10px; solid black;"
                            style:transition="background-color 0.3s ease-in-out;"
                            style:box-sizing="border-box"
                            class=hitrate_class
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
