use leptos::*;

#[component]
pub fn drawer(cx: Scope, title: String, children: Children) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(cx, false);
    let children = children(cx).nodes.into_iter().collect::<Vec<_>>();
    view! {
        cx,
        <div
                class = "drawer"
                class: open = is_open
        >
            <div
                style: height = "1rem"
                style: width = "100vw"
                style: background-color = "#414868"
                on:click = move |_| {
                    set_is_open(!is_open());
                }
            >
                {title}
            </div>

            {children}
        </div>
    }
}
