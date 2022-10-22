use super::{Base, Event};

impl Base {
    pub fn new(body: &str) -> Self {
        Base {
            body: body.to_string(),
        }
    }
    pub fn type_of<T>(_: T) -> String {
        let a = std::any::type_name::<T>();
        a.to_string()
    }
}
impl Event for Base {
    fn body(&self) -> String {
        self.body.clone()
    }
    fn name(&self) -> &'static str {
        "event"
    }
}
