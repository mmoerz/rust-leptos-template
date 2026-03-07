use crate::components::Footer;
use crate::components::Navbar;

view! {
    <>
        <Navbar/>
        <main>
            {children()}
        </main>
        <Footer/>
    </>
}