use dioxus::prelude::*;

use crate::NAVBAR_CSS;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        style { {NAVBAR_CSS} }

        div {
            id: "navbar",
            class: "flexing_row",
            {children}
        }
    }
}
