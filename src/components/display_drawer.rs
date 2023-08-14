use leptos::*;

#[component]
pub fn Drawer<F, IV>(cx: Scope, render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
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
                {render_prop()}
            </div>

            {children}
        </div>
    }
}
