use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::buttons::button_styles::get_button_styles;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: f32,
    pub onclick: Callback<f32>,
}

#[function_component]
pub fn NumberButton(Props { value, onclick }: &Props) -> Html {
    let value = *value;

    html!(
      <button class={get_button_styles(classes!("bg-lightGrey", "active:bg-[rgb(180,180,180)]"))} onclick={onclick.reform(move |_| value)}>
        {value}
      </button>
    )
}
