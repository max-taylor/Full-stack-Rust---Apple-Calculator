use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::{app::Action, buttons::button_styles::get_button_styles};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub action: Action,
    pub selected_action: Option<Action>,
    pub onclick: Callback<Action>,
}

#[function_component]
pub fn ActionButton(
    Props {
        action,
        selected_action,
        onclick,
    }: &Props,
) -> Html {
    let action = *action;

    let is_selected = {
        if let Some(selected_action) = selected_action {
            *selected_action == action
        } else {
            false
        }
    };

    let classes = {
        if is_selected {
            classes!(
                "border-[1.2px]",
                "border-black",
                "h-full",
                "w-full",
                "flex",
                "items-center",
                "justify-center"
            )
        } else {
            classes!("")
        }
    };

    html!(
      <button class={get_button_styles(classes!("bg-orange", "active:bg-[rgb(184,115,51)]"))} onclick={onclick.reform(move |_| action)}>
          <div class={classes}>{action.to_string()}</div>
      </button>
    )
}
