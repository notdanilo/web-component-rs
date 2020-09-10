#[macro_export]
macro_rules! web_component {
    ($name:ident) => {
        paste::item! {
            #[wasm_bindgen::prelude::wasm_bindgen]
            pub fn [<components_web_ $name:lower _create>](attributes: NamedNodeMap) -> usize {
                let object = $name::create_component(attributes);
                unsafe {
                    $crate::objects_register::OBJECTS_REGISTER.register_object(Box::new(object))
                }
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub fn [<components_web_ $name:lower _update_field>](object: usize, name: String, value: String) {
                unsafe {
                    let object = $crate::objects_register::OBJECTS_REGISTER.object(object);
                    object.update_field(&name, &value);
                    object.field_updated(&name);
                }
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub fn [<components_web_ $name:lower _get_data>](object: usize) -> String {
                unsafe {
                    let object = $crate::objects_register::OBJECTS_REGISTER.object(object);
                    object.get_data()
                }
            }

            #[wasm_bindgen::prelude::wasm_bindgen]
            pub fn [<components_web_ $name:lower _on_loaded>](object:usize, shadow_root:ShadowRoot) {
                unsafe {
                    let object = $crate::objects_register::OBJECTS_REGISTER.object(object);
                    object.on_loaded(shadow_root);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! template {
    ($name:ident) => {
        paste::item! {
            #[wasm_bindgen::prelude::wasm_bindgen]
            pub fn [<components_web_ $name:lower _template>]() -> String {
                include_str!("template.html").to_string()
            }
        }
    }
}

#[macro_export]
macro_rules! package {
    () => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn web_component_target_wasm() {}
    }
}

#[macro_export]
macro_rules! update_field {
    ($self:ident, $name:ident, $value:ident $(,$property:ident)*) => {
        $(
            if $name == stringify!($property) {
                $self.$property = serde_json::from_str(&$value).unwrap();
            }
        )*
    }
}