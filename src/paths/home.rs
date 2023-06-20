use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! { Link{to: "/new", "New"} })
}
