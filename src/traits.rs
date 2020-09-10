use web_sys::NamedNodeMap;
use web_sys::ShadowRoot;

pub trait WebComponentBinding: erased_serde::Serialize {
    fn update_field(&mut self, name: &str, value: &str);

    fn get_data(&self) -> String {
        let mut vec = Vec::with_capacity(128);
        let json = &mut serde_json::Serializer::new(&mut vec);
        let json = &mut erased_serde::Serializer::erase(json);
        self.erased_serialize(json).expect("Couldn't serialize WebComponent.");
        unsafe {
            // We do not emit invalid UTF-8.
            String::from_utf8_unchecked(vec)
        }
    }
}

// Move
pub trait WebComponent: WebComponentBinding {
    fn create_component(attributes:NamedNodeMap) -> Self where Self: Sized;

    fn on_loaded(&mut self,_shadow_root:ShadowRoot) {}

    fn field_updated(&self, _name: &String) {}
}
