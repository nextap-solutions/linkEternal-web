#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
mod paths;
use paths::home::Home;
use paths::new::New;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/new", New {} }
            Route { to: "/", Home {} }
        }
    })
}
