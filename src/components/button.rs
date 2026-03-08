use leptos::ev::MouseEvent;
use leptos::*;
use leptos::prelude::*;

/// Button style variants
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

/// Button sizes
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// A reusable button component.
///
/// # Props
///
/// - `children`: content inside the button
/// - `on_click`: click event handler
/// - `class`: optional CSS classes
///
/// # Example
///
/// ```rust
/// view! {
///     <Button
///     variant=ButtonVariant::Primary
///     size=ButtonSize::Large
///     on_click=Callback::new(|_| log!("clicked"))
///     >
///         "Save"
///     </Button>
/// }
/// ```
#[component]
pub fn Button(
    children: Children,

    #[prop(optional)]
    variant: Option<ButtonVariant>,

    #[prop(optional)]
    size: Option<ButtonSize>,

    #[prop(optional)]
    disabled: bool,

    #[prop(optional)]
    loading: bool,

    #[prop(optional)]
    class: Option<&'static str>,

    //#[prop(optional)]
    on_click: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let variant_class = match variant.unwrap_or(ButtonVariant::Primary) {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Danger => "btn-danger",
    };

    let size_class = match size.unwrap_or(ButtonSize::Medium) {
        ButtonSize::Small => "btn-sm",
        ButtonSize::Medium => "btn-md",
        ButtonSize::Large => "btn-lg",
    };

    let extra_class = class.unwrap_or("");

    view! {
        <button
            class=format!("btn {} {} {}", variant_class, size_class, extra_class)
            disabled=disabled || loading
            aria-busy=loading
            on:click=on_click
        >
            {view!{
                if loading {
                    view! {<span class="btn-spinner"></span> }
                } else {
                    children().into_view()
                }}
            }
        </button>
    }
}