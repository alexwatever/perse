cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        /// # Perse

        /// # Hyration
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn hydrate() {
            use leptos::*;
            use perse_controller::*;

            // Setup Debugging
            console_error_panic_hook::set_once();

            // Hydrate Controller
            leptos::mount_to_body(move || {
                view! { <Controller/> }
            });
        }
    }
}
