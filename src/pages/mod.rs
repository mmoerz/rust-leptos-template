// Re-export your page components so they are accessible via `pages::*`

pub mod home;
pub mod about;
//pub mod dashboard;
//pub mod contact;

// Optional: re-export for easier imports
pub use home::Home;
pub use about::About;
// pub use dashboard::Dashboard;
// pub use contact::Contact;