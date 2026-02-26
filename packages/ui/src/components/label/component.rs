use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};

use crate::label::LABEL_CSS;

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        style { {LABEL_CSS} }
        label::Label {
            class: "label",
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}
