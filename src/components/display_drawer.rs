use leptos::*;

#[component]
pub fn Drawer<F, IV>(cx: Scope, render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let children = children(cx).nodes.into_iter().collect::<Vec<_>>();
    view! {
        cx,
        <details
                style: height = "auto"
                style: width = "100vw"
                style: background-color = "#414868"
                style: overflow = "auto"
        >
                <summary
                    style: list-style = "none"
                    style: text-align = "center"
                    style: webkit-details-marker = "none"
                >
                    {render_prop()}
                </summary>
                {children}
        </details>
    }
}
