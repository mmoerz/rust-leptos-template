use leptos::ev::MouseEvent;
use leptos::*;
use leptos::prelude::*;

/// Modal component
///
/// # Props
/// - `open`: a signal indicating whether the modal is visible
/// - `on_close`: callback when the modal is closed
/// - `children`: content inside the modal
#[component]
pub fn Modal(
    open: ReadSignal<bool>,
    on_close: impl FnMut(MouseEvent) + 'static,
    children: Children,
) -> impl IntoView {

    view! {
        <div
            class=move || if open.get() { "modal-backdrop show" } else { "modal-backdrop hide" }
            
        >
            <div class="modal-content" on:click=|ev| ev.stop_propagation()>
                {children()}
                <button class="modal-close" on:click=on_close>"×"</button>
            </div>
        </div>
    }
}