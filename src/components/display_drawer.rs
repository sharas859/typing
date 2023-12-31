use leptos::html::Div;
use leptos::*;

#[component]
pub fn Drawer<F, IV>(cx: Scope, render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let children = children(cx).nodes.into_iter().collect::<Vec<_>>();
    let (is_open, set_is_open) = create_signal(cx, false);
    view! { cx,
        <div
            style:height="auto"
            style:width="100vw"
            style:background-color="#414868"
            style:overflow="auto"
            style:user-select="none"
        >

            <div
                style:text-align="center"
                style:display="flex"
                style:flex-direction="row"
                style:cursor = "pointer"
                on:click=move |_| set_is_open(!is_open())
                class="drawer-header"
            >
                {render_prop()}
                {move || if is_open() { "▲" } else { "▼" }}
            </div>




        <div >
            <Show when=is_open fallback=|cx| view! { cx, <div></div> }>
                {children.to_owned()}
            </Show>
        </div>
        </div>
    }
}
