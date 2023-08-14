use leptos::*;

#[component]
pub fn Drawer<F, IV>(cx: Scope, render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let children = children(cx).nodes.into_iter().collect::<Vec<_>>();
    let (is_open, set_is_open) = create_signal(cx, false);
    view! {
        cx,
        <details
                style: height = "auto"
                style: width = "100vw"
                style: background-color = "#414868"
                style: overflow = "auto"
                style: user-select = "none"
        >
                <summary
                    style: list-style = "none"
                    style: text-align = "center"
                    style: webkit-details-marker = "none"
                    on: click = move |_| set_is_open(!is_open())
                >
                    {render_prop()}
                </summary>

                <Show
                    when=is_open
                    fallback=|_| {"collapsed"}
                >
                    {children.to_owned()}
                </Show>

        </details>
    }
}
