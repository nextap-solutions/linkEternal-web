use dioxus::{
    html::{table, td},
    prelude::*,
};
use dioxus_router::Link;

struct Link {
    url: String,
    tags: Vec<String>,
    description: String,
}

pub fn Home(cx: Scope) -> Element {
    let links: Vec<Link> = Vec::from([
        Link {
            url: "https://www.google.com".to_string(),
            tags: Vec::from(["search".to_string(), "engine".to_string()]),
            description: "Google".to_string(),
        },
        Link {
            url: "https://www.youtube.com".to_string(),
            tags: Vec::from(["video".to_string(), "streaming".to_string()]),
            description: "Youtube".to_string(),
        },
    ]);

    let links_rsx = links
        .iter()
        .map(|link| rsx!(tr {td{ a { href: "{link.url.to_string()}", link.url.to_string()}}, td {link.description.to_string()}, td {link.tags.iter().map(|tag| rsx!(span {"{tag.to_string()}, "}))}}));

    cx.render(rsx! {  style { include_str!("../main.css") },
    Link{to: "/new", "New"}, table { tr {th {"URL"}, th {"DESCRIPTION"}, th {"TAGS"}} , links_rsx } })
}
