use leptos::*;
use chrono::Datelike;
use leptos::prelude::*;

/// Footer component for the application.
///
/// # Example
/// ```rust
/// view! {
///     <Footer/>
/// }
/// ```
#[component]
pub fn Footer() -> impl IntoView {
    let year = chrono::Utc::now().year();

    view! {
        <footer class="app-footer">
            <div class="container">
                <p>
                    "© "
                    {year}
                    " MyLeptosApp. All rights reserved."
                </p>
                <p>
                    "Built with "
                    <a href="https://leptos.rs" target="_blank" rel="noopener noreferrer">
                        "Leptos"
                    </a>
                </p>
            </div>
        </footer>
    }
}