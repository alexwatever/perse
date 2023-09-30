pub mod modules;

cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        ///////////////////////
        /// # Leptos Hyration
        ///////////////////////
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            use modules::app::*;
            use leptos::*;

            console_error_panic_hook::set_once();

            leptos::mount_to_body(move || {
                view! { <App/> }
            });
        }
    }
}
