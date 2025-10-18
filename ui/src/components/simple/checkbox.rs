use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct Props {
    is_selected: bool,
}

#[component]
pub fn Checkbox(props: Props) -> Element {
    rsx! {
      div {
        class: "w-5 h-5 border-2 rounded flex items-center justify-center",
        class: if props.is_selected { "border-teal-400 bg-teal-500" } else { "border-gray-500" },
        if props.is_selected {
          "✓"
        }
      }
    }
}
