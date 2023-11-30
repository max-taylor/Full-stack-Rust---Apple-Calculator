use yew::prelude::*;

use crate::buttons::{
    action_button::ActionButton, button_styles::get_button_styles, number_button::NumberButton,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Add,
    Subtract,
    Divide,
    Multiply,
    Equals,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Add => "+".to_string(),
            Action::Subtract => "-".to_string(),
            Action::Divide => "÷".to_string(),
            Action::Multiply => "*".to_string(),
            Action::Equals => "=".to_string(),
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let temp_value: UseStateHandle<Option<f32>> = use_state(|| None);
    let memory: UseStateHandle<f32> = use_state(|| 0.0);
    let action: UseStateHandle<Option<Action>> = use_state(|| None);
    let trailing_dot: UseStateHandle<bool> = use_state(|| false);
    let has_dot_pressed: UseStateHandle<bool> = use_state(|| false);

    let display_value: f32 = {
        if let Some(i) = *temp_value {
            i
        } else {
            *memory
        }
    };

    let clear_all = {
        if (*temp_value).is_some() || (*action).is_some() || *trailing_dot {
            false
        } else {
            true
        }
    };

    let clear_state = {
        let temp_value = temp_value.clone();
        let action = action.clone();
        let memory = memory.clone();
        let trailing_dot = trailing_dot.clone();
        let has_dot_pressed = has_dot_pressed.clone();

        Callback::from(move |_| {
            if clear_all {
                memory.set(0.0);
                temp_value.set(None);
                action.set(None);
            } else {
                action.set(None);
                temp_value.set(None);
            }

            trailing_dot.set(false);
            has_dot_pressed.set(false);
        })
    };

    let invert_sign = {
        let temp_value = temp_value.clone();
        let memory = memory.clone();

        Callback::from(move |_| {
            if let Some(i) = *temp_value {
                temp_value.set(Some(i * -1.0));
            } else {
                memory.set(*memory * -1.0);
            }
        })
    };

    let apply_percentage = {
        let temp_value = temp_value.clone();
        let memory = memory.clone();

        Callback::from(move |_| {
            if let Some(i) = *temp_value {
                temp_value.set(Some(i / 100.0));
            } else {
                memory.set(*memory / 100.0);
            }
        })
    };

    let handle_action_button = {
        let temp_value = temp_value.clone();
        let memory = memory.clone();
        let action = action.clone();
        let trailing_dot = trailing_dot.clone();
        let has_dot_pressed = has_dot_pressed.clone();

        Callback::from(move |passed_action: Action| {
            // If there is an action already pressed and a temporary_value, then we want to apply that previous action
            if let (Some(existing_action), Some(temp_value)) = (*action, *temp_value) {
                match existing_action {
                    Action::Add => {
                        memory.set(*memory + temp_value);
                    }
                    Action::Subtract => {
                        memory.set(*memory - temp_value);
                    }
                    Action::Multiply => {
                        memory.set(*memory * temp_value);
                    }
                    Action::Divide => {
                        memory.set(*memory / temp_value);
                    }
                    Action::Equals => {
                        memory.set(temp_value);
                    }
                }
            } else {
                let memory_update = match *temp_value {
                    Some(i) => i,
                    None => *memory,
                };

                memory.set(memory_update);
            }
            if passed_action != Action::Equals {
                action.set(Some(passed_action));
            } else {
                action.set(None);
            }

            // Reset our temporary states
            temp_value.set(None);
            trailing_dot.set(false);
            has_dot_pressed.set(false);
        })
    };

    let handle_number_button = {
        let temp_value = temp_value.clone();
        let trailing_dot = trailing_dot.clone();
        let has_dot_pressed = has_dot_pressed.clone();

        Callback::from(move |value: f32| {
            if let Some(i) = *temp_value {
                let trailing_dot_string = match *trailing_dot {
                    true => ".",
                    false => "",
                };

                let new_value = format!("{}{}{}", i, trailing_dot_string, value);

                temp_value.set(Some(new_value.parse::<f32>().unwrap()));
            } else {
                if *trailing_dot {
                    let value = format!("0.{}", value).parse::<f32>().unwrap();
                    temp_value.set(Some(value));
                } else {
                    temp_value.set(Some(value));
                }
            }

            if *trailing_dot {
                trailing_dot.set(false);
                has_dot_pressed.set(true);
            }
        })
    };

    // let handle_dot_press = {
    //     let trailing_dot = trailing_dot.clone();
    //     let has_dot_pressed = has_dot_pressed.clone();

    //     Callback::from(move |_| {
    //         if *has_dot_pressed {
    //             return;
    //         }

    //         trailing_dot.set(true);
    //     })
    // };

    html! {
      <div class={classes!("h-screen", "w-screen", "bg-background", "flex-col", "flex")}>
        <div class={classes!("flex", "justify-end", "text-5xl", "pr-2", "py-1")}>
          {display_value}{match *trailing_dot {
            true => ".",
            false => "",
          }}
        </div>
        <div class={classes!("flex", "flex-1", "flex-row")}>
          <button onclick={clear_state} class={get_button_styles(classes!("bg-darkGrey","active:bg-lightGrey"))}>
            {match clear_all {
              true => "AC",
              false => "C",
            }}
          </button>
          <button onclick={invert_sign} class={get_button_styles(classes!("bg-darkGrey","active:bg-lightGrey"))}>
            {"±"}
          </button>
          <button onclick={apply_percentage} class={get_button_styles(classes!("bg-darkGrey","active:bg-lightGrey"))}>
            {"%"}
          </button>
          <ActionButton action={Action::Divide} onclick={handle_action_button.clone()} selected_action={*action}  />
        </div>
        // <div class={classes!("flex", "flex-1", "flex-row")}>
        //   <NumberButton value={7.0} onclick={handle_number_button.clone()} />
        //   <NumberButton value={8.0} onclick={handle_number_button.clone()} />
        //   <NumberButton value={9.0} onclick={handle_number_button.clone()} />
        //   <ActionButton action={Action::Multiply} onclick={handle_action_button.clone()} selected_action={*action} />
        // </div>
        // <div class={classes!("flex", "flex-1", "flex-row")}>
        //   <NumberButton value={4.0} onclick={handle_number_button.clone()} />
        //   <NumberButton value={5.0} onclick={handle_number_button.clone()} />
        //   <NumberButton value={6.0} onclick={handle_number_button.clone()} />
        //   <ActionButton action={Action::Subtract} onclick={handle_action_button.clone()} selected_action={*action} />
        // </div>
        // <div class={classes!("flex", "flex-1", "flex-row")}>
        //   <NumberButton value={1.0} onclick={handle_number_button.clone()} />
        //   <NumberButton value={2.0} onclick={handle_number_button.clone()} />
        //   <NumberButton value={3.0} onclick={handle_number_button.clone()} />
        //   <ActionButton action={Action::Add} onclick={handle_action_button.clone()} selected_action={*action} />
        // </div>
        // <div class={classes!("flex", "flex-1", "flex-row")}>
        //   <div class={classes!("flex", "flex-1")}>
        //     <NumberButton value={0.0} onclick={handle_number_button.clone()} />
        //   </div>
        //   <div class={classes!("flex", "flex-1")}>
        //     <button onclick={handle_dot_press} class={get_button_styles(classes!("bg-lightGrey"))}>
        //       {"."}
        //     </button>
        //     <ActionButton action={Action::Equals} onclick={handle_action_button.clone()} selected_action={*action}  />
        //   </div>
        // </div>
      </div>
    }
}
