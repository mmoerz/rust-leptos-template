//! Reusable UI components used throughout the application.

pub mod navbar;
pub mod footer;
pub mod button;
pub mod modal;
pub mod counter;

// Nested component groups
pub mod form;

// Re-export commonly used components
pub use navbar::Navbar;
pub use footer::Footer;
pub use button::Button;
pub use modal::Modal;
pub use counter::Counter;