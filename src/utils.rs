use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! impl_getter_setter {
    ($ty:ty, $field:ident: $field_ty:ty, $getter:ident) => {
        #[wasm_bindgen]
        impl $ty {
            #[wasm_bindgen(getter)]
            pub fn $getter(&self) -> $field_ty {
                self.$field
            }
        }
    };
    ($ty:ty, $field:ident: $field_ty:ty, $getter:ident, $setter:ident) => {
        #[wasm_bindgen]
        impl $ty {
            #[wasm_bindgen(getter)]
            pub fn $getter(&self) -> $field_ty {
                self.$field
            }

            #[wasm_bindgen(setter)]
            pub fn $setter(&mut self, $field: $field_ty) {
                self.$field = $field
            }
        }
    };
}
