use leptos::*;
use leptos_router::*;

/// Navbar component with links
///
/// # Example
/// ```rust
/// view! {
///     <Navbar/>
/// }
/// ```
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="app-navbar">
            <div class="container">
                <div class="logo">
                    <a href="/">"MyLeptosApp"</a>
                </div>
                <ul class="nav-links">
                    <li><A href="/">"Home"</A></li>
                    <li><A href="/about">"About"</A></li>
                    <li><A href="/dashboard">"Dashboard"</A></li>
                    <li><A href="/contact">"Contact"</A></li>
                </ul>
            </div>
        </nav>
    }
}