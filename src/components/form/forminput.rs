use leptos::*;
use leptos::prelude::*;

/// Controlled input component
///
/// # Props
/// - `value`: the current value of the input
/// - `on_input`: event handler for input changes
/// - `placeholder`: optional placeholder text
/// - `class`: optional CSS classes
#[component]
pub fn FormInput(
    value: String,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (input_value, set_input_value) = 
        signal(value);

    view! {
        <input
            type="text"
            class=class.unwrap_or("form-input")
            placeholder=placeholder.unwrap_or("")
            value=input_value.get()
            bind:value=(input_value, set_input_value)
        />
    }
}