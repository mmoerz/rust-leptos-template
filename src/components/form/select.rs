use leptos::*;
use leptos::prelude::*;
use web_sys::{EventTarget, HtmlSelectElement};

/// Controlled select component
///
/// # Props
/// - `value`: current selected value
/// - `options`: list of (value, label) tuples
/// - `on_change`: callback when selection changes
/// - `placeholder`: optional placeholder / first disabled option
/// - `class`: optional CSS classes
#[component]
pub fn FormSelect(
    value: String,
    options: Vec<(String, String)>,
    on_change: impl FnMut(String) + 'static,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let (selected_value, set_selected_value) =
        signal(value);

    view! {
        <select
            class=class.unwrap_or("form-select")
            prop:value=selected_value.get()
            on:change:target=move |select| {
                let value = select.target().value();
                set_selected_value.set(value.parse().unwrap());
            }
        >
            {placeholder.map(|ph| view! { <option value="" disabled>{ph}</option> })}
            {options.into_iter().map(|(val, label)| {
                view! { <option value=val>{label}</option> }
            }).collect::<Vec<_>>()}
        </select>
    }
}