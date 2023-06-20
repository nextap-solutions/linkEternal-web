use dioxus::prelude::*;

pub fn New(cx: Scope) -> Element {
    cx.render(rsx! { div { "New" }})
}
