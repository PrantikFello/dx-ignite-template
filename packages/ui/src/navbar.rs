use dioxus::prelude::*;

use crate::{CONTAINER, DX_THEMER, FLEXING, NAVBAR_CSS, ROOT_CSS};

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        style { {DX_THEMER} }
        style { {FLEXING} }
        style { {ROOT_CSS} }
        style { {NAVBAR_CSS} }
        style { {CONTAINER} }

        div {
            id: "navbar",
            class: "flexing_row",
            {children}
        }
    }
}
